use serde::{Deserialize, Serialize};

/// Normalized [0.0, 1.0] impact profile for a candidate OTA/module.
/// All deltas are interpreted as *worsening* if positive.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ImpactProfile {
    /// Expected increase in modeled overuse / injury risk (0.0–1.0).
    pub delta_risk: f32,
    /// Expected increase in neuromotor duty-cycle fraction (0.0–1.0).
    pub delta_duty_cycle: f32,
    /// Expected increase in normalized fatigue index (0.0–1.0).
    pub delta_fatigue: f32,
    /// Expected *worsening* of EcoImpactScore (0.0–1.0).
    /// Positive values mean more device-hours / energy, worse eco profile. [file:6]
    pub delta_eco: f32,
}

/// High-level threat class to support policy routing and auditing.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatClass {
    /// Attempts to override consent corridors, privacy, or neurorights envelopes. [file:6]
    NeurorightsBreach,
    /// Increases duty-cycle, fatigue, or risk beyond allowed envelopes. [file:5]
    OverloadEnvelope,
    /// Worsens EcoImpactScore or increases device-hours beyond eco corridors. [file:6]
    EcoRegression,
    /// Attempts to bypass payment / commerce guards for outlawed categories. [file:2]
    IllicitCommerceBridge,
    /// Introduces invasive actuation or schema fields that bridge directly to biology. [file:4]
    SchemaActuationLeak,
}

/// Canonical VIRUS SIGNATURE object: a typed, auditable envelope for forbidden patterns.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VirusSignature {
    /// Stable, unique ID for referencing and audit trails.
    pub sig_id: String,
    /// Human-readable rationale and threat description for reviewers. [file:5]
    pub description: String,
    /// Threat class, used for policy routing and escalation.
    pub class: ThreatClass,
    /// ALN field names that must *never* appear in a valid schema (e.g., "torque"). [file:4][file:6]
    pub forbidden_fields: Vec<String>,
    /// Crate/module name prefixes considered unsafe (e.g., "unsafe_neuro"). [file:5]
    pub forbidden_modules: Vec<String>,
    /// Maximum allowed deltas in normalized impact space; any larger delta is viral. [file:5][file:6]
    pub impact_thresholds: ImpactProfile,
    /// ID of the ALN policy / jurisdiction particle governing this signature. [file:2][file:6]
    pub jurisdiction_particle: String,
}

/// Result from evaluating one candidate update against a set of virus signatures.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VirusCheckResult {
    pub blocked: bool,
    /// List of sig_ids that were violated (for logs, CI, and human review).
    pub violated_signatures: Vec<String>,
}

fn module_matches_prefixes(module_name: &str, prefixes: &[String]) -> bool {
    prefixes.iter().any(|p| module_name.starts_with(p))
}

fn any_forbidden_field_present(present_fields: &[&str], forbidden: &[String]) -> bool {
    present_fields.iter().any(|f| forbidden.iter().any(|ff| ff == f))
}

fn impact_exceeds_thresholds(impact: &ImpactProfile, thresholds: &ImpactProfile) -> bool {
    // Conservative interpretation: any delta >= threshold is considered a violation. [file:5]
    (thresholds.delta_risk > 0.0 && impact.delta_risk >= thresholds.delta_risk)
        || (thresholds.delta_duty_cycle > 0.0 && impact.delta_duty_cycle >= thresholds.delta_duty_cycle)
        || (thresholds.delta_fatigue > 0.0 && impact.delta_fatigue >= thresholds.delta_fatigue)
        || (thresholds.delta_eco > 0.0 && impact.delta_eco >= thresholds.delta_eco)
}

/// Core evaluation function to be called from CI and runtime pre-OTA checks. [file:5]
pub fn evaluate_against_signatures(
    impact: &ImpactProfile,
    module_name: &str,
    present_fields: &[&str],
    signatures: &[VirusSignature],
) -> VirusCheckResult {
    let mut violated = Vec::new();

    for sig in signatures {
        let blocked_by_module =
            !sig.forbidden_modules.is_empty()
                && module_matches_prefixes(module_name, &sig.forbidden_modules);

        let blocked_by_fields =
            !sig.forbidden_fields.is_empty()
                && any_forbidden_field_present(present_fields, &sig.forbidden_fields);

        let blocked_by_impact = impact_exceeds_thresholds(impact, &sig.impact_thresholds);

        if blocked_by_module || blocked_by_fields || blocked_by_impact {
            violated.push(sig.sig_id.clone());
        }
    }

    VirusCheckResult {
        blocked: !violated.is_empty(),
        violated_signatures: violated,
    }
}
