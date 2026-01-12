use std::time::SystemTime;

use crate::{BioscaleUpgradeStore, HostBudget, UpgradeDecision, UpgradeDescriptor};
use reality_os::cargo_env::{describe_cargo_env, CargoEnvDescriptor};

/// Evaluate an upgrade under both HostBudget and CargoEnvDescriptor envelopes.
pub fn evaluate_with_env<S: BioscaleUpgradeStore>(
    store: &S,
    host: HostBudget,
    upgrade: UpgradeDescriptor,
    requested_start: SystemTime,
) -> (UpgradeDecision, CargoEnvDescriptor) {
    let env = describe_cargo_env();
    let decision = store.evaluate_upgrade(host, upgrade, requested_start);
    (decision, env)
}
