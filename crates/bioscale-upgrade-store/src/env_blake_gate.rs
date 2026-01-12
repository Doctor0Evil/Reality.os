use std::time::SystemTime;
use bioscale_upgrade_store::{BioscaleUpgradeStore, HostBudget, UpgradeDecision, UpgradeDescriptor};
use reality_os_cargo_env_descriptor::{describe_cargo_env, CargoEnvDescriptor};

/// Evaluate an upgrade with both HostBudget and Blake3 posture enforced.
pub fn evaluate_with_env_and_blake<S: BioscaleUpgradeStore>(
    store: &S,
    host: HostBudget,
    upgrade: UpgradeDescriptor,
    requested_start: SystemTime,
) -> (UpgradeDecision, CargoEnvDescriptor) {
    let env = describe_cargo_env().expect("CargoEnvDescriptor required");
    let evidence_tags: Vec<String> = upgrade.evidence.tags.iter().cloned().collect();

    // Blake gate: deny if this descriptor intersects BlakePatternSet without proper evidence.
    if !env.permits_blake3(&evidence_tags) && upgrade.crypto_profile.uses_blake3 {
        return (
            UpgradeDecision::Denied {
                reason: "Blake3 use is not permitted by CargoEnvDescriptor posture".into(),
            },
            env,
        );
    }

    (store.evaluate_upgrade(host, upgrade, requested_start), env)
}
