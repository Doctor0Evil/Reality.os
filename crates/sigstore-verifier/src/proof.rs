use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnchorTarget {
    RealityOs,
    Googolswarm,
    Organichain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorRequest {
    pub anchor_target: AnchorTarget,
    pub ledger_id: String,      // e.g. ".donutloop.aln stream id"
    pub subject_id: String,     // Bostrom / OrganicCPU subject
    pub binary_path: String,    // local path at verification time
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainProof {
    pub schemaversion: String,
    pub kind: String, // "sigstore-anchor-proof.v1"
    pub proof_id: Uuid,

    pub created_at: DateTime<Utc>,
    pub subject_uri: String,
    pub binary_sha1: Option<String>,
    pub binary_sha256: Option<String>,

    pub bundle_sha256: String,

    pub fulcio_subject: String,
    pub fulcio_issuer: String,
    pub fulcio_not_before: DateTime<Utc>,
    pub fulcio_not_after: DateTime<Utc>,

    pub tsa_timestamp: Option<DateTime<Utc>>,

    pub anchor_target: AnchorTarget,
    pub ledger_id: String,
    pub subject_id: String,
    pub binary_path: String,
}
