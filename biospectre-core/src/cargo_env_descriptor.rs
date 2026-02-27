use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct CryptoPostureCorridor {
    pub corridor_id: String,
    pub blake3_allowed: bool,
    pub argon2_allowed: bool,
    pub allowed_hash_families: HashSet<String>,
    pub apply_to_targets: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct CargoEnvDescriptor {
    pub target_pkg_name: String,
    pub posture: CryptoPostureCorridor,
}

impl CargoEnvDescriptor {
    pub fn is_in_scope(&self) -> bool {
        self.posture
            .apply_to_targets
            .contains(&self.target_pkg_name)
    }

    pub fn blake3_allowed(&self) -> bool {
        self.posture.blake3_allowed
    }

    pub fn argon2_allowed(&self) -> bool {
        self.posture.argon2_allowed
    }

    pub fn family_allowed(&self, family: &str) -> bool {
        self.posture.allowed_hash_families.contains(family)
    }
}
