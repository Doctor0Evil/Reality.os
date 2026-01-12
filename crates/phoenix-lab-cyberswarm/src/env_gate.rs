use reality_os::cargo_env::describe_cargo_env;
use bioscale_upgrade_store::{BioscaleUpgradeStore, UpgradeDescriptor, UpgradeDecision};
use cyberswarm_router::CyberSwarmNeurostackRouter;

/// Combined gate that must pass before any evolution is attempted.
pub fn env_precheck<S: BioscaleUpgradeStore>(
    store: &S,
    router: &CyberSwarmNeurostackRouter,
    upgrade: &UpgradeDescriptor,
    target_triple: &str,
    ota_repo_org: &str,
    ota_repo_name: &str,
    ota_branch: &str,
) -> Result<(), &'static str> {
    let env = describe_cargo_env();

    if !env.is_target_allowed(target_triple) {
        return Err("target triple not allowed by CargoEnvDescriptor");
    }
    if !env.is_ota_repo_allowed(ota_repo_org, ota_repo_name, ota_branch) {
        return Err("OTA repo not allowed by CargoEnvDescriptor");
    }

    // Optionally, cross-check host-budget envelopes:
    // e.g. refuse upgrades whose corridor score would exceed env.bioscale.max_corridor_score
    // using your existing nanoswarm-host-math and router corridor APIs.

    Ok(())
}
