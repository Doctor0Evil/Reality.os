use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealityOsMeta {
    pub specid: String,
    pub version: String,
    pub subjectid: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealityOsService {
    pub name: String,
    pub mode: String,
    pub tsafekernel: String,
    pub donutloop: String,
    pub rohmodel: String,
    pub neurorights: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealityEndpoint {
    pub id: String,
    pub kind: String,
    pub readonly: bool,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealityOsSpec {
    pub meta: RealityOsMeta,
    pub service: RealityOsService,
    pub endpoints: Vec<RealityEndpoint>,
}

impl RealityOsSpec {
    pub fn is_advisory_only(&self) -> bool {
        self.service.mode == "advisory_only"
    }

    pub fn validate_invariants(&self, roh_ceiling: f32) -> Result<(), String> {
        if !self.is_advisory_only() {
            return Err("RealityOsSpec invariant violated: service.mode must be advisory_only".into());
        }
        if (roh_ceiling - 0.30_f32).abs() > 1e-6 {
            return Err("RealityOsSpec invariant violated: roh_ceiling != 0.30".into());
        }
        Ok(())
    }
}
