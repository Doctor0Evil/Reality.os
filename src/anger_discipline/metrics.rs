use crate::anger_discipline::{AngerState, AngerOutcome};

/// Metrics for evaluating anger discipline
pub struct AngerDisciplineMetrics {
    /// Fraction of anger-eligible states that were disciplined
    pub discipline_index: f64,
    /// Fraction of anger-eligible states that led to counter-harm
    pub counter_harm_index: f64,
    /// Average time between anger trigger and repair coupling
    pub repair_coupling_time: f64,
    /// Fraction of anger-triggered deeds that referenced evidence
    pub evidence_utilization: f64,
    /// Fraction of anger-triggered deeds that were proportional
    pub proportionality_compliance: f64,
    /// Fraction of anger-triggered deeds that led to policy changes
    pub policy_change_rate: f64,
}

impl AngerDisciplineMetrics {
    /// Calculate metrics from a set of anger states
    pub fn from_anger_states(anger_states: &[AngerState]) -> Self {
        let total = anger_states.len() as f64;
        if total == 0.0 {
            return Self::default();
        }
        
        let disciplined_count = anger_states.iter()
            .filter(|a| a.disciplined)
            .count() as f64;
        
        let counter_harm_count = anger_states.iter()
            .filter(|a| matches!(a.outcome_type, AngerOutcome::CounterHarm))
            .count() as f64;
        
        let mut repair_times = Vec::new();
        let mut evidence_count = 0;
        let mut proportional_count = 0;
        let mut policy_changes = 0;
        
        for state in anger_states {
            if state.disciplined && state.repair_coupled {
                // Calculate repair coupling time
                if let Some(trigger_time) = anger_states.iter()
                    .find(|s| s.evidence_references == state.evidence_references) 
                    .map(|s| s.trigger_time)
                {
                    repair_times.push(state.trigger_time - trigger_time);
                }
            }
            
            evidence_count += state.evidence_references.len();
            proportional_count += if state.intensity <= state.max_escalation { 1 } else { 0 };
            
            if matches!(state.outcome_type, AngerOutcome::PolicyChange) {
                policy_changes += 1;
            }
        }
        
        let avg_repair_time = if !repair_times.is_empty() {
            repair_times.iter().sum::<u64>() as f64 / repair_times.len() as f64
        } else {
            0.0
        };
        
        Self {
            discipline_index: disciplined_count / total,
            counter_harm_index: counter_harm_count / total,
            repair_coupling_time: avg_repair_time,
            evidence_utilization: if total > 0.0 { evidence_count as f64 / total } else { 0.0 },
            proportionality_compliance: proportional_count as f64 / total,
            policy_change_rate: policy_changes as f64 / total,
        }
    }
    
    /// Determine if anger discipline meets Tree-of-Life safety thresholds
    pub fn is_tree_of_life_compliant(&self) -> bool {
        self.discipline_index > 0.7 &&
        self.counter_harm_index < 0.3 &&
        self.repair_coupling_time < 100.0
    }
}

impl Default for AngerDisciplineMetrics {
    fn default() -> Self {
        Self {
            discipline_index: 0.0,
            counter_harm_index: 0.0,
            repair_coupling_time: f64::INFINITY,
            evidence_utilization: 0.0,
            proportionality_compliance: 0.0,
            policy_change_rate: 0.0,
        }
    }
}
