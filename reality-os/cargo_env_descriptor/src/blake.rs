use serde::{Deserialize, Serialize};

/// Blake3 / Blake-family posture for this host.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlakeCryptoPosture {
    /// If false, any Blake3-related build or runtime use is forbidden.
    pub blake3_allowed: bool,
    /// Optional whitelist gate; only descriptors carrying this tag may use Blake3.
    pub allowed_evidence_tag: Option<String>,
    /// Versioned static pattern set baked into this host.
    pub blake_pattern_set_version: String,
}
