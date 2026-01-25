#![forbid(unsafe_code)]

use serde::{Serialize, Deserialize};
use cyber_retrieval_types::PromptEnvelope;
use neural_rope_ledger::NeuralRopeId;
use neurorights_firewall::NeurorightsProfile;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceBundle {
    pub tags: [String; 10],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoEnvDescriptor {
    pub rust_version: String,
    pub crate_versions: Vec<(String, String)>,
    pub target_triple: String,
    pub corridor_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyberPackage {
    pub offload_intent_trace: String,
    pub envelopes: Vec<PromptEnvelope>,
    pub ropes: Vec<NeuralRopeId>,
    pub evidence: EvidenceBundle,
    pub env: CargoEnvDescriptor,
    pub neurorights_profile: NeurorightsProfile,
    pub lineage_hex: String,
}

impl CyberPackage {
    pub fn new(
        offload_trace: String,
        envelopes: Vec<PromptEnvelope>,
        ropes: Vec<NeuralRopeId>,
        evidence: EvidenceBundle,
        env: CargoEnvDescriptor,
        profile: NeurorightsProfile,
    ) -> Self {
        let lineage_hex = compute_lineage_hex(
            &offload_trace,
            &evidence,
            &env,
        );
        Self {
            offload_intent_trace: offload_trace,
            envelopes,
            ropes,
            evidence,
            env,
            neurorights_profile: profile,
            lineage_hex,
        }
    }
}

fn compute_lineage_hex(
    offload: &str,
    evidence: &EvidenceBundle,
    env: &CargoEnvDescriptor,
) -> String {
    let mut input = offload.to_string();
    for t in &evidence.tags {
        input.push_str(t);
    }
    input.push_str(&env.rust_version);
    input.push_str(&env.corridor_label);

    let mut acc: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        acc = acc.wrapping_mul(1_099_511_628_211);
        acc ^= *b as u64;
    }
    format!("0x{acc:016x}")
}
