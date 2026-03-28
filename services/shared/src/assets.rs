use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::identity::*;

/// Minimal reusable handle for referring to a semantic asset across services.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

/// Discovery/registry ingestion payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub id: Uuid,
    pub device_id: Uuid,
    pub observed_at: DateTime<Utc>,
    pub source: ObservationSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_kind: Option<Isa95NodeKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_port: Option<String>,
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
    pub facility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_identities: Option<Vec<HardwareIdentity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_identities: Option<Vec<ApplicationIdentity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<SemanticRelation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RecordStatus>,
}

/// Normalized semantic asset record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticRecord {
    pub device_id: Uuid,
    pub fqdn: String,
    pub node_kind: Isa95NodeKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub protocols: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_port: Option<String>,
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
    pub facility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<String>,
    #[serde(rename = "function", skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    pub hardware_identities: Vec<HardwareIdentity>,
    pub application_identities: Vec<ApplicationIdentity>,
    pub aliases: Vec<String>,
    pub relations: Vec<SemanticRelation>,
    pub status: RecordStatus,
    pub updated_at: DateTime<Utc>,
    pub field_sources: BTreeMap<String, MetadataField>,
}
