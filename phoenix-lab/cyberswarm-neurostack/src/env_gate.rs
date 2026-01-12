use reality_os_cargo_env_descriptor::{describe_cargo_env, CargoEnvDescriptor};
use crate::router_core::CyberSwarmNeurostackRouter;
use bioscale_upgrade_store::{BioscaleUpgradeStore, UpgradeDecision};

pub struct EnvGatedRouter<R: BioscaleUpgradeStore> {
    pub env: CargoEnvDescriptor,
    pub store: R,
    pub router: CyberSwarmNeurostackRouter,
}

impl<R: BioscaleUpgradeStore> EnvGatedRouter<R> {
    pub fn new(store: R, router: CyberSwarmNeurostackRouter) -> Self {
        let env = describe_cargo_env().expect("CargoEnvDescriptor must be available");
        Self { env, store, router }
    }

    /// Refuse any evolution if the environment is not safety-qualified.
    pub fn evaluate_and_route(&self, ctx: crate::router_core::PolicyContext, snapshot: crate::router_core::GatewayStateSnapshot)
        -> crate::router_core::RoutingDecision
    {
        if !self.env.is_bci_safety_qualified() {
            return crate::router_core::RoutingDecision {
                userid: snapshot.userid,
                selectedtargets: vec![],
                deniedreason: Some("Environment not BCI-safety-qualified by CargoEnvDescriptor".into()),
            };
        }

        // Then rely on existing bioscale evaluation + routing.[file:11][file:12]
        match crate::bioscale_integration::evaluate_intent_with_store(&self.store, ctx.intent, snapshot.timestamp) {
            UpgradeDecision::Denied { reason } => crate::router_core::RoutingDecision {
                userid: snapshot.userid,
                selectedtargets: vec![],
                deniedreason: Some(format!("Bioscale denied: {}", reason)),
            },
            UpgradeDecision::Approved { .. } => self.router.route(snapshot, ctx),
        }
    }
}
