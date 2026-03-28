use serde::{Deserialize, Serialize};

use crate::identity::{Isa95NodeKind, Isa95WorkCenterKind};

/// Composable filter for querying semantic records.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<Isa95NodeKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_identity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_center: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_center_kind: Option<Isa95WorkCenterKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

/// Resolve a single asset by FQDN, IP, alias, or identity string.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveTarget {
    pub target: String,
}
