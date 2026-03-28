use std::future::Future;
use std::pin::Pin;

use cf_shared::assets::SemanticRecord;
use cf_shared::query::RecordFilter;
use cf_shared::system::SyncStatus;

use crate::routes::AssetProvider;

pub struct UpstreamProvider {
    base_url: String,
    client: reqwest::Client,
    token: Option<String>,
}

impl UpstreamProvider {
    pub fn new(base_url: String) -> Self {
        let token = std::env::var("SEMANTIC_DNS_TOKEN").ok();
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
            token,
        }
    }
}

impl AssetProvider for UpstreamProvider {
    fn resolve(
        &self,
        target: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<SemanticRecord>, String>> + Send + '_>> {
        let target = target.to_string();
        Box::pin(async move {
            let url = format!("{}/api/v1/resolve/{}", self.base_url, target);
            let mut req = self.client.get(&url);
            if let Some(token) = &self.token {
                req = req.bearer_auth(token);
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            if resp.status() == reqwest::StatusCode::NOT_FOUND {
                return Ok(None);
            }
            if !resp.status().is_success() {
                return Err(format!("upstream returned {}", resp.status()));
            }
            let record: SemanticRecord = resp.json().await.map_err(|e| e.to_string())?;
            Ok(Some(record))
        })
    }

    fn query(
        &self,
        filter: &RecordFilter,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<SemanticRecord>, String>> + Send + '_>> {
        let qs = serde_json::to_value(filter)
            .ok()
            .and_then(|v| v.as_object().cloned())
            .unwrap_or_default()
            .into_iter()
            .filter_map(|(k, v)| v.as_str().map(|s| (k, s.to_string())))
            .collect::<Vec<_>>();

        Box::pin(async move {
            let url = format!("{}/api/v1/dns/query", self.base_url);
            let mut req = self.client.get(&url);
            if let Some(token) = &self.token {
                req = req.bearer_auth(token);
            }
            for (k, v) in &qs {
                req = req.query(&[(k.as_str(), v.as_str())]);
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            if !resp.status().is_success() {
                return Err(format!("upstream returned {}", resp.status()));
            }
            let records: Vec<SemanticRecord> = resp.json().await.map_err(|e| e.to_string())?;
            Ok(records)
        })
    }

    fn sync_status(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<SyncStatus, String>> + Send + '_>> {
        Box::pin(async move {
            let url = format!("{}/api/v1/dhcp/dns/sync-status", self.base_url);
            let mut req = self.client.get(&url);
            if let Some(token) = &self.token {
                req = req.bearer_auth(token);
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            if !resp.status().is_success() {
                return Err(format!("upstream returned {}", resp.status()));
            }
            let status: SyncStatus = resp.json().await.map_err(|e| e.to_string())?;
            Ok(status)
        })
    }
}
