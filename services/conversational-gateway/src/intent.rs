use std::sync::Arc;

use cf_shared::assets::SemanticRecord;
use cf_shared::gateway::*;
use cf_shared::query::RecordFilter;

use crate::summary;
use crate::GatewayState;

pub async fn handle_request(state: &Arc<GatewayState>, req: GatewayRequest) -> GatewayResponse {
    match req.intent {
        GatewayIntent::ResolveAsset => handle_resolve(state, &req).await,
        GatewayIntent::QueryAssets => handle_query(state, &req).await,
        GatewayIntent::DescribeTopology => handle_describe_topology(state, &req).await,
        GatewayIntent::SummarizeAssets => handle_summarize(state, &req).await,
    }
}

async fn handle_resolve(state: &Arc<GatewayState>, req: &GatewayRequest) -> GatewayResponse {
    let target = match &req.target {
        Some(t) => &t.target,
        None => {
            return GatewayResponse {
                request_id: req.request_id,
                status: GatewayStatus::InvalidRequest,
                summary: None,
                records: None,
                resolved_target: None,
                applied_filter: None,
                citations: None,
                errors: Some(vec!["resolve-asset intent requires a target".into()]),
            };
        }
    };

    match resolve_upstream(state, target).await {
        Ok(Some(record)) => {
            let citations = extract_citations(&record);
            let summ = summary::summarize_record(&record);
            GatewayResponse {
                request_id: req.request_id,
                status: GatewayStatus::Ok,
                summary: Some(summ),
                records: Some(vec![record.clone()]),
                resolved_target: Some(record.fqdn.clone()),
                applied_filter: None,
                citations: Some(citations),
                errors: None,
            }
        }
        Ok(None) => GatewayResponse {
            request_id: req.request_id,
            status: GatewayStatus::NotFound,
            summary: Some(format!("No asset found matching \"{}\".", target)),
            records: None,
            resolved_target: None,
            applied_filter: None,
            citations: None,
            errors: None,
        },
        Err(e) => error_response(req.request_id, e),
    }
}

async fn handle_query(state: &Arc<GatewayState>, req: &GatewayRequest) -> GatewayResponse {
    let filter = req.filter.clone().unwrap_or_default();
    match query_upstream(state, &filter).await {
        Ok(records) => {
            let citations: Vec<Citation> = records.iter().flat_map(|r| extract_citations(r)).collect();
            let summ = summary::summarize_query_results(&records, &filter);
            GatewayResponse {
                request_id: req.request_id,
                status: if records.is_empty() { GatewayStatus::NotFound } else { GatewayStatus::Ok },
                summary: Some(summ),
                records: Some(records),
                resolved_target: None,
                applied_filter: Some(filter),
                citations: Some(citations),
                errors: None,
            }
        }
        Err(e) => error_response(req.request_id, e),
    }
}

async fn handle_describe_topology(state: &Arc<GatewayState>, req: &GatewayRequest) -> GatewayResponse {
    // Query all assets, then summarize topology
    let filter = req.filter.clone().unwrap_or_default();
    match query_upstream(state, &filter).await {
        Ok(records) => {
            let summ = summary::describe_topology(&records);
            GatewayResponse {
                request_id: req.request_id,
                status: GatewayStatus::Ok,
                summary: Some(summ),
                records: Some(records),
                resolved_target: None,
                applied_filter: Some(filter),
                citations: None,
                errors: None,
            }
        }
        Err(e) => error_response(req.request_id, e),
    }
}

async fn handle_summarize(state: &Arc<GatewayState>, req: &GatewayRequest) -> GatewayResponse {
    let filter = req.filter.clone().unwrap_or_default();
    match query_upstream(state, &filter).await {
        Ok(records) => {
            let summ = summary::summarize_fleet(&records, &filter);
            GatewayResponse {
                request_id: req.request_id,
                status: if records.is_empty() { GatewayStatus::NotFound } else { GatewayStatus::Ok },
                summary: Some(summ),
                records: match req.response_mode {
                    ResponseMode::SummaryOnly => None,
                    _ => Some(records),
                },
                resolved_target: None,
                applied_filter: Some(filter),
                citations: None,
                errors: None,
            }
        }
        Err(e) => error_response(req.request_id, e),
    }
}

fn error_response(request_id: uuid::Uuid, msg: String) -> GatewayResponse {
    GatewayResponse {
        request_id,
        status: GatewayStatus::Error,
        summary: None,
        records: None,
        resolved_target: None,
        applied_filter: None,
        citations: None,
        errors: Some(vec![msg]),
    }
}

fn extract_citations(record: &SemanticRecord) -> Vec<Citation> {
    let mut cites = vec![Citation {
        kind: CitationKind::Fqdn,
        value: record.fqdn.clone(),
    }];
    for hw in &record.hardware_identities {
        cites.push(Citation {
            kind: CitationKind::HardwareIdentity,
            value: hw.value.clone(),
        });
    }
    for app in &record.application_identities {
        cites.push(Citation {
            kind: CitationKind::ApplicationIdentity,
            value: app.value.clone(),
        });
    }
    for alias in &record.aliases {
        cites.push(Citation {
            kind: CitationKind::Alias,
            value: alias.clone(),
        });
    }
    cites
}

async fn resolve_upstream(state: &Arc<GatewayState>, target: &str) -> Result<Option<SemanticRecord>, String> {
    let url = format!("{}/assets/resolve", state.query_plane_url);
    let resp = state
        .client
        .get(&url)
        .query(&[("target", target)])
        .send()
        .await
        .map_err(|e| format!("query-plane unreachable: {}", e))?;

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Err(format!("query-plane returned {}", resp.status()));
    }
    let record: SemanticRecord = resp.json().await.map_err(|e| format!("bad response: {}", e))?;
    Ok(Some(record))
}

async fn query_upstream(state: &Arc<GatewayState>, filter: &RecordFilter) -> Result<Vec<SemanticRecord>, String> {
    let url = format!("{}/assets/query", state.query_plane_url);

    // Build query params from non-None filter fields
    let params = serde_json::to_value(filter)
        .ok()
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(k, v)| match v {
            serde_json::Value::String(s) => Some((k, s)),
            _ => None,
        })
        .collect::<Vec<_>>();

    let resp = state
        .client
        .get(&url)
        .query(&params)
        .send()
        .await
        .map_err(|e| format!("query-plane unreachable: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("query-plane returned {}", resp.status()));
    }
    let records: Vec<SemanticRecord> = resp.json().await.map_err(|e| format!("bad response: {}", e))?;
    Ok(records)
}
