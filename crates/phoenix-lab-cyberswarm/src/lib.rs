use reality_os::cargo_env::describe_cargo_env;
use bioscale_upgrade_store::{BioscaleUpgradeStore, HostBudget};
use cyberswarm_router::CyberSwarmNeurostackRouter;
use cybernetic_evolution_points::EvolutionPoint; // your trait

evolve!(
    env  = describe_cargo_env(),
    host = host_budget: HostBudget,
    store = store,           // impl BioscaleUpgradeStore
    router = router,         // CyberSwarmNeurostackRouter
    point = MotorAssistV2,   // impl EvolutionPoint
    target = "x86_64-unknown-linux-gnu",
    ota    = {
        org    = "Doctor0Evil",
        repo   = "Cyberswarm",
        branch = "main",
    }
);
