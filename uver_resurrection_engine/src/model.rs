use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModuleState {
    Active,
    Inactive,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleRecord {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub state: ModuleState,
    pub last_updated: DateTime<Utc>,
    pub energy_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentRecord {
    pub id: Uuid,
    pub path: String,
    pub status: String,
    pub details: String,
    pub fixed: bool,
    pub timestamp: DateTime<Utc>,
}
