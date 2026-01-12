use std::time::{Duration, SystemTime};

use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::bioscale_integration::{
    evaluate_intent_with_store,  // wrapper around BioscaleUpgradeStore::evaluateupgrade.[file:11][file:12]
};
use crate::router_core::{
    CyberSwarmNeurostackRouter,
    GatewayStateSnapshot,
    IntentClass,
    PolicyContext,
    RoutingDecision,
};

use bioscale_upgrade_store::{
    BioscaleUpgradeStore,
    EvidenceBundle,
    EvidenceTag,
    HostBudget,
    UpgradeDecision,
};

/// Blake-family pattern set; any match triggers quarantine.
/// Case-insensitive, matches "blake", "blake2", "blake3" in IDs, manifests, tags.
#[derive(Debug, Clone)]
pub struct BlakePatternSet {
    re: Regex,
}

impl BlakePatternSet {
    pub fn new() -> Self {
        // (?i) = case-insensitive, match any of the BLAKE tokens.
        let re = Regex::new(r"(?i)blake3?|blake2[bs]?").expect("valid blake regex");
        Self { re }
    }

    pub fn matches<S: AsRef<str>>(&self, s: S) -> bool {
        self.re.is_match(s.as_ref())
    }
}

/// Evidence bundle: why Blake-family crypto is quarantined in this BCI context.
/// This uses your 10-sequence pattern and maps the policy to biophysical safety.[file:11][file:12]
pub const BLAKE_QUARANTINE_EVIDENCE: EvidenceBundle = EvidenceBundle {
    sequences: [
        EvidenceTag {
            shorthex: "a1f3c9b2",
            description: "Resting metabolic rate / ATP load: cryptographic hot paths must not consume budget reserved for neural recovery.",
        },
        EvidenceTag {
            shorthex: "4be79d01",
            description: "Mitochondrial OXPHOS efficiency: sustained high-entropy compute (e.g., hashing) raises ATP demand near cortical tissue.",
        },
        EvidenceTag {
            shorthex: "9cd4a7e8",
            description: "Protein synthesis cost: elevated compute-induced stressors alter protein turnover in neuromodulatory pathways.",
        },
        EvidenceTag {
            shorthex: "2f8c6b44",
            description: "Thermoregulatory limits: cryptographic workloads can increase local and systemic heat, narrowing safe core ΔT.",
        },
        EvidenceTag {
            shorthex: "7e1da2ff",
            description: "Peripheral circulation adaptation: cryptographic coprocessors compete with neurovascular demands under load.",
        },
        EvidenceTag {
            shorthex: "5b93e0c3",
            description: "Neurovascular coupling: sustained compute bursts can confound BOLD/EEG-based safety telemetry.",
        },
        EvidenceTag {
            shorthex: "d0174aac",
            description: "Safe EEG-driven duty cycles: guardrails against covert high-rate stimulation piggybacking on crypto tasks.",
        },
        EvidenceTag {
            shorthex: "6ac2f9d9",
            description: "Neuromorphic ML energy profiles: reserve ML envelope for clinically intended models, not hash pipelines.",
        },
        EvidenceTag {
            shorthex: "c4e61b20",
            description: "Protein turnover in neural tissue: avoid chronic stressors from unnecessary cryptographic heat/ROS.",
        },
        EvidenceTag {
            shorthex: "8f09d5ee",
            description: "Inflammation/pain thresholds: any suspected malware path must be strictly reversible with clear triggers.",
        },
    ],
};

/// A single audit record for a Blake-related quarantine decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlakeQuarantineRecord {
    pub userid: Uuid,
    pub timestamp: u64,
    pub reason: String,
    pub context: String,
    pub intent: Option<IntentClass>,
    pub blake_token: Option<String>,
    pub evidence: EvidenceBundle,
}

/// Abstraction over your existing audit sink (e.g., HyperledgerVital, KernelAudit).[file:11]
#[async_trait::async_trait]
pub trait BlakeAuditSink: Send + Sync {
    async fn record(&self, rec: BlakeQuarantineRecord);
}

/// Simple stdout sink for lab bring-up; swap with Hyperledger-backed sink in prod.
pub struct StdoutBlakeSink;

