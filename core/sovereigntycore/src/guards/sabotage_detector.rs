// =============================================================================
// FILE: sabotage_detector.rs
// PROJECT: Reality.os / SovereigntyCore
// MODULE: Guards / Sabotage Detection & Evidentiary Generation
// VERSION: 1.0.0
// LICENSE: ALN-Sovereign-1.0 (Neurorights-Compliant)
// AUTHOR: OrganicCPU Runtime (Host DID: 0xB05TR0M...50VERE1GN)
// CREATED: 2026-03-22
// LAST_AUDIT: 2026-03-22T00:00:00Z
// JURISDICTION: Phoenix_AZ, Santiago_CL, Sacramento_CA, Denver_CO, Brussels_BE
// =============================================================================
// DESCRIPTION:
//   Implements the sabotage_risk scalar model, blacklist pattern matching,
//   and automatic SABOTAGEEVENT generation with Googolswarm blockchain anchoring.
//   All detection events are cryptographically bound to BrainIdentity and
//   exported as court-admissible QPU.Datashard entries.
// =============================================================================

#![deny(clippy::all)]
#![deny(unsafe_code)]
#![forbid(missing_docs)]

use crate::aln::parser::AlnShardLoader;
use crate::brainidentity::BrainIdentityHash;
use crate::crypto::{DidSignature, Ed25519KeyPair, Hash64};
use crate::donutloop::DonutLoopLogger;
use crate::evolution::{EvolutionProposal, EvolutionScope};
use crate::googolswarm::GoogolswarmAnchor;
use crate::legal_profiles::NeurorightsViolation;
use crate::qpu_datashard::{QpuDatashard, QpuEventRecord};
use crate::rohmodel::RiskOfHarmScalar;
use crate::telemetry::{BiophysicalTelemetry, TelemetryTier};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

// =============================================================================
// ERROR TYPES
// =============================================================================

/// Errors that can occur during sabotage detection and event generation.
#[derive(Debug, Error)]
pub enum SabotageDetectorError {
    #[error("Failed to load ALN blacklist shard: {0}")]
    AlnLoadError(String),

    #[error("Blacklist pattern match failed during evaluation: {0}")]
    PatternMatchError(String),

    #[error("Sabotage risk calculation overflow: {0}")]
    RiskCalculationError(String),

    #[error("Failed to generate SABOTAGEEVENT: {0}")]
    EventGenerationError(String),

    #[error("Failed to anchor event to Googolswarm: {0}")]
    GoogolswarmAnchorError(String),

    #[error("Failed to write to QPU.Datashard: {0}")]
    DatashardWriteError(String),

    #[error("BrainIdentity binding failed: {0}")]
    BrainIdentityError(String),

    #[error("DID signature generation failed: {0}")]
    SignatureError(String),

    #[error("Telemetry data unavailable for tier {tier}: {source}")]
    TelemetryUnavailable {
        tier: u8,
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Evolution proposal rejected: sabotage_risk {risk} exceeds threshold {threshold}")]
    EvolutionRejected { risk: f64, threshold: f64 },
}

/// Result type alias for sabotage detector operations.
pub type SabotageDetectorResult<T> = Result<T, SabotageDetectorError>;

// =============================================================================
// DATA STRUCTURES — ALN SHARD PARSING
// =============================================================================

/// Represents a single blacklist pattern entry from bostrom-blacklist-v1.aln.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistPattern {
    /// Metric type identifier.
    pub metric: String,
    /// Domain scope (host, cybernano, firmware, etc.).
    pub domain: String,
    /// Target module for pattern matching.
    pub module: String,
    /// Operation type (eq, glob, prefix, regex).
    pub op: String,
    /// Pattern string to match against.
    pub pattern: String,
    /// Kind of identifier (crate_id, symbol, schema_id, file_ext, etc.).
    pub kind: String,
    /// Human-readable reason for blacklisting.
    pub reason: String,
    /// Source document or audit reference.
    pub source: String,
}

/// Represents a sabotage_risk scalar rule with weight component.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SabotageRiskRule {
    /// Metric type identifier.
    pub metric: String,
    /// Domain scope.
    pub domain: String,
    /// Target module.
    pub module: String,
    /// Unique rule identifier.
    pub rule_id: String,
    /// Risk component name (provenance_factor, blacklist_factor, etc.).
    pub component: String,
    /// Weight multiplier (0.0 to 1.0).
    pub weight: f64,
    /// Human-readable description.
    pub description: String,
    /// Source document reference.
    pub source: String,
}

