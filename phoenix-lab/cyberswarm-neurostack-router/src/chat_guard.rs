//! Global ALN chat guard for CyberTunnel / dev-tunnel corridors.
//!
//! This module enforces:
//! - BCI-first ALN chat safety (terminal/env/safety/evidence gates).
//! - Absolute ban on Blake-family cryptography in all cybernetic domains.
//! - First-gate integration with CyberTunnelSession::handlerequest so
//!   no Blake-related intent can ever reach bioscale or router layers.[file:12][file:13]

use std::time::SystemTime;

use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cybertunnel::{
    CargoEnvDescriptor,
    CyberTunnelRequest,
    CyberTunnelResponse,
};
use crate::stakeholder::{StakeholderClass, StakeholderProfile};
use bioscale_upgrade_store::{EvidenceBundle, EvidenceTag};

/// Decision returned by guard_chat_command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardDecision {
    pub allowed: bool,
    pub reason: Option<String>,
    pub evidence_hash: Option<String>,
}

/// Static configuration for the chat guard, including the
/// pre-upgrade GuardEvidenceBundle.[file:14]
#[derive(Debug, Clone)]
pub struct ChatGuardConfig {
    pub required_terminal_class: &'static str,
    pub evidence_bundle: EvidenceBundle,
}

impl ChatGuardConfig {
    /// Pre-upgrade chat gate evidence bundle with 10 biophysical anchors.[file:14]
    pub fn pre_upgrade_chat_bundle() -> Self {
        let bundle = EvidenceBundle {
            sequences: vec![
                EvidenceTag {
                    short_hex: "0x1a7c3f".into(),
                    description: "EEG duty stability under <20 ms routing latency with TelemetryOnly chat intents.".into(),
                },
                EvidenceTag {
                    short_hex: "0x4b92d1".into(),
                    description: "Protein-consumption drift ≤2%/24h when upgrades require chat guard approval.".into(),
                },
                EvidenceTag {
                    short_hex: "0x8ef046".into(),
                    description: "Thermal envelope within 0.2 °C during high-frequency chat-driven Decoder traffic.".into(),
                },
                EvidenceTag {
                    short_hex: "0x33aa9c".into(),
                    description: "HRV baseline preservation when MotorAssist/SleepXR must pass neurorights + env gate.".into(),
                },
                EvidenceTag {
                    short_hex: "0x9d10b2".into(),
                    description: "Neurochemical stability when CriticalActuation is unreachable from chat without multisig.".into(),
                },
                EvidenceTag {
                    short_hex: "0x57c3ee".into(),
                    description: "No neural blink desynchronization when high-impact intents are denied at chat guard.".into(),
                },
                EvidenceTag {
                    short_hex: "0xf1047a".into(),
                    description: "BioKarma risk kept below thresholds when high-band chat routes require blood-token debits.".into(),
                },
                EvidenceTag {
                    short_hex: "0x6c2d51".into(),
                    description: "Reversible downgrade restores biomarkers ≤60 minutes on invariant breach.".into(),
                },
                EvidenceTag {
                    short_hex: "0xa8e0c9".into(),
                    description: "Stable sleep architecture when nocturnal chat actuation is clamped to TelemetryOnly/SoftActuation.".into(),
                },
                EvidenceTag {
                    short_hex: "0xd7b61f".into(),
                    description: "BCI decoding accuracy preserved within 1% when dev-tunnel upgrades are guard-mediated.".into(),
                },
            ],
        };

        ChatGuardConfig {
            required_terminal_class: "classcybernetic-stakeholder",
            evidence_bundle: bundle,
        }
    }
}

/// Parsed ALN chat envelope (result of regex/grammar layer). [SESSION/INTENT/SAFETY/EVIDENCE/TERMINAL][file:11][file:12]
#[derive(Debug, Clone)]
pub struct ParsedAlnChatCommand {
    pub session_line: String,
    pub intent_line: String,
    pub safety_line: String,
    pub evidence_line: String,
    pub terminal_line: String,
}

/// Blake is globally forbidden: any mention in INTENT is an automatic deny,
/// regardless of env posture, evidence, or stakeholder class.[file:13]
fn denies_blake_intent(intent_line: &str) -> bool {
    intent_line.to_ascii_lowercase().contains("blake")
}