#[async_trait::async_trait]
impl BlakeAuditSink for StdoutBlakeSink {
    async fn record(&self, rec: BlakeQuarantineRecord) {
        println!(
            "[BLAKE-QUARANTINE] user={} ts={} reason={} ctx={} token={:?}",
            rec.userid, rec.timestamp, rec.reason, rec.context, rec.blake_token
        );
    }
}

/// BlakePolicyGuard sits in front of the router and Bioscale store and enforces:
///   - No BLAKE tokens in ALN shard IDs, OTA manifests, or upgrade descriptors.
///   - No BLAKE tokens in nanoswarm / BLE device logical names (hooked via NanoDeviceDescriptor).
///   - All denials are evidence-tagged and auditable.
pub struct BlakePolicyGuard<S: BioscaleUpgradeStore, A: BlakeAuditSink> {
    patterns: BlakePatternSet,
    store: S,
    audit: A,
}

impl<S: BioscaleUpgradeStore, A: BlakeAuditSink> BlakePolicyGuard<S, A> {
    pub fn new(store: S, audit: A) -> Self {
        Self {
            patterns: BlakePatternSet::new(),
            store,
            audit,
        }
    }

    /// Core predicate: decide if a text token is Blake-related.
    pub fn is_blake_token<Ss: AsRef<str>>(&self, s: Ss) -> bool {
        self.patterns.matches(s)
    }

    /// Check OTA manifest paths, upgrade IDs, and any crate metadata for Blake markers.
    fn upgrade_uses_blake(&self, upgrade_id: &str, otamanifest: &str, meta: &[&str]) -> bool {
        if self.is_blake_token(upgrade_id) || self.is_blake_token(otamanifest) {
            return true;
        }
        meta.iter().any(|m| self.is_blake_token(m))
    }

    /// Wrap the existing evaluate_intent_with_store to add Blake-family checks.
    pub async fn evaluate_intent_with_blake_guard(
        &self,
        router: &CyberSwarmNeurostackRouter,
        snapshot: GatewayStateSnapshot,
        policy: PolicyContext,
    ) -> RoutingDecision {
        // Neurorights guard remains first gate.[file:11]
        if !CyberSwarmNeurostackRouter::check_state_neurorights_safe(&snapshot) {
            let rec = BlakeQuarantineRecord {
                userid: snapshot.userid,
                timestamp: snapshot.timestamp,
                reason: "Neurorights safety check failed before Blake inspection".into(),
                context: "router.check_state_neurorights_safe".into(),
                intent: Some(policy.intent),
                blake_token: None,
                evidence: BLAKE_QUARANTINE_EVIDENCE,
            };
            self.audit.record(rec).await;
            return RoutingDecision {
                userid: snapshot.userid,
                selectedtargets: vec![],
                deniedreason: Some(
                    "Neurorights safety check failed for decoded states".into(),
                ),
            };
        }

        // Resolve UpgradeDescriptor for this intent via your existing mapping.[file:11][file:12]
        let intent = policy.intent;
        let now = snapshot.timestamp;

        // Derive a conservative host budget snapshot for this user.
        // (You already do this in bioscale_integration::host_budget_from_snapshot.)[file:11][file:12]
        let host: HostBudget = crate::bioscale_integration::host_budget_from_snapshot(now);

        // Lookup upgrade ID and descriptor from store.
        let (upgrade_id, upgrade_desc) =
            crate::bioscale_intent_map::upgrade_for_intent(intent, &self.store);

        // Layer 1: purely structural / metadata scan for Blake tokens.
        let mut meta_tokens: Vec<&str> = Vec::new();
        meta_tokens.push(upgrade_desc.github_repo_slug);
        meta_tokens.push(upgrade_desc.ota_manifest_path);
        meta_tokens.push(upgrade_desc.display_name);
        meta_tokens.push(upgrade_desc.version);

        if self.upgrade_uses_blake(upgrade_id.0, upgrade_desc.ota_manifest_path, &meta_tokens) {
            let rec = BlakeQuarantineRecord {
                userid: snapshot.userid,
                timestamp: snapshot.timestamp,
                reason: "Blake-family cryptography detected in upgrade metadata; corridor requires non-Blake primitives."
                    .into(),
                context: "BlakePolicyGuard::upgrade_uses_blake".into(),
                intent: Some(intent),
                blake_token: Some("metadata".into()),
                evidence: BLAKE_QUARANTINE_EVIDENCE,
            };
            self.audit.record(rec).await;
            return RoutingDecision {
                userid: snapshot.userid,
                selectedtargets: vec![],
                deniedreason: Some(
                    "Blake-family cryptography not permitted for this BCI corridor.".into(),
                ),
            };
        }

        // Layer 2: bioscale evaluation (energy/protein/thermo) remains in force.[file:11][file:12]
        match evaluate_intent_with_store(&self.store, intent, snapshot.timestamp) {
            UpgradeDecision::Denied { reason } => {
                let rec = BlakeQuarantineRecord {
                    userid: snapshot.userid,
                    timestamp: snapshot.timestamp,
                    reason: format!("Bioscale upgrade denied: {}", reason),
                    context: "BioscaleUpgradeStore::evaluate_upgrade".into(),
                    intent: Some(intent),
                    blake_token: None,
                    evidence: BLAKE_QUARANTINE_EVIDENCE,
                };
                self.audit.record(rec).await;
                RoutingDecision {
                    userid: snapshot.userid,
                    selectedtargets: vec![],
                    deniedreason: Some(format!(
                        "Bioscale envelope denied for intent {:?}: {}",
                        intent, reason
                    )),
                }
            }
            UpgradeDecision::Approved {
                scheduled_at: _,
                expected_completion: _,
            } => {
                // Only after passing Blake and bioscale gates, call the existing router.[file:11]
                router.route(snapshot, policy)
            }
        }
    }
}

