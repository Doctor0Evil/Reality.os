use serde::{Deserialize, Serialize};
use crate::did::DidUri;
use chrono::{DateTime, Utc};

/// ROW (Recognition of Work) entry: append-only, DID-anchored
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RowEntry {
    /// Unique ROW entry ID (UUID v7)
    pub row_id: String,
    /// Citizen DID who performed the work
    pub citizen_did: DidUri,
    /// Type of work (mission, annotation, code_contribution, care, learning)
    pub work_type: WorkType,
    /// Description of work performed
    pub work_description: String,
    /// Eco-impact delta (positive = improvement)
    pub eco_impact_delta: f32,
    /// Joules consumed for this work
    pub joules_consumed: u64,
    /// Timestamp of work completion
    pub work_timestamp: DateTime<Utc>,
    /// Block height when anchored
    pub block_height: u64,
    /// Transaction hash
    pub transaction_hash: String,
    /// Hex-stamped proof (Merkle root)
    pub hex_stamp_proof: String,
    /// Forward-only: always true (ROW is immutable)
    pub is_immutable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WorkType {
    Mission,
    Annotation,
    CodeContribution,
    Care,
    Learning,
    Validation,
    EcoRestoration,
}

/// RPM (Reward/Participate/Motivate) entry: non-monetary governance weight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpmEntry {
    /// Unique RPM entry ID
    pub rpm_id: String,
    /// Citizen DID
    pub citizen_did: DidUri,
    /// ROW entry IDs that this RPM is based on
    pub row_entry_ids: Vec<String>,
    /// Governance weight earned (0-1 scale)
    pub governance_weight: f32,
    /// Safety score contribution (0-1)
    pub safety_score_contribution: f32,
    /// Learning score contribution (0-1)
    pub learning_score_contribution: f32,
    /// Care score contribution (0-1)
    pub care_score_contribution: f32,
    /// CAC index update
    pub cac_index_update: f32,
    /// Timestamp
    pub rpm_timestamp: DateTime<Utc>,
    /// Block height when anchored
    pub block_height: u64,
    /// Forward-only: always true
    pub is_immutable: bool,
}

impl RpmEntry {
    /// Calculate governance weight from ROW entries
    pub fn calculate_governance_weight(row_entries: &[RowEntry]) -> f32 {
        if row_entries.is_empty() {
            return 0.0;
        }

        let total_eco_impact: f32 = row_entries.iter().map(|r| r.eco_impact_delta.max(0.0)).sum();
        let total_joules: u64 = row_entries.iter().map(|r| r.joules_consumed).sum();

        // Formula: eco impact per joule, normalized to 0-1
        if total_joules == 0 {
            return 0.0;
        }

        let eco_efficiency = total_eco_impact / (total_joules as f32);
        (eco_efficiency * 100.0).clamp(0.0, 1.0)
    }
}

/// ROW/RPM Ledger Shard (append-only)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RowRpmLedgerShard {
    pub row_entries: Vec<RowEntry>,
    pub rpm_entries: Vec<RpmEntry>,
    pub total_row_entries: u64,
    pub total_rpm_entries: u64,
    pub last_updated_height: u64,
}

impl RowRpmLedgerShard {
    /// Append a new ROW entry (forward-only)
    pub fn append_row_entry(&mut self, entry: RowEntry, height: u64) {
        self.row_entries.push(entry);
        self.total_row_entries = self.row_entries.len() as u64;
        self.last_updated_height = height;
    }

    /// Append a new RPM entry (forward-only)
    pub fn append_rpm_entry(&mut self, entry: RpmEntry, height: u64) {
        self.rpm_entries.push(entry);
        self.total_rpm_entries = self.rpm_entries.len() as u64;
        self.last_updated_height = height;
    }

    /// Get all ROW entries for a citizen
    pub fn get_row_entries_for_citizen(&self, citizen_did: &DidUri) -> Vec<&RowEntry> {
        self.row_entries
            .iter()
            .filter(|e| e.citizen_did == *citizen_did)
            .collect()
    }

    /// Get all RPM entries for a citizen
    pub fn get_rpm_entries_for_citizen(&self, citizen_did: &DidUri) -> Vec<&RpmEntry> {
        self.rpm_entries
            .iter()
            .filter(|e| e.citizen_did == *citizen_did)
            .collect()
    }

    /// Calculate total governance weight for a citizen
    pub fn calculate_total_governance_weight(&self, citizen_did: &DidUri) -> f32 {
        let rpm_entries = self.get_rpm_entries_for_citizen(citizen_did);
        rpm_entries.iter().map(|r| r.governance_weight).sum()
    }

    /// Calculate CAC index for a citizen from RPM entries
    pub fn calculate_cac_index(&self, citizen_did: &DidUri) -> f32 {
        let rpm_entries = self.get_rpm_entries_for_citizen(citizen_did);
        if rpm_entries.is_empty() {
            return 0.0;
        }
        let latest = rpm_entries.last().unwrap();
        latest.cac_index_update
    }
}
