use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwarmMeta {
    pub policyid: String,
    pub version: String,
    pub subjectid: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwarmConfig {
    pub controller: String,
    pub tsafekernel: String,
    pub maxagents: u32,
    pub maxdepth: u32,
    pub mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwarmRole {
    pub name: String,
    pub may_read: Vec<String>,
    pub may_write: Vec<String>,
    pub forbidden: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RealitySwarmPolicy {
    pub meta: SwarmMeta,
    pub swarm: SwarmConfig,
    pub roles: Vec<SwarmRole>,
}

impl RealitySwarmPolicy {
    pub fn is_gated_only(&self) -> bool {
        self.swarm.mode == "gated_only"
    }

    pub fn has_no_actuation_role(&self) -> bool {
        self.roles.iter().all(|r| !r.forbidden.is_empty())
    }
}
