use crate::cargoenv::CargoEnvDescriptor;

/// Crypto posture for this host: BLAKE-family and SHA3-family forbidden.
#[derive(Clone, Debug)]
pub struct CryptoPosture {
    /// If false, *any* BLAKE-family compile/run/OTA is forbidden.
    pub blake_allowed: bool,
    /// If false, *any* SHA3-family compile/run/OTA is forbidden.
    pub sha3_allowed: bool,
    /// Versioned denylist pattern set identifier.
    pub patternset_version: &'static str,
}

#[derive(Clone, Debug)]
pub struct CryptoAwareEnv {
    pub env: CargoEnvDescriptor,
    pub crypto: CryptoPosture,
}

impl CryptoAwareEnv {
    pub fn current() -> Self {
        let env = CargoEnvDescriptor::new_default_bioscale(); // existing constructor.[file:40][file:31]

        let crypto = CryptoPosture {
            blake_allowed: false,
            sha3_allowed: false,
            patternset_version: "crypto-patterns-v1",
        };

        Self { env, crypto }
    }

    /// Hard policy: no BLAKE-family allowed.
    pub fn permits_blake(&self) -> bool {
        self.crypto.blake_allowed
    }

    /// Hard policy: no SHA3-family allowed.
    pub fn permits_sha3(&self) -> bool {
        self.crypto.sha3_allowed
    }
}
