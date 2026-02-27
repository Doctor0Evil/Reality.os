#[derive(Debug, Clone)]
pub enum CryptoResidualClass {
    BlakeLike,
    Argon2Like,
    UnknownSafe,
}

#[derive(Debug, Clone)]
pub struct CryptoResidualTag {
    pub class: CryptoResidualClass,
    pub source: String,     // module or shard id
    pub proof_hex: String,  // ties into HexProofRegistry
}

pub fn runtime_veto(tag: &CryptoResidualTag, env: &CargoEnvDescriptor) -> bool {
    if !env.is_in_scope() {
        return false;
    }
    match tag.class {
        CryptoResidualClass::BlakeLike if !env.blake3_allowed() => true,
        CryptoResidualClass::Argon2Like if !env.argon2_allowed() => true,
        _ => false,
    }
}
