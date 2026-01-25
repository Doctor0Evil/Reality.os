use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::json;
use crate::{PromptEnvelope, Identity, Provenance, Governance, Intent, SecurityLevel};
use neurorights_firewall::NeurorightsProfile;

pub struct RawPrompt<'a> {
    pub user_did: &'a str,
    pub text: &'a str,
    pub security_level: SecurityLevel,
    pub source_client: &'a str,
    pub session_id: &'a str,
    pub aln: &'a str,
    pub bostrom_address: &'a str,
}

pub fn normalize_prompt(raw: RawPrompt) -> PromptEnvelope {
    let now = SystemTime::now();
    let ts = now.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let bucket = (ts / 86_400).to_string();

    let trace_id = make_trace_id(raw.user_did, raw.text, &bucket);
    let intent = infer_intent(raw.text);
    let args = json!({ "prompt": raw.text });

    let identity = Identity {
        user_did: raw.user_did.to_string(),
        aln: raw.aln.to_string(),
        bostrom_address: raw.bostrom_address.to_string(),
    };

    let provenance = Provenance {
        created_at: ts.to_string(),
        source_client: raw.source_client.to_string(),
        model_context: "Cyber-Retrieval-Router-v1".to_string(),
        session_id: raw.session_id.to_string(),
        parent_trace_id: None,
    };

    let governance = Governance {
        eibon_label: "EibonProposal".to_string(),
        policy_scope: "Cyber-Retrieval".to_string(),
        jurisdiction: "Phoenix-AZ-US".to_string(),
    };

    let profile = NeurorightsProfile::citizen_v1("did:web:cybercore-brain.org/neurorights");

    PromptEnvelope {
        trace_id,
        intent,
        args,
        security_level: raw.security_level,
        identity,
        provenance,
        governance,
        neurorights_profile: profile,
    }
}

// Deterministic fold; non-cryptographic, governance-safe.
fn make_trace_id(user_did: &str, text: &str, bucket: &str) -> String {
    let input = format!("{user_did}:{text}:{bucket}");
    let mut acc: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        acc = acc.wrapping_mul(1_099_511_628_211);
        acc ^= *b as u64;
    }
    format!("0x{acc:016x}")
}

fn infer_intent(text: &str) -> Intent {
    let lower = text.to_ascii_lowercase();
    if lower.contains("policy") || lower.contains("governance") {
        Intent::Governance
    } else if lower.contains("simulate") || lower.contains("simulation") {
        Intent::Simulate
    } else if lower.contains("plan") {
        Intent::Plan
    } else if lower.contains("analyz") {
        Intent::Analyze
    } else if lower.contains("retrieve") || lower.contains("lookup") || lower.contains("fetch") {
        Intent::Retrieve
    } else {
        Intent::Unknown
    }
}
