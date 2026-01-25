use serde::{Serialize, Deserialize};
use serde_json::Value;
use neurorights_firewall::{NeurorightsProfile, HasNeurorightsProfile};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub user_did: String,
    pub aln: String,
    pub bostrom_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub created_at: String,
    pub source_client: String,
    pub model_context: String,
    pub session_id: String,
    pub parent_trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Governance {
    pub eibon_label: String,
    pub policy_scope: String,
    pub jurisdiction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Intent {
    Retrieve,
    Analyze,
    Plan,
    Simulate,
    Governance,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLevel {
    Public,
    Restricted,
    Sensitive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptEnvelope {
    pub trace_id: String,
    pub intent: Intent,
    pub args: Value,
    pub security_level: SecurityLevel,
    pub identity: Identity,
    pub provenance: Provenance,
    pub governance: Governance,
    pub neurorights_profile: NeurorightsProfile,
}

impl HasNeurorightsProfile for PromptEnvelope {
    fn set_neurorights_profile(&mut self, profile: NeurorightsProfile) {
        self.neurorights_profile = profile;
    }
}
