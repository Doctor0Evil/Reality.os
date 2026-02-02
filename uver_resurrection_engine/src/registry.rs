use crate::model::{ModuleRecord, ModuleState};
use chrono::Utc;
use sled::Db;
use std::path::Path;
use uuid::Uuid;

#[derive(Clone)]
pub struct ModuleRegistry {
    db: Db,
}

impl ModuleRegistry {
    pub fn open(path: &str) -> Self {
        let db = sled::open(Path::new(path)).expect("module registry open");
        Self { db }
    }

    pub fn bootstrap_if_empty(&mut self) {
        if self.db.len() > 0 {
            return;
        }
        let modules = vec![
            ("DataLakeValidator", "1.0.0", ModuleState::Inactive),
            ("ModuleResurrectionEngine", "1.0.0", ModuleState::Inactive),
            ("EnergyBallAPI", "1.0.0", ModuleState::Inactive),
            ("ZeroTrustMonitor", "1.0.0", ModuleState::Inactive),
        ];
        for (name, version, state) in modules {
            let rec = ModuleRecord {
                id: Uuid::new_v4(),
                name: name.into(),
                version: version.into(),
                state,
                last_updated: Utc::now(),
                energy_policy: "infinite".into(),
            };
            let key = rec.name.as_bytes().to_vec();
            let val = serde_json::to_vec(&rec).unwrap();
            let _ = self.db.insert(key, val);
        }
    }

    pub fn all(&self) -> Vec<ModuleRecord> {
        let mut out = Vec::new();
        for item in self.db.iter() {
            if let Ok((_, v)) = item {
                if let Ok(rec) = serde_json::from_slice::<ModuleRecord>(&v) {
                    out.push(rec);
                }
            }
        }
        out
    }

    pub fn save(&self, rec: &ModuleRecord) {
        let key = rec.name.as_bytes().to_vec();
        let val = serde_json::to_vec(rec).unwrap();
        let _ = self.db.insert(key, val);
    }
}