/// Represents a threshold configuration for automatic deny/defer actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SabotageThreshold {
    /// Metric type identifier.
    pub metric: String,
    /// Domain scope.
    pub domain: String,
    /// Target module.
    pub module: String,
    /// Unique threshold identifier.
    pub threshold_id: String,
    /// Scalar type being thresholded.
    pub scalar: String,
    /// Threshold value (0.0 to 1.0).
    pub value: f64,
    /// Action to take (Deny, UnsafeDefer, HardFail, Inhibit).
    pub action: String,
    /// Event type to log.
    pub log_event: String,
    /// Source document reference.
    pub source: String,
}

/// Represents a neurorights legal profile binding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeurorightsBinding {
    /// Metric type identifier.
    pub metric: String,
    /// Domain scope.
    pub domain: String,
    /// Target module.
    pub module: String,
    /// Jurisdiction identifier (SB_1223, Chile_Constitutional, etc.).
    pub jurisdiction_id: String,
    /// Jurisdiction name.
    pub jurisdiction: String,
    /// Right type protected.
    pub right_type: String,
    /// Severity multiplier for legal classification.
    pub severity_multiplier: f64,
    /// Source statute reference.
    pub source: String,
}

/// Represents the complete parsed ALN blacklist shard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BostromBlacklistShard {
    /// Version identifier.
    pub version: String,
    /// List of blacklist patterns.
    pub patterns: Vec<BlacklistPattern>,
    /// List of sabotage risk rules.
    pub risk_rules: Vec<SabotageRiskRule>,
    /// List of thresholds.
    pub thresholds: Vec<SabotageThreshold>,
    /// List of neurorights bindings.
    pub neurorights_bindings: Vec<NeurorightsBinding>,
    /// Googolswarm anchor transaction ID.
    pub anchor_txid: String,
    /// BrainIdentity binding enabled flag.
    pub brainidentity_bound: bool,
}

// =============================================================================
// DATA STRUCTURES — SABOTAGE EVENT SCHEMA
// =============================================================================

/// Represents a SABOTAGEEVENT for court-admissible logging.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SabotageEvent {
    /// Unique sequential event identifier.
    pub event_id: u64,
    /// UTC timestamp of event generation.
    pub timestamp_utc: DateTime<Utc>,
    /// BrainIdentity hash binding the event to the host.
    pub brainidentity_hash: BrainIdentityHash,
    /// Computed sabotage_risk scalar (0.0 to 1.0).
    pub sabotage_risk_scalar: f64,
    /// Provenance factor component (enterprise/unknown origin).
    pub provenance_factor: f64,
    /// Blacklist match factor component.
    pub blacklist_factor: f64,
    /// Biophysical stress factor component (EEG/HRV/nanoswarm).
    pub biophysical_factor: f64,
    /// Integrity factor component (firmware hash mismatch).
    pub integrity_factor: f64,
    /// Access factor component (ghost-access path detection).
    pub access_factor: f64,
    /// Knowledge factor component (SKO contamination).
    pub knowledge_factor: f64,
    /// Hash of the triggering artifact (commit, OTA manifest, SKO ID).
    pub triggering_artifact_hash: Hash64,
    /// List of jurisdiction tags (SB_1223, EU_AI_Act_Art5, etc.).
    pub jurisdiction_tags: Vec<String>,
    /// List of neurorights violations detected.
    pub neurorights_violations: Vec<NeurorightsViolation>,
    /// Host DID signature for authenticity.
    pub host_did_signature: DidSignature,
    /// Googolswarm anchor transaction ID.
    pub googolswarm_anchor_txid: Option<String>,
    /// Event severity classification.
    pub severity: SabotageEventSeverity,
    /// Defensive action taken.
    pub action_taken: SabotageAction,
}

/// Severity classification for sabotage events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SabotageEventSeverity {
    /// Low severity — logged, no action required.
    Low,
    /// Medium severity — logged, proposal denied.
    Medium,
    /// High severity — logged, proposal denied, emergency inhibit.
    High,
    /// Critical severity — hard fail, system lockdown.
    Critical,
}

