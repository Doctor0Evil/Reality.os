use reality_os_core::biophysical::{
    TokenFlow, TokenKind, BiophysicalLoad, ResponsibilityGradient
};
use reality_os_core::justice::{
    DeedKind, Judgment, EthicalRegulator, TreeOfLifeInvariant
};
use reality_os_core::state::{
    SiteState, MicroSociety, JetsonLine
};
use reality_os_core::w_cycle::WReflection;
use reality_os_core::neuromorph::{GODInvariant, ConstraintViolation};

/// Core anger instrumentation schema with traceability
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AngerState {
    /// True when biophysical unfairness exceeds threshold (ERG > 0.7)
    pub anger_eligible: bool,
    /// Timestamp when anger eligibility was first triggered
    pub trigger_time: u64,
    /// Evidence-based metrics that triggered anger state
    pub evidence_metrics: Vec<ResponsibilityGradient>,
    /// Current anger intensity (0.0-1.0, based on FEAR and unresolved unfairness)
    pub intensity: f64,
    /// True if anger is being handled with discipline constraints
    pub disciplined: bool,
    /// True if anger has been coupled to repair actions within k ticks
    pub repair_coupled: bool,
    /// Tracks whether anger led to boundary-setting or counter-harm
    pub outcome_type: AngerOutcome,
    /// References to specific biophysical records that validate the anger trigger
    pub evidence_references: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AngerOutcome {
    BoundarySetting,  // Healthy: establishes safety limits
    CounterHarm,      // Unhealthy: creates new harm
    RepairPathway,    // Healthy: leads to restorative action
    PolicyChange,     // Healthy: triggers system adaptation
    None,             // No outcome yet
}

/// Discipline constraints that must be satisfied for anger to be healthy
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AngerDisciplineConstraints {
    /// Minimum evidence required (references to specific token flows)
    pub min_evidence_count: usize,
    /// Maximum time between anger trigger and repair action
    pub max_ticks_to_repair: u64,
    /// Maximum acceptable escalation before repair coupling
    pub max_escalation: f64,
    /// Minimum proportionality threshold (harm:response ratio)
    pub proportionality_threshold: f64,
}

/// Core instrumentation for anger states in the Jetson-Line
pub fn instrument_anger(
    micro_society: &mut MicroSociety,
    biophysical: &BiophysicalLoad,
    regulator: &EthicalRegulator
) -> Vec<AngerState> {
    let mut anger_states = Vec::new();
    
    for (site_id, site) in micro_society.sites.iter_mut() {
        // Check if site meets anger eligibility conditions
        let is_eligible = check_anger_eligibility(site, biophysical, regulator);
        
        if is_eligible {
            // Create or update anger state
            let anger_state = update_anger_state(
                site, 
                biophysical, 
                regulator,
                micro_society.last_anger_state(*site_id)
            );
            
            // Apply discipline constraints
            apply_discipline_constraints(&anger_state, site, regulator);
            
            // Log for traceability
            regulator.log_anger_state(site_id, &anger_state);
            
            anger_states.push(anger_state);
        }
    }
    
    anger_states
}

/// Check if a site meets the anger eligibility conditions
fn check_anger_eligibility(
    site: &SiteState,
    biophysical: &BiophysicalLoad,
    regulator: &EthicalRegulator
) -> bool {
    // Check for persistent unfairness (ERG > 0.7)
    let exposure_responsibility_gap = calculate_exposure_responsibility_gap(site, biophysical);
    
    // Check for blocked peaceful correction
    let peaceful_channels_blocked = regulator.has_blocked_peaceful_channels(site);
    
    // Check for violation of Church-FAER-POWER-TECH constraints
    let fairness_violation = site.church < regulator.min_justice_threshold() || 
                           site.power > regulator.max_power_ceiling(site.church);
    
    // Anger is eligible when all three conditions hold
    exposure_responsibility_gap > 0.7 && 
    peaceful_channels_blocked &&
    fairness_violation
}

/// Calculate Exposure-Responsibility Gap (ERG) metric
fn calculate_exposure_responsibility_gap(
    site: &SiteState,
    biophysical: &BiophysicalLoad
) -> f64 {
    // Implementation of the ERG metric
    // Higher values indicate greater unfairness
    let exposure = site.habit_load as f64 / biophysical.max_capacity as f64;
    let responsibility = site.power as f64 / site.church.max(1) as f64;
    
    (exposure - responsibility).abs().min(1.0)
}

/// Apply discipline constraints to anger states
fn apply_discipline_constraints(
    anger_state: &AngerState,
    site: &mut SiteState,
    regulator: &EthicalRegulator
) {
    // Ensure anger-triggered deeds reference evidence
    if anger_state.anger_eligible {
        regulator.set_deed_constraint(
            DeedKind::Conflict,
            |deed| {
                deed.references_evidence() && 
                anger_state.evidence_references.contains(&deed.evidence_id)
            }
        );
        
        // Enforce proportionality constraint
        regulator.set_deed_constraint(
            DeedKind::Conflict,
            |deed| {
                let harm = deed.calculate_harm();
                let proportional_max = anger_state.proportionality_threshold * 
                                     anger_state.evidence_metrics.iter().map(|m| m.harm).sum::<f64>();
                harm <= proportional_max
            }
        );
    }
    
    // Monitor repair coupling
    if anger_state.anger_eligible && !anger_state.repair_coupled {
        site.set_repair_incentive(regulator.repair_incentive_multiplier());
    }
}
