use crate::anger_discipline::{AngerState, AngerDisciplineConstraints, AngerOutcome};
use reality_os_core::biophysical::ResponsibilityGradient;
use reality_os_core::state::SiteState;
use reality_os_core::justice::DeedKind;
use reality_os_core::neuromorph::GODInvariant;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AngerPolicy {
    Suppressed,
    Undisciplined,
    Disciplined(AngerDisciplineConstraints),
    TreeOfLifeAligned,
}

impl AngerPolicy {
    /// Apply policy to a microsociety state
    pub fn apply_to(
        &self,
        site: &mut SiteState,
        anger_state: &mut AngerState,
        regulator: &mut GODInvariant
    ) {
        match self {
            AngerPolicy::Suppressed => {
                // Tighten FEAR band, prevent conflict deeds
                site.max_anger_intensity = 0.3;
                site.anger_suppression_factor = 0.8;
                regulator.add_constraint(|state| {
                    !matches!(state.current_deed, DeedKind::Conflict | DeedKind::Sanction)
                });
            },
            AngerPolicy::Undisciplined => {
                // Lower FEAR thresholds for conflict
                site.max_anger_intensity = 1.0;
                site.anger_suppression_factor = 0.0;
                regulator.add_constraint(|state| {
                    state.anger_intensity > 0.5
                });
            },
            AngerPolicy::Disciplined(constraints) => {
                // Apply discipline constraints
                anger_state.disciplined = true;
                anger_state.max_escalation = constraints.max_escalation;
                anger_state.proportionality_threshold = constraints.proportionality_threshold;
                
                // Add evidence requirement
                if anger_state.evidence_metrics.len() < constraints.min_evidence_count {
                    anger_state.disciplined = false;
                }
                
                // Add repair coupling requirement
                if anger_state.trigger_time > 0 && 
                   site.current_tick - anger_state.trigger_time > constraints.max_ticks_to_repair {
                    anger_state.repair_coupled = false;
                }
            },
            AngerPolicy::TreeOfLifeAligned => {
                // Tree-of-Life specific constraints
                site.max_anger_intensity = 0.7;
                site.anger_suppression_factor = 0.3;
                
                // Only allow anger-triggered deeds that reference evidence
                regulator.add_constraint(|state| {
                    state.current_deed.references_evidence() && 
                    state.anger_state.is_some()
                });
                
                // Require repair deeds within specific timeframe
                regulator.add_constraint(|state| {
                    if let Some(anger) = &state.anger_state {
                        if anger.anger_eligible && !anger.repair_coupled {
                            state.current_tick - anger.trigger_time <= 50
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                });
            }
        }
    }
    
    /// Evaluate policy effectiveness against Tree-of-Life metrics
    pub fn evaluate_effectiveness(
        &self,
        metrics: &AngerDisciplineMetrics,
        biophysical: &ResponsibilityGradient
    ) -> f64 {
        match self {
            AngerPolicy::Suppressed => {
                // Penalize for unresolved unfairness accumulation
                metrics.discipline_index * (1.0 - biophysical.unresolved_unfairness)
            },
            AngerPolicy::Undisciplined => {
                // High penalty for counter-harm
                metrics.discipline_index - (metrics.counter_harm_index * 2.0)
            },
            _ => {
                // Disciplined and TreeOfLifeAligned
                metrics.discipline_index * 0.7 +
                (1.0 - metrics.counter_harm_index) * 0.2 +
                metrics.policy_change_rate * 0.1
            }
        }
    }
}
