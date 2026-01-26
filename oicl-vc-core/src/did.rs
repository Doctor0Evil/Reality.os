#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OiclDid(String);

impl OiclDid {
    pub fn new(did: impl Into<String>) -> Self {
        let s = did.into();
        // Minimal validation: must start with "did:"
        assert!(s.starts_with("did:"), "invalid DID format");
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
