use chrono::{DateTime, Utc};
use reality_core::events::LedgerEvent;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;
use hex::encode as hex_encode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleNode {
    pub hash: String,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLedgerAnchor {
    pub day: String,
    pub root_hash: String,
    pub external_anchor_tx: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Default)]
pub struct TransparencyLedger {
    pub events: Vec<LedgerEvent>,
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex_encode(h.finalize())
}

fn hash_event(event: &LedgerEvent) -> String {
    let json = serde_json::to_vec(event).expect("serialize event");
    hash_bytes(&json)
}

impl TransparencyLedger {
    pub fn append(&mut self, event: LedgerEvent) {
        self.events.push(event);
    }

    pub fn build_merkle_root(&self) -> Option<MerkleNode> {
        if self.events.is_empty() {
            return None;
        }

        let mut leaves: Vec<MerkleNode> = self
            .events
            .iter()
            .map(|e| MerkleNode {
                hash: hash_event(e),
                left: None,
                right: None,
            })
            .collect();

        while leaves.len() > 1 {
            let mut next = Vec::new();
            for chunk in leaves.chunks(2) {
                if chunk.len() == 1 {
                    next.push(chunk[0].clone());
                } else {
                    let combined =
                        hash_bytes(format!("{}{}", chunk[0].hash, chunk[1].hash).as_bytes());
                    next.push(MerkleNode {
                        hash: combined,
                        left: Some(Box::new(chunk[0].clone())),
                        right: Some(Box::new(chunk[1].clone())),
                    });
                }
            }
            leaves = next;
        }

        leaves.into_iter().next()
    }

    pub fn daily_anchor(&self, day: &str, tx_ref: Option<String>) -> Option<DailyLedgerAnchor> {
        self.build_merkle_root().map(|root| DailyLedgerAnchor {
            day: day.to_string(),
            root_hash: root.hash,
            external_anchor_tx: tx_ref,
            created_at: Utc::now(),
        })
    }
}
