#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlakeCryptoPosture {
    pub blake3_allowed: bool,
    pub allowed_evidence_tag: Option<String>,
}

impl BlakeCryptoPosture {
    pub fn forbid_all() -> Self {
        BlakeCryptoPosture {
            blake3_allowed: false,
            allowed_evidence_tag: None,
        }
    }
}