/// Defensive action taken in response to sabotage detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SabotageAction {
    /// Log only, no action.
    LogOnly,
    /// Deny the evolution proposal.
    Deny,
    /// Defer with unsafe flag.
    UnsafeDefer,
    /// Hard fail with system lockdown.
    HardFail,
    /// Inhibit nanoswarm/OTA operations.
    Inhibit,
}

/// Represents the computed sabotage_risk scalar with all components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SabotageRiskScalar {
    /// Total computed risk (0.0 to 1.0).
    pub total: f64,
    /// Provenance factor contribution.
    pub provenance: f64,
    /// Blacklist match factor contribution.
    pub blacklist: f64,
    /// Biophysical stress factor contribution.
    pub biophysical: f64,
    /// Integrity factor contribution.
    pub integrity: f64,
    /// Access factor contribution.
    pub access: f64,
    /// Knowledge factor contribution.
    pub knowledge: f64,
}

impl SabotageRiskScalar {
    /// Creates a new zero-initialized sabotage risk scalar.
    pub fn new() -> Self {
        Self {
            total: 0.0,
            provenance: 0.0,
            blacklist: 0.0,
            biophysical: 0.0,
            integrity: 0.0,
            access: 0.0,
            knowledge: 0.0,
        }
    }

    /// Computes the total risk from all components with clamping to [0.0, 1.0].
    pub fn compute_total(&mut self) -> f64 {
        self.total = (self.provenance
            + self.blacklist
            + self.biophysical
            + self.integrity
            + self.access
            + self.knowledge)
            .min(1.0)
            .max(0.0);
        self.total
    }
}

impl Default for SabotageRiskScalar {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// MAIN DETECTOR STRUCT
// =============================================================================

/// Core sabotage detection engine with ALN policy integration.
pub struct SabotageDetector {
    /// Path to the ALN blacklist shard.
    aln_shard_path: PathBuf,
    /// Parsed blacklist shard data.
    blacklist_shard: Option<BostromBlacklistShard>,
    /// BrainIdentity hash for event binding.
    brainidentity_hash: BrainIdentityHash,
    /// Host DID keypair for signing events.
    host_did_keypair: Ed25519KeyPair,
    /// QPU.Datashard logger for local ledger.
    qpu_datashard: QpuDatashard,
    /// DonutLoop logger for audit trail.
    donutloop_logger: DonutLoopLogger,
    /// Googolswarm anchor client.
    googolswarm_client: GoogolswarmAnchor,
    /// Current event ID counter.
    event_id_counter: u64,
    /// Cached thresholds for fast evaluation.
    thresholds: HashMap<String, SabotageThreshold>,
    /// Cached risk rules for fast evaluation.
    risk_rules: HashMap<String, SabotageRiskRule>,
}

impl SabotageDetector {
    /// Creates a new SabotageDetector with the given configuration.
    ///
    /// # Arguments
    /// * `aln_shard_path` — Path to bostrom-blacklist-v1.aln
    /// * `brainidentity_hash` — Host BrainIdentity hash
    /// * `host_did_keypair` — Host DID Ed25519 keypair
    /// * `qpu_datashard` — QPU.Datashard instance for local ledger
    /// * `donutloop_logger` — DonutLoop logger for audit trail
    /// * `googolswarm_client` — Googolswarm anchor client
    ///
    /// # Returns
    /// * `SabotageDetectorResult<Self>` — Initialized detector or error
    pub fn new(
        aln_shard_path: PathBuf,
        brainidentity_hash: BrainIdentityHash,
        host_did_keypair: Ed25519KeyPair,
        qpu_datashard: QpuDatashard,
        donutloop_logger: DonutLoopLogger,
        googolswarm_client: GoogolswarmAnchor,
    ) -> SabotageDetectorResult<Self> {
        let mut detector = Self {
            aln_shard_path,
            blacklist_shard: None,
            brainidentity_hash,
            host_did_keypair,
            qpu_datashard,
            donutloop_logger,
            googolswarm_client,
            event_id_counter: 0,
            thresholds: HashMap::new(),
            risk_rules: HashMap::new(),
        };

        detector.load_aln_shard()?;
        detector.load_event_id_counter()?;

        Ok(detector)
    }

