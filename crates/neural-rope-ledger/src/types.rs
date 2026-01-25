use serde::{Serialize, Deserialize};
use serde_json::Value;
use cyber_retrieval_types::PromptEnvelope;
use neurorights_firewall::{NeurorightsEnvelope, NeurorightsBound};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RopeStep {
    pub index: u64,
    pub envelope: PromptEnvelope,
    pub tool_name: String,
    pub tool_args_hash: String,
    pub tool_output_hash: String,
    pub knowledge_factor: f32,
    pub risk_of_harm: f32,
    pub cybostate_factor: f32,
    pub hex_stamp: String,
    pub eco_impact: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralRopeId {
    pub rope_id: String,
    pub steps: Vec<RopeStep>,
}

impl NeuralRopeId {
    pub fn new(rope_id: String) -> Self {
        Self { rope_id, steps: Vec::new() }
    }

    pub fn append_step(
        &mut self,
        prev_hex: &str,
        env: &NeurorightsBound<PromptEnvelope, NeurorightsEnvelope>,
        tool_name: &str,
        tool_args_hash: &str,
        tool_output_hash: &str,
        k: f32,
        r: f32,
        c: f32,
        eco: f32,
    ) {
        let idx = self.steps.len() as u64;
        let hex_stamp = link_hex(prev_hex, tool_name, tool_args_hash, tool_output_hash, idx);

        self.steps.push(RopeStep {
            index: idx,
            envelope: env.inner().clone(),
            tool_name: tool_name.to_string(),
            tool_args_hash: tool_args_hash.to_string(),
            tool_output_hash: tool_output_hash.to_string(),
            knowledge_factor: k,
            risk_of_harm: r,
            cybostate_factor: c,
            hex_stamp,
            eco_impact: eco,
        });
    }
}

// Deterministic “rope” linking; stays non-crypto but hash-like in Rust.
fn link_hex(
    prev_hex: &str,
    tool_name: &str,
    args_hash: &str,
    out_hash: &str,
    idx: u64,
) -> String {
    let input = format!("{prev_hex}:{tool_name}:{args_hash}:{out_hash}:{idx}");
    let mut acc: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        acc = acc.wrapping_mul(1_099_511_628_211);
        acc ^= *b as u64;
    }
    format!("0x{acc:016x}")
}