/// Optional: integrate with CyberNanoGuard to block Blake-labeled implants at the BLE edge.[file:11]
use crate::cybernano_guard_v2::{CyberNanoGuard, NanoDeviceDescriptor};

impl<S: BioscaleUpgradeStore, A: BlakeAuditSink> BlakePolicyGuard<S, A> {
    pub async fn filter_nano_device(
        &self,
        guard: &CyberNanoGuard,
        dev: NanoDeviceDescriptor,
        userid: &str,
    ) -> bool {
        let name = dev.logical_name.to_ascii_lowercase();
        let mac = dev.mac_addr.to_ascii_uppercase();

        if self.is_blake_token(&name) || self.is_blake_token(&mac) {
            // Permanently block this endpoint; treat as suspected Blake-channel.
            guard
                .block_permanently(
                    dev.clone(),
                    "Blake-family pattern detected in device identifier; quarantined.",
                )
                .await;

            let rec = BlakeQuarantineRecord {
                userid: Uuid::parse_str(userid).unwrap_or_else(|_| dev.id.0),
                timestamp: current_unix_ts(),
                reason: "Nano device quarantined due to Blake-family naming pattern".into(),
                context: "BlakePolicyGuard::filter_nano_device".into(),
                intent: None,
                blake_token: Some(name),
                evidence: BLAKE_QUARANTINE_EVIDENCE,
            };
            self.audit.record(rec).await;
            false
        } else {
            true
        }
    }
}

fn current_unix_ts() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_secs()
}

// ----- ALN / "?" object shard for policy exposure (ALN syntax) -----
//
// ?blake_quarantine {
//   policy: "no-blake-crypto-in-bci-corridor",
//   evidence: [
//     "a1f3c9b2", "4be79d01", "9cd4a7e8", "2f8c6b44", "7e1da2ff",
//     "5b93e0c3", "d0174aac", "6ac2f9d9", "c4e61b20", "8f09d5ee"
//   ],
//   reversal_allowed: true,
//   scope: ["VitalNetBCI", "CyberNanoGuard", "NeurostackRouter"],
//   downgrade_trigger: "any blake-token match OR bioscale-deny"
// }
//
// This ALN particle can be bound to your Bostrom DID to make this corridor
// policy legally visible and machine-verifiable in the governance layer.[file:5][file:12]