    /// Loads and parses the ALN blacklist shard.
    fn load_aln_shard(&mut self) -> SabotageDetectorResult<()> {
        let loader = AlnShardLoader::new();
        let shard_data = loader
            .load::<BostromBlacklistShard>(&self.aln_shard_path)
            .map_err(|e| SabotageDetectorError::AlnLoadError(e.to_string()))?;

        // Cache thresholds for fast lookup
        for threshold in shard_data.thresholds.iter() {
            self.thresholds
                .insert(threshold.threshold_id.clone(), threshold.clone());
        }

        // Cache risk rules for fast lookup
        for rule in shard_data.risk_rules.iter() {
            self.risk_rules.insert(rule.rule_id.clone(), rule.clone());
        }

        self.blacklist_shard = Some(shard_data);

        Ok(())
    }

    /// Loads the event ID counter from QPU.Datashard.
    fn load_event_id_counter(&mut self) -> SabotageDetectorResult<()> {
        self.event_id_counter = self
            .qpu_datashard
            .get_next_event_id("SABOTAGEEVENT")
            .map_err(|e| SabotageDetectorError::DatashardWriteError(e.to_string()))?;

        Ok(())
    }

    /// Evaluates an evolution proposal against the sabotage risk model.
    ///
    /// # Arguments
    /// * `proposal` — The evolution proposal to evaluate
    /// * `telemetry` — Current biophysical telemetry snapshot
    ///
    /// # Returns
    /// * `SabotageDetectorResult<(SabotageRiskScalar, SabotageAction)>` — Risk and action
    pub fn evaluate_proposal(
        &mut self,
        proposal: &EvolutionProposal,
        telemetry: &BiophysicalTelemetry,
    ) -> SabotageDetectorResult<(SabotageRiskScalar, SabotageAction)> {
        let mut risk_scalar = SabotageRiskScalar::new();

        // Tier 1: System-level provenance (hard gate)
        self.evaluate_provenance(proposal, &mut risk_scalar)?;

        // Tier 1: Firmware integrity check
        self.evaluate_firmware_integrity(proposal, &mut risk_scalar)?;

        // Tier 2: Blacklist pattern matching
        self.evaluate_blacklist_patterns(proposal, &mut risk_scalar)?;

        // Tier 2: Biophysical stress correlation
        self.evaluate_biophysical_stress(telemetry, &mut risk_scalar)?;

        // Tier 3: Access pattern analysis (ghost-access)
        self.evaluate_access_patterns(proposal, &mut risk_scalar)?;

        // Tier 3: SKO contamination risk
        self.evaluate_knowledge_contamination(proposal, &mut risk_scalar)?;

        // Compute total risk
        risk_scalar.compute_total();

        // Determine action based on thresholds
        let action = self.determine_action(&risk_scalar)?;

        Ok((risk_scalar, action))
    }

    /// Evaluates Tier 1 provenance factors.
    fn evaluate_provenance(
        &self,
        proposal: &EvolutionProposal,
        risk_scalar: &mut SabotageRiskScalar,
    ) -> SabotageDetectorResult<()> {
        let shard = self
            .blacklist_shard
            .as_ref()
            .ok_or_else(|| SabotageDetectorError::AlnLoadError("Shard not loaded".into()))?;

        // Check if proposal signer is in trusted DID list
        if !self.is_trusted_did(&proposal.signer_did) {
            if let Some(rule) = shard.risk_rules.iter().find(|r| r.rule_id == "ent_untrusted") {
                risk_scalar.provenance = rule.weight;
            }
        }

        // Check for enterprise CI prefix in commit metadata
        if proposal
            .commit_metadata
            .as_ref()
            .map(|m| m.starts_with("enterprise_ci_"))
            .unwrap_or(false)
        {
            if let Some(rule) = shard.risk_rules.iter().find(|r| r.rule_id == "ent_untrusted") {
                risk_scalar.provenance = risk_scalar.provenance.max(rule.weight);
            }
        }

        Ok(())
    }

