use reality_os::crypto_env::CryptoAwareEnv;
use reality_os::crypto_patterns::CryptoPatternSet;
use reality_os::cargoenv::CargoEnvDescriptor;

pub struct CryptoEnvGate {
    env: CryptoAwareEnv,
    patterns: CryptoPatternSet,
}

impl CryptoEnvGate {
    pub fn new() -> Self {
        let env = CryptoAwareEnv::current();
        let patterns = CryptoPatternSet::v1();
        Self { env, patterns }
    }

    /// Hard fail if the requested crate or symbol suggests BLAKE or SHA3.
    pub fn allow_crate(&self, crate_name: &str) -> bool {
        if self.patterns.matches_any(crate_name) {
            return false;
        }
        true
    }

    /// AI-chat dev-tunnel check: environment must be BCI-safe and crypto-safe.
    pub fn is_chat_env_safe(&self) -> bool {
        self.env.env.is_bci_safety_qualified()   // existing bioscale + neurorights checks.[file:40]
            && !self.env.crypto.blake_allowed
            && !self.env.crypto.sha3_allowed
    }
}