/// Hash the evidence bundle into a short traceable anchor; use your existing
/// bioscale or audit hash helper here.
fn hash_evidence_bundle(bundle: &EvidenceBundle) -> String {
    // Minimal stable hash surrogate: concatenate short_hex tags and SHA-256 off-path.
    let mut s = String::new();
    for tag in &bundle.sequences {
        s.push_str(&tag.short_hex);
        s.push('|');
    }
    // In your real stack, route through your approved hash primitive (non-Blake). [file:13]
    format!("evidence-hash:{}", crate::hash::sha2_256_hex(s.as_bytes()))
}

/// Core chat guard: runs after ALN regex/grammar, before any UpgradeDescriptor mapping,
/// bioscale evaluateupgrade, or router.route_snapshot calls.[file:12][file:20]
pub fn guard_chat_command(
    cfg: &ChatGuardConfig,
    env: &CargoEnvDescriptor,
    stakeholder: &StakeholderProfile,
    cmd: &ParsedAlnChatCommand,
) -> GuardDecision {
    // 1. Terminal class: TERMINAL classcybernetic-stakeholder gridnode... [file:12][file:20]
    if !is_cybernetic_terminal(cfg.required_terminal_class, &cmd.terminal_line) {
        return GuardDecision {
            allowed: false,
            reason: Some("Terminal not classed as cybernetic-stakeholder; enforcing explain-only.".into()),
            evidence_hash: Some(hash_evidence_bundle(&cfg.evidence_bundle)),
        };
    }
    if stakeholder.class != StakeholderClass::CyberneticHost {
        return GuardDecision {
            allowed: false,
            reason: Some("Stakeholder DID is not classified as CyberneticHost.".into()),
            evidence_hash: Some(hash_evidence_bundle(&cfg.evidence_bundle)),
        };
    }

    // 2. Environment gate: must be BCI-safety-qualified and map session/intent to allowed target+repo.[file:13][file:11]
    if !env.is_bci_safety_qualified() {
        return GuardDecision {
            allowed: false,
            reason: Some("CargoEnvDescriptor is not BCI-safety-qualified.".into()),
            evidence_hash: Some(hash_evidence_bundle(&cfg.evidence_bundle)),
        };
    }
    if !env.is_target_allowed_from_session(&cmd.session_line) {
        return GuardDecision {
            allowed: false,
            reason: Some("Target triple not allowed by CargoEnvDescriptor.".into()),
            evidence_hash: Some(hash_evidence_bundle(&cfg.evidence_bundle)),
        };
    }
    if !env.is_repo_allowed_from_intent(&cmd.intent_line) {
        return GuardDecision {
            allowed: false,
            reason: Some("Repository/branch not allowed by CargoEnvDescriptor.".into()),
            evidence_hash: Some(hash_evidence_bundle(&cfg.evidence_bundle)),
        };
    }

    // 3. Safety clauses: reversibletrue, envgateReality.os, noraweegexport must be present.[file:11][file:12]
    if !safety_clauses_present(&cmd.safety_line) {
        return GuardDecision {
            allowed: false,
            reason: Some("Required SAFETY clauses missing (reversibletrue, envgateReality.os, noraweegexport).".into()),
            evidence_hash: Some(hash_evidence_bundle(&cfg.evidence_bundle)),
        };
    }

    // 4. Evidence bundle: full 10-sequence biophysical bundle must be present and registered.[file:11][file:14]
    if !evidence_bundle_matches(&cfg.evidence_bundle, &cmd.evidence_line) {
        return GuardDecision {
            allowed: false,
            reason: Some("EVIDENCE block does not match required 10-sequence GuardEvidenceBundle.".into()),
            evidence_hash: Some(hash_evidence_bundle(&cfg.evidence_bundle)),
        };
    }

    // 5. Blake global ban: any Blake token in INTENT is rejected before bioscale/router.[file:13]
    if denies_blake_intent(&cmd.intent_line) {
        return GuardDecision {
            allowed: false,
            reason: Some(
                "Blake-family cryptography is globally forbidden in this corridor (build, runtime, and network)."
                    .into(),
            ),
            evidence_hash: Some(hash_evidence_bundle(&cfg.evidence_bundle)),
        };
    }

    GuardDecision {
        allowed: true,
        reason: None,
        evidence_hash: Some(hash_evidence_bundle(&cfg.evidence_bundle)),
    }
}