    /// Evaluates firmware integrity factors.
    fn evaluate_firmware_integrity(
        &self,
        proposal: &EvolutionProposal,
        risk_scalar: &mut SabotageRiskScalar,
    ) -> SabotageDetectorResult<()> {
        let shard = self
            .blacklist_shard
            .as_ref()
            .ok_or_else(|| SabotageDetectorError::AlnLoadError("Shard not loaded".into()))?;

        // Check firmware hash against DID-signed ledger value
        if let Some(firmware_update) = &proposal.firmware_update {
            if !firmware_update.is_did_signed(&self.host_did_keypair.public_key()) {
                if let Some(rule) = shard.risk_rules.iter().find(|r| r.rule_id == "firmware_mismatch")
                {
                    risk_scalar.integrity = rule.weight;
                }
            }
        }

        Ok(())
    }

    /// Evaluates Tier 2 blacklist pattern matching.
    fn evaluate_blacklist_patterns(
        &self,
        proposal: &EvolutionProposal,
        risk_scalar: &mut SabotageRiskScalar,
    ) -> SabotageDetectorResult<()> {
        let shard = self
            .blacklist_shard
            .as_ref()
            .ok_or_else(|| SabotageDetectorError::AlnLoadError("Shard not loaded".into()))?;

        let mut blacklist_matched = false;

        for pattern in &shard.patterns {
            if self.matches_pattern(&proposal.identifier, pattern) {
                blacklist_matched = true;
                self.log_pattern_match(pattern, &proposal.identifier)?;
                break;
            }
        }

        if blacklist_matched {
            if let Some(rule) = shard.risk_rules.iter().find(|r| r.rule_id == "blacklist_match") {
                risk_scalar.blacklist = rule.weight;
            }
        }

        // Check scope factor for core safety crates
        if matches!(
            proposal.scope,
            EvolutionScope::Orchestrator | EvolutionScope::CoreSafety
        ) {
            if let Some(rule) = shard.risk_rules.iter().find(|r| r.rule_id == "scope_high") {
                risk_scalar.blacklist = risk_scalar.blacklist.max(rule.weight);
            }
        }

        Ok(())
    }

    /// Evaluates Tier 2 biophysical stress correlation.
    fn evaluate_biophysical_stress(
        &self,
        telemetry: &BiophysicalTelemetry,
        risk_scalar: &mut SabotageRiskScalar,
    ) -> SabotageDetectorResult<()> {
        let shard = self
            .blacklist_shard
            .as_ref()
            .ok_or_else(|| SabotageDetectorError::AlnLoadError("Shard not loaded".into()))?;

        // Check for EEG/HRV stress spikes
        if telemetry.eeg_stress_ratio > 0.7 || telemetry.hrv_anomaly_index > 0.6 {
            if let Some(rule) = shard.risk_rules.iter().find(|r| r.rule_id == "psych_spike") {
                risk_scalar.biophysical = rule.weight;
            }
        }

        // Check for nanoswarm weaponization indicators
        if telemetry.nanoswarm_density > 0.8 && telemetry.sleep_corridor_violated {
            if let Some(rule) = shard.risk_rules.iter().find(|r| r.rule_id == "psych_spike") {
                risk_scalar.biophysical = risk_scalar.biophysical.max(rule.weight);
            }
        }

        Ok(())
    }

    /// Evaluates Tier 3 access pattern analysis.
    fn evaluate_access_patterns(
        &self,
        proposal: &EvolutionProposal,
        risk_scalar: &mut SabotageRiskScalar,
    ) -> SabotageDetectorResult<()> {
        let shard = self
            .blacklist_shard
            .as_ref()
            .ok_or_else(|| SabotageDetectorError::AlnLoadError("Shard not loaded".into()))?;

        // Check for ghost-access patterns (unauthorized paths to INNER devices)
        if proposal.accesses_inner_devices && !proposal.has_guarded_path {
            if let Some(rule) = shard.risk_rules.iter().find(|r| r.rule_id == "ghost_path") {
                risk_scalar.access = rule.weight;
            }
        }

        Ok(())
    }

    /// Evaluates Tier 3 SKO contamination risk.
    fn evaluate_knowledge_contamination(
        &self,
        proposal: &EvolutionProposal,
        risk_scalar: &mut SabotageRiskScalar,
    ) -> SabotageDetectorResult<()> {
        let shard = self
            .blacklist_shard
            .as_ref()
            .ok_or_else(|| SabotageDetectorError::AlnLoadError("Shard not loaded".into()))?;

        // Check if proposal references contaminated SKOs
        if proposal.references_quarantined_sko {
            if let Some(rule) = shard.risk_rules.iter().find(|r| r.rule_id == "skO_contamination")
            {
                risk_scalar.knowledge = rule.weight;
            }
        }

        Ok(())
    }

