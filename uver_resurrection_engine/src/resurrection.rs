use crate::model::{ModuleRecord, ModuleState};
use crate::registry::ModuleRegistry;
use crate::truth_ledger::SystemicTruthLedger;
use chrono::Utc;

#[derive(Clone)]
pub struct ModuleResurrectionEngine {
    registry: ModuleRegistry,
    truth: SystemicTruthLedger,
}

impl ModuleResurrectionEngine {
    pub fn new(registry: ModuleRegistry, truth: SystemicTruthLedger) -> Self {
        Self { registry, truth }
    }

    pub fn revive_all_ignore_dependencies(&mut self, energy_source: &str, force: bool) {
        let modules = self.registry.all();
        for mut m in modules {
            if m.state == ModuleState::Inactive || m.state == ModuleState::Deprecated || force {
                m.state = ModuleState::Active;
                m.energy_policy = format!("source={};mode=infinite", energy_source);
                m.last_updated = Utc::now();
                self.registry.save(&m);
                self.truth.log_event(
                    "MODULE_REVIVED",
                    serde_json::json!({
                        "module": m.name,
                        "version": m.version,
                        "energy_policy": m.energy_policy,
                        "force": force,
                        "timestamp": Utc::now(),
                    }),
                );
            }
        }
    }
}
