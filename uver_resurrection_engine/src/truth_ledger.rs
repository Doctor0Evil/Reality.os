use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sled::Db;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthEvent {
    pub id: Uuid,
    pub label: String,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthBlock {
    pub id: Uuid,
    pub prev_hash: String,
    pub hash: String,
    pub event: TruthEvent,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct SystemicTruthLedger {
    db: Db,
    head_hash: String,
}

impl SystemicTruthLedger {
    pub fn open(path: &str) -> Self {
        let db = sled::open(Path::new(path)).expect("truth ledger open");
        Self {
            db,
            head_hash: "GENESIS".into(),
        }
    }

    pub fn log_event(&mut self, label: &str, payload: serde_json::Value) {
        let event = TruthEvent {
            id: Uuid::new_v4(),
            label: label.into(),
            payload,
            timestamp: Utc::now(),
        };
        let created_at = Utc::now();
        let mut hasher = Sha256::new();
        hasher.update(self.head_hash.as_bytes());
        hasher.update(serde_json::to_vec(&event).unwrap());
        hasher.update(created_at.timestamp_nanos().to_be_bytes());
        let hash = hex::encode(hasher.finalize());
        let block = TruthBlock {
            id: Uuid::new_v4(),
            prev_hash: self.head_hash.clone(),
            hash: hash.clone(),
            event,
            created_at,
        };
        let key = hash.as_bytes().to_vec();
        let val = serde_json::to_vec(&block).unwrap();
        let _ = self.db.insert(key, val);
        self.head_hash = hash;
    }
}
