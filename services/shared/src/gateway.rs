use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::assets::SemanticRecord;
use crate::query::{RecordFilter, ResolveTarget};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GatewayIntent {
    ResolveAsset,
    QueryAssets,
    DescribeTopology,
    SummarizeAssets,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResponseMode {
    RecordsOnly,
    SummaryOnly,
    SummaryAndRecords,
}

impl Default for ResponseMode {
    fn default() -> Self {
        Self::SummaryAndRecords
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayRequest {
    pub request_id: Uuid,
    pub intent: GatewayIntent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natural_language_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ResolveTarget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<RecordFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fields: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "is_default_response_mode")]
    pub response_mode: ResponseMode,
}

fn is_default_response_mode(mode: &ResponseMode) -> bool {
    matches!(mode, ResponseMode::SummaryAndRecords)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GatewayStatus {
    Ok,
    NotFound,
    InvalidRequest,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CitationKind {
    Fqdn,
    ApplicationIdentity,
    HardwareIdentity,
    Alias,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citation {
    pub kind: CitationKind,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayResponse {
    pub request_id: Uuid,
    pub status: GatewayStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<SemanticRecord>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_filter: Option<RecordFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citations: Option<Vec<Citation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<String>,
    #[serde(default = "default_read_only")]
    pub read_only: bool,
}

fn default_read_only() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCatalog {
    pub server_name: String,
    pub version: String,
    pub tools: Vec<ToolDefinition>,
}