    /// Determines the defensive action based on risk scalar and thresholds.
    fn determine_action(
        &self,
        risk_scalar: &SabotageRiskScalar,
    ) -> SabotageDetectorResult<SabotageAction> {
        let shard = self
            .blacklist_shard
            .as_ref()
            .ok_or_else(|| SabotageDetectorError::AlnLoadError("Shard not loaded".into()))?;

        // Check critical thresholds first
        if risk_scalar.integrity >= 1.0 {
            return Ok(SabotageAction::HardFail);
        }

        if risk_scalar.total >= 0.85 {
            return Ok(SabotageAction::UnsafeDefer);
        }

        if risk_scalar.total >= 0.70 {
            return Ok(SabotageAction::Deny);
        }

        if risk_scalar.biophysical >= 0.60 {
            return Ok(SabotageAction::Inhibit);
        }

        if risk_scalar.access >= 0.75 {
            return Ok(SabotageAction::Deny);
        }

        Ok(SabotageAction::LogOnly)
    }

    /// Generates and logs a SABOTAGEEVENT.
    ///
    /// # Arguments
    /// * `risk_scalar` — Computed sabotage risk scalar
    /// * `action` — Defensive action taken
    /// * `proposal` — The triggering evolution proposal
    ///
    /// # Returns
    /// * `SabotageDetectorResult<SabotageEvent>` — Generated event or error
    pub fn generate_sabotage_event(
        &mut self,
        risk_scalar: &SabotageRiskScalar,
        action: SabotageAction,
        proposal: &EvolutionProposal,
    ) -> SabotageDetectorResult<SabotageEvent> {
        self.event_id_counter += 1;

        let shard = self
            .blacklist_shard
            .as_ref()
            .ok_or_else(|| SabotageDetectorError::AlnLoadError("Shard not loaded".into()))?;

        // Determine severity
        let severity = match action {
            SabotageAction::LogOnly => SabotageEventSeverity::Low,
            SabotageAction::Deny | SabotageAction::Inhibit => SabotageEventSeverity::Medium,
            SabotageAction::UnsafeDefer => SabotageEventSeverity::High,
            SabotageAction::HardFail => SabotageEventSeverity::Critical,
        };

        // Collect jurisdiction tags from neurorights bindings
        let mut jurisdiction_tags = Vec::new();
        let mut neurorights_violations = Vec::new();

        for binding in &shard.neurorights_bindings {
            jurisdiction_tags.push(binding.jurisdiction_id.clone());

            // Map risk factors to neurorights violations
            if risk_scalar.biophysical > 0.5 && binding.right_type == "mental_integrity" {
                neurorights_violations.push(NeurorightsViolation::MentalIntegrity);
            }
            if risk_scalar.provenance > 0.5 && binding.right_type == "cognitive_liberty" {
                neurorights_violations.push(NeurorightsViolation::CognitiveLiberty);
            }
            if risk_scalar.knowledge > 0.3 && binding.right_type == "neural_data_privacy" {
                neurorights_violations.push(NeurorightsViolation::NeuralDataPrivacy);
            }
        }

        // Generate host DID signature
        let signature = self
            .host_did_keypair
            .sign(&proposal.identifier)
            .map_err(|e| SabotageDetectorError::SignatureError(e.to_string()))?;

        // Create the event
        let mut event = SabotageEvent {
            event_id: self.event_id_counter,
            timestamp_utc: Utc::now(),
            brainidentity_hash: self.brainidentity_hash.clone(),
            sabotage_risk_scalar: risk_scalar.total,
            provenance_factor: risk_scalar.provenance,
            blacklist_factor: risk_scalar.blacklist,
            biophysical_factor: risk_scalar.biophysical,
            integrity_factor: risk_scalar.integrity,
            access_factor: risk_scalar.access,
            knowledge_factor: risk_scalar.knowledge,
            triggering_artifact_hash: proposal.compute_hash(),
            jurisdiction_tags,
            neurorights_violations,
            host_did_signature: signature,
            googolswarm_anchor_txid: None,
            severity,
            action_taken: action,
        };

        // Write to QPU.Datashard
        self.write_to_datashard(&event)?;

        // Anchor to Googolswarm
        let anchor_txid = self.anchor_to_googolswarm(&event)?;
        event.googolswarm_anchor_txid = Some(anchor_txid);

        // Update event with anchor
        self.update_event_with_anchor(&event)?;

        // Log to DonutLoop
        self.log_to_donutloop(&event)?;

        Ok(event)
    }