fn is_cybernetic_terminal(required: &str, terminal_line: &str) -> bool {
    let re = Regex::new(r"^TERMINAL\s+classcybernetic-stakeholder\s+gridnode[a-z0-9]+").unwrap();
    terminal_line.contains(required) && re.is_match(terminal_line)
}

fn safety_clauses_present(safety_line: &str) -> bool {
    safety_line.contains("reversibletrue")
        && safety_line.contains("envgateReality.os")
        && safety_line.contains("noraweegexport")
}

fn evidence_bundle_matches(required: &EvidenceBundle, evidence_line: &str) -> bool {
    let re = Regex::new(r"^EVIDENCE\s+([0-9a-f]{8}\s+){9}[0-9a-f]{8}$").unwrap();
    if !re.is_match(evidence_line) {
        return false;
    }
    let parts: Vec<&str> = evidence_line.split_whitespace().skip(1).collect();
    if parts.len() != 10 {
        return false;
    }
    let required_tags: Vec<&str> = required
        .sequences
        .iter()
        .map(|t| t.short_hex.trim_start_matches("0x"))
        .collect();
    parts
        .iter()
        .all(|p| required_tags.iter().any(|t| t.eq_ignore_ascii_case(p)))
}

/// Integration hook for CyberTunnelSession: first-gate guard before any bioscale/router.
impl<S> crate::cybertunnel::CyberTunnelSession<S>
where
    S: bioscale_upgrade_store::BioscaleUpgradeStore + Send,
{
    pub fn handlerequest(
        &mut self,
        request: CyberTunnelRequest,
        parsed_cmd: ParsedAlnChatCommand,
        env: CargoEnvDescriptor,
        stakeholder: StakeholderProfile,
    ) -> CyberTunnelResponse {
        let cfg = ChatGuardConfig::pre_upgrade_chat_bundle();
        let decision = guard_chat_command(&cfg, &env, &stakeholder, &parsed_cmd);

        if !decision.allowed {
            return CyberTunnelResponse {
                session_id: self.sessionid,
                user_id: self.userid,
                allowed: false,
                router_decision: None,
                bioscale_decision: None,
                neurorights_denied: false,
                reason: decision.reason,
                city_payload: serde_json::json!({
                    "mode": "explain-only",
                    "status": "denied",
                    "component": "chat-guard",
                    "evidence_hash": decision.evidence_hash,
                }),
            };
        }

        // Only after guard passes:
        // - evaluatebioscale(capability, timestamp)
        // - router.route_snapshot(snapshot, policy).[file:20]
        let snapshot = self.gateway.export_state_snapshot();
        let policy = self.policy_for_capability(request.capability);
        let bioscale = self.evaluate_bioscale(request.capability, request.timestamp);

        match bioscale {
            bioscale_upgrade_store::UpgradeDecision::Denied { reason } => CyberTunnelResponse {
                session_id: self.sessionid,
                user_id: self.userid,
                allowed: false,
                router_decision: None,
                bioscale_decision: Some(format!("Denied: {}", reason)),
                neurorights_denied: false,
                reason: Some("Bioscale upgrade denied capability not scheduled.".into()),
                city_payload: serde_json::json!({
                    "mode": "explain-only",
                    "status": "denied",
                    "component": "bioscale",
                    "reason": reason,
                }),
            },
            bioscale_upgrade_store::UpgradeDecision::Approved { .. } => {
                let decision = self.router.route(snapshot.clone(), policy.clone());
                if decision.denied_reason.is_some() {
                    return CyberTunnelResponse {
                        session_id: self.sessionid,
                        user_id: self.userid,
                        allowed: false,
                        router_decision: Some(decision),
                        bioscale_decision: Some("Approved".into()),
                        neurorights_denied: true,
                        reason: Some("Neurorights safety check failed in router.".into()),
                        city_payload: serde_json::json!({
                            "mode": "explain-only",
                            "status": "denied",
                            "component": "router",
                            "reason": "neurorights-guard",
                        }),
                    };
                }

                let payload = self.build_city_payload(request, decision.clone());
                CyberTunnelResponse {
                    session_id: self.sessionid,
                    user_id: self.userid,
                    allowed: true,
                    router_decision: Some(decision),
                    bioscale_decision: Some("Approved".into()),
                    neurorights_denied: false,
                    reason: None,
                    city_payload: payload,
                }
            }
        }
    }
}
