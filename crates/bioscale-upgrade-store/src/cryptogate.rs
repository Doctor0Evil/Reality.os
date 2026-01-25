use std::time::SystemTime;

use crate::{BioscaleUpgradeStore, HostBudget, UpgradeDecision, UpgradeDescriptor};
use reality_os::crypto_env::CryptoAwareEnv;
use reality_os::crypto_patterns::CryptoPatternSet;

/// Helper trait to detect crypto usage from descriptors.
pub trait CryptoProfileExt {
    fn uses_blake3(&self) -> bool;
    fn uses_sha3(&self) -> bool;
}

impl CryptoProfileExt for UpgradeDescriptor {
    fn uses_blake3(&self) -> bool {
        let p = CryptoPatternSet::v1();
        let mut found = false;
        let meta = [
            self.github_repo_slug.as_str(),
            self.ota_manifest_path.as_str(),
            self.display_name.as_str(),
            self.version.as_str(),
        ];
        for m in meta {
            if p.is_blake(m) {
                found = true;
                break;
            }
        }
        found
    }

    fn uses_sha3(&self) -> bool {
        let p = CryptoPatternSet::v1();
        let mut found = false;
        let meta = [
            self.github_repo_slug.as_str(),
            self.ota_manifest_path.as_str(),
            self.display_name.as_str(),
            self.version.as_str(),
        ];
        for m in meta {
            if p.is_sha3(m) {
                found = true;
                break;
            }
        }
        found
    }
}

/// Evaluate an upgrade under HostBudget + crypto posture.
/// Any BLAKE or SHA3 usage is rejected unconditionally.
pub fn evaluate_with_crypto_gate<S: BioscaleUpgradeStore>(
    store: S,
    host: HostBudget,
    upgrade: UpgradeDescriptor,
    start: SystemTime,
    env: &CryptoAwareEnv,
) -> UpgradeDecision {
    if upgrade.uses_blake3() && !env.permits_blake() {
        return UpgradeDecision::Denied {
            reason: "BLAKE-family cryptography not permitted in this corridor".into(),
        };
    }

    if upgrade.uses_sha3() && !env.permits_sha3() {
        return UpgradeDecision::Denied {
            reason: "SHA3-family cryptography not permitted in this corridor".into(),
        };
    }

    store.evaluate_upgrade(host, upgrade, start)
}