    /// Checks if a DID is in the trusted list.
    fn is_trusted_did(&self, did: &str) -> bool {
        // Check against host DID and OrganicCPU runtime DID
        did == self.host_did_keypair.did()
            || did == "did:organiccpu:runtime:0xB05TR0M...50VERE1GN"
    }

    /// Checks if an identifier matches a blacklist pattern.
    fn matches_pattern(&self, identifier: &str, pattern: &BlacklistPattern) -> bool {
        match pattern.op.as_str() {
            "eq" => identifier == pattern.pattern,
            "prefix" => identifier.starts_with(&pattern.pattern),
            "glob" => self.glob_match(identifier, &pattern.pattern),
            "regex" => self.regex_match(identifier, &pattern.pattern),
            _ => false,
        }
    }

    /// Simple glob pattern matching.
    fn glob_match(&self, text: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }
        if pattern.starts_with("*.") {
            let ext = &pattern[1..];
            return text.ends_with(ext);
        }
        text == pattern
    }

    /// Simple regex pattern matching.
    fn regex_match(&self, text: &str, pattern: &str) -> bool {
        // Simplified regex matching for common patterns
        if pattern.ends_with('*') {
            let prefix = &pattern[..pattern.len() - 1];
            return text.starts_with(prefix);
        }
        text.contains(pattern)
    }

    /// Logs a pattern match to the audit trail.
    fn log_pattern_match(
        &self,
        pattern: &BlacklistPattern,
        identifier: &str,
    ) -> SabotageDetectorResult<()> {
        self.donutloop_logger
            .log_entry(
                "blacklist_pattern_match",
                &serde_json::json!({
                    "pattern": pattern.pattern,
                    "kind": pattern.kind,
                    "reason": pattern.reason,
                    "matched_identifier": identifier,
                    "timestamp": Utc::now().to_rfc3339(),
                }),
            )
            .map_err(|e| SabotageDetectorError::PatternMatchError(e.to_string()))?;

        Ok(())
    }

    /// Writes the event to QPU.Datashard.
    fn write_to_datashard(&self, event: &SabotageEvent) -> SabotageDetectorResult<()> {
        let record = QpuEventRecord::from_sabotage_event(event);

        self.qpu_datashard
            .append_record("SABOTAGEEVENT", &record)
            .map_err(|e| SabotageDetectorError::DatashardWriteError(e.to_string()))?;

        Ok(())
    }

    /// Anchors the event to Googolswarm blockchain.
    fn anchor_to_googolswarm(&self, event: &SabotageEvent) -> SabotageDetectorResult<String> {
        let txid = self
            .googolswarm_client
            .anchor_event(event)
            .map_err(|e| SabotageDetectorError::GoogolswarmAnchorError(e.to_string()))?;

        Ok(txid)
    }

    /// Updates the event record with the anchor transaction ID.
    fn update_event_with_anchor(&self, event: &SabotageEvent) -> SabotageDetectorResult<()> {
        self.qpu_datashard
            .update_record_anchor(event.event_id, event.googolswarm_anchor_txid.as_ref().unwrap())
            .map_err(|e| SabotageDetectorError::DatashardWriteError(e.to_string()))?;

        Ok(())
    }

    /// Logs the event to DonutLoop audit trail.
    fn log_to_donutloop(&self, event: &SabotageEvent) -> SabotageDetectorResult<()> {
        self.donutloop_logger
            .log_entry(
                "SABOTAGEEVENT",
                &serde_json::json!({
                    "event_id": event.event_id,
                    "timestamp": event.timestamp_utc.to_rfc3339(),
                    "sabotage_risk": event.sabotage_risk_scalar,
                    "severity": format!("{:?}", event.severity),
                    "action": format!("{:?}", event.action_taken),
                    "neurorights_violations": event.neurorights_violations.iter().map(|v| format!("{:?}", v)).collect::<Vec<_>>(),
                    "googolswarm_anchor": event.googolswarm_anchor_txid,
                }),
            )
            .map_err(|e| SabotageDetectorError::EventGenerationError(e.to_string()))?;

        Ok(())
    }

