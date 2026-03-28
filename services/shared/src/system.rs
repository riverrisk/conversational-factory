use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    pub total_leases: u64,
    pub dns_records_synced: u64,
    pub pending_updates: u64,
    pub failed_updates: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reconciliation: Option<DateTime<Utc>>,
}
