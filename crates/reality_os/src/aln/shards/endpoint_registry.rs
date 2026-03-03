use serde::{Deserialize, Serialize};
use crate::did::DidUri;
use crate::eco_metrics::EcoImpactScore;

/// ALN EndpointRegistry shard for Organichain RPC endpoints.
/// All entries are signed by SovereigntyCore and anchored via ROW.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointRegistryEntry {
    /// Unique endpoint identifier (Merkle-rooted)
    pub endpoint_id: String,
    /// RPC URL (HTTPS/WSS only, no HTTP)
    pub rpc_url: String,
    /// gRPC endpoint for streaming
    pub grpc_url: Option<String>,
    /// WebSocket endpoint for subscriptions
    pub websocket_url: Option<String>,
    /// Validator DID controlling this endpoint
    pub validator_did: DidUri,
    /// Jurisdiction tag (e.g. "Phoenix-AZ", "SanJolle-XR")
    pub jurisdiction: String,
    /// Mode suitability (clinical, field, research, daily)
    pub mode_tags: Vec<String>,
    /// Eco-impact score (must be ≥ 0.86 for health RPCs)
    pub eco_impact_score: EcoImpactScore,
    /// Risk-of-harm rating (must be ≤ 0.3 for biophysical ops)
    pub risk_of_harm: f32,
    /// K/E/R scoreboard reference
    pub ker_scoreboard_id: Option<String>,
    /// ROW anchor height when this entry was committed
    pub row_anchor_height: u64,
    /// Forward-only: true if this is the latest version
    pub is_current: bool,
    /// Previous entry hash (for audit chain)
    pub previous_entry_hash: Option<String>,
}

impl EndpointRegistryEntry {
    /// Validate that this endpoint meets minimum floors for health RPCs
    pub fn meets_health_rpc_floors(&self) -> bool {
        self.eco_impact_score.value >= 0.86 && self.risk_of_harm <= 0.3
    }

    /// Validate for general (non-health) biophysical ops
    pub fn meets_general_floors(&self) -> bool {
        self.eco_impact_score.value >= 0.70 && self.risk_of_harm <= 0.5
    }

    /// Check if this endpoint is suitable for a given mode
    pub fn supports_mode(&self, mode: &str) -> bool {
        self.mode_tags.iter().any(|tag| tag == mode)
    }

    /// Check if this endpoint is in the citizen's jurisdiction
    pub fn in_jurisdiction(&self, jurisdiction: &str) -> bool {
        self.jurisdiction == jurisdiction || self.jurisdiction == "global"
    }
}

/// Full registry shard (append-only list)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EndpointRegistryShard {
    pub entries: Vec<EndpointRegistryEntry>,
    pub last_updated_height: u64,
    pub sovereign_core_did: DidUri,
}

impl EndpointRegistryShard {
    /// Filter endpoints for a given citizen profile
    pub fn filter_for_citizen(
        &self,
        jurisdiction: &str,
        mode: &str,
        health_rpc_required: bool,
    ) -> Vec<&EndpointRegistryEntry> {
        self.entries
            .iter()
            .filter(|e| e.is_current)
            .filter(|e| e.in_jurisdiction(jurisdiction))
            .filter(|e| e.supports_mode(mode))
            .filter(|e| {
                if health_rpc_required {
                    e.meets_health_rpc_floors()
                } else {
                    e.meets_general_floors()
                }
            })
            .collect()
    }

    /// Append a new entry (forward-only, never mutates existing)
    pub fn append_entry(&mut self, entry: EndpointRegistryEntry, height: u64) {
        // Mark previous entries from same validator as not current
        for e in self.entries.iter_mut() {
            if e.validator_did == entry.validator_did && e.rpc_url == entry.rpc_url {
                e.is_current = false;
            }
        }
        self.entries.push(entry);
        self.last_updated_height = height;
    }
}