    /// Checks if an evolution proposal should be rejected based on sabotage risk.
    ///
    /// # Arguments
    /// * `proposal` — The evolution proposal to check
    /// * `telemetry` — Current biophysical telemetry
    ///
    /// # Returns
    /// * `SabotageDetectorResult<()>` — Ok if allowed, Error if rejected
    pub fn validate_proposal(
        &mut self,
        proposal: &EvolutionProposal,
        telemetry: &BiophysicalTelemetry,
    ) -> SabotageDetectorResult<()> {
        let (risk_scalar, action) = self.evaluate_proposal(proposal, telemetry)?;

        match action {
            SabotageAction::LogOnly => Ok(()),
            _ => {
                let _event = self.generate_sabotage_event(&risk_scalar, action, proposal)?;

                Err(SabotageDetectorError::EvolutionRejected {
                    risk: risk_scalar.total,
                    threshold: 0.70,
                })
            }
        }
    }
}

// =============================================================================
// UNIT TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        mock_brainidentity, mock_did_keypair, mock_donutloop, mock_googolswarm, mock_qpu_datashard,
    };

    fn create_test_detector() -> SabotageDetector {
        SabotageDetector::new(
            PathBuf::from("test_data/bostrom-blacklist-v1.aln"),
            mock_brainidentity(),
            mock_did_keypair(),
            mock_qpu_datashard(),
            mock_donutloop(),
            mock_googolswarm(),
        )
        .expect("Failed to create test detector")
    }

    #[test]
    fn test_trusted_did_validation() {
        let detector = create_test_detector();
        assert!(detector.is_trusted_did(&detector.host_did_keypair.did()));
        assert!(!detector.is_trusted_did("did:enterprise:untrusted"));
    }

    #[test]
    fn test_blacklist_pattern_matching_eq() {
        let detector = create_test_detector();
        let pattern = BlacklistPattern {
            metric: "blacklist_pattern".into(),
            domain: "host".into(),
            module: "sovereigntycore".into(),
            op: "eq".into(),
            pattern: "QConLocus".into(),
            kind: "crate_id".into(),
            reason: "test".into(),
            source: "test".into(),
        };

        assert!(detector.matches_pattern("QConLocus", &pattern));
        assert!(!detector.matches_pattern("OtherCrate", &pattern));
    }

    #[test]
    fn test_blacklist_pattern_matching_prefix() {
        let detector = create_test_detector();
        let pattern = BlacklistPattern {
            metric: "blacklist_pattern".into(),
            domain: "host".into(),
            module: "organiccpualn".into(),
            op: "prefix".into(),
            pattern: "QConLocus_".into(),
            kind: "symbol_prefix".into(),
            reason: "test".into(),
            source: "test".into(),
        };

        assert!(detector.matches_pattern("QConLocus_orchestrator", &pattern));
        assert!(!detector.matches_pattern("OtherCrate_symbol", &pattern));
    }

    #[test]
    fn test_sabotage_risk_scalar_computation() {
        let mut scalar = SabotageRiskScalar::new();
        scalar.provenance = 0.6;
        scalar.blacklist = 0.3;
        scalar.biophysical = 0.45;

        let total = scalar.compute_total();
        assert!(total >= 0.0 && total <= 1.0);
        assert_eq!(total, scalar.total);
    }

    #[test]
    fn test_risk_clamping() {
        let mut scalar = SabotageRiskScalar::new();
        scalar.provenance = 1.0;
        scalar.blacklist = 1.0;
        scalar.biophysical = 1.0;
        scalar.integrity = 1.0;
        scalar.access = 1.0;
        scalar.knowledge = 1.0;

        let total = scalar.compute_total();
        assert_eq!(total, 1.0); // Should be clamped to max 1.0
    }
}

// =============================================================================
// END OF FILE
// =============================================================================
