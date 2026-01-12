use cargo_env_descriptor::{CargoEnvDescriptor, CargoEnvValidator, PhoenixNeurostackEnvValidator};
use bioscale_upgrade_store::HostBudget;

pub fn admit_env_for_host(
    desc: &CargoEnvDescriptor,
    host_budget: &HostBudget,
) -> Result<(), String> {
    PhoenixNeurostackEnvValidator
        .validate_env(desc)
        .map_err(|e| format!("Env validation failed: {e:?}"))?;

    if desc.neurorights.host_budget_profile.dailyenergyjoules
        > host_budget.dailyenergyjoules
    {
        return Err("Descriptor host budget exceeds actual host envelope".into());
    }

    Ok(())
}
