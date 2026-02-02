mod model;
mod registry;
mod resurrection;
mod datalake_validator;
mod truth_ledger;

use crate::registry::ModuleRegistry;
use crate::resurrection::ModuleResurrectionEngine;
use crate::datalake_validator::DataLakeValidator;
use crate::truth_ledger::SystemicTruthLedger;
use chrono::Utc;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

static GLOBAL_REGISTRY: Lazy<RwLock<ModuleRegistry>> =
    Lazy::new(|| RwLock::new(ModuleRegistry::open("data/module_registry")));
static GLOBAL_TRUTH: Lazy<RwLock<SystemicTruthLedger>> =
    Lazy::new(|| RwLock::new(SystemicTruthLedger::open("data/systemic_truth")));
static GLOBAL_VALIDATOR: Lazy<RwLock<DataLakeValidator>> =
    Lazy::new(|| RwLock::new(DataLakeValidator::new(
        "data/data_lake",
        "/core_routes",
        "/var/log/data_lake/alignment.log",
    )));

#[tokio::main]
async fn main() {
    {
        let mut truth = GLOBAL_TRUTH.write();
        truth.log_event("BOOT", serde_json::json!({
            "timestamp": Utc::now(),
            "message": "uver_resurrection_engine boot",
        }));
    }

    {
        let mut registry = GLOBAL_REGISTRY.write();
        registry.bootstrap_if_empty();
    }

    {
        let registry = GLOBAL_REGISTRY.read().clone();
        let mut truth = GLOBAL_TRUTH.write();
        let mut engine = ModuleResurrectionEngine::new(registry, truth.clone());
        engine.revive_all_ignore_dependencies("Energy_Ball", true);
    }

    {
        let mut validator = GLOBAL_VALIDATOR.write();
        validator.validate_and_autofix(true);
    }
}
