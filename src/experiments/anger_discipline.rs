use std::time::Instant;
use reality_os_core::biophysical::{TokenFlow, BiophysicalLoad};
use reality_os_core::justice::{DeedKind, Judgment};
use reality_os_core::state::{MicroSociety, JetsonLine, SiteState};
use crate::anger_discipline::{AngerPolicy, AngerState, AngerDisciplineMetrics};

pub struct AngerDisciplineExperiment {
    /// The microsociety being tested
    pub micro_society: MicroSociety,
    /// Biophysical load parameters
    pub biophysical: BiophysicalLoad,
    /// Policy being tested
    pub policy: AngerPolicy,
    /// Historical anger states
    pub anger_history: Vec<AngerState>,
    /// Metrics for evaluation
    pub metrics: AngerDisciplineMetrics,
    /// Start time of the experiment
    pub start_time: Instant,
    /// Current tick count
    pub tick_count: u64,
    /// W-cycle reflections on anger handling
    pub w_reflections: Vec<WReflection>,
}

impl AngerDisciplineExperiment {
    pub fn new(micro_society: MicroSociety, biophysical: BiophysicalLoad, policy: AngerPolicy) -> Self {
        Self {
            micro_society,
            biophysical,
            policy,
            anger_history: Vec::new(),
            metrics: AngerDisciplineMetrics::default(),
            start_time: Instant::now(),
            tick_count: 0,
            w_reflections: Vec::new(),
        }
    }
    
    /// Run the experiment for a specified number of ticks
    pub fn run(&mut self, ticks: u64) -> &Self {
        let original_state = self.micro_society.clone();
        
        for _ in 0..ticks {
            // Update microsociety state
            self.micro_society.tick(&self.biophysical);
            
            // Instrument anger states
            let anger_states = self.instrument_anger_states();
            
            // Apply policy to anger states
            self.apply_policy(&anger_states);
            
            // Record W-cycle reflections
            self.record_reflections(&anger_states);
            
            self.tick_count += 1;
        }
        
        // Calculate metrics
        self.metrics = AngerDisciplineMetrics::from_anger_states(&self.anger_history);
        
        // Check for Tree-of-Life compliance
        let is_compliant = self.metrics.is_tree_of_life_compliant();
        self.w_reflections.push(WReflection::new(
            format!("Tree-of-Life compliance: {}", is_compliant),
            "Final compliance check".to_string(),
            if is_compliant { "Anger discipline maintained Tree-of-Life constraints" } 
            else { "Anger discipline violated Tree-of-Life constraints" }.to_string(),
            self.tick_count
        ));
        
        // Log results for biophysical-blockchain
        self.log_results(original_state);
        
        self
    }
    
    /// Instrument anger states in the current microsociety
    fn instrument_anger_states(&mut self) -> Vec<AngerState> {
        let anger_states = crate::anger_discipline::instrument_anger(
            &mut self.micro_society,
            &self.biophysical,
            &self.micro_society.ethical_regulator
        );
        
        self.anger_history.extend(anger_states.clone());
        anger_states
    }
    
    /// Apply the selected anger policy
    fn apply_policy(&mut self, anger_states: &[AngerState]) {
        for state in anger_states {
            self.policy.apply_to(
                self.micro_society.get_mut_site(state.site_id),
                &mut self.anger_history.last_mut().unwrap().clone(),
                &mut self.micro_society.ethical_regulator.god_invariants
            );
        }
    }
    
    /// Record W-cycle reflections for anger states
    fn record_reflections(&mut self, anger_states: &[AngerState]) {
        // Individual "What?"
        for state in anger_states {
            self.w_reflections.push(WReflection::new(
                format!("Anger triggered at site {}", state.site_id),
                "What?".to_string(),
                format!("ERG: {:.2}, Intensity: {:.2}, Evidence count: {}", 
                    state.calculate_erg(), state.intensity, state.evidence_references.len()),
                self.tick_count
            ));
        }
        
        // Group "So What?"
        if self.tick_count % 100 == 0 && !anger_states.is_empty() {
            let avg_erg = anger_states.iter().map(|s| s.calculate_erg()).sum::<f64>() / 
                         anger_states.len() as f64;
            
            self.w_reflections.push(WReflection::new(
                "Group anger analysis".to_string(),
                "So What?".to_string(),
                format!("Average ERG: {:.2}, {:.1}% disciplined, {:.1}% repair-coupled",
                    avg_erg,
                    self.metrics.discipline_index * 100.0,
                    self.metrics.evidence_utilization * 100.0),
                self.tick_count
            ));
        }
    }
    
    /// Log results to biophysical-blockchain
    fn log_results(&self, original_state: MicroSociety) {
        let mut log = self.micro_society.create_log_entry("anger_discipline");
        
        // Record key metrics
        log.add_metric("discipline_index", self.metrics.discipline_index);
        log.add_metric("counter_harm_index", self.metrics.counter_harm_index);
        log.add_metric("repair_coupling_time", self.metrics.repair_coupling_time);
        
        // Record policy effectiveness
        let effectiveness = self.policy.evaluate_effectiveness(
            &self.metrics, 
            &self.biophysical.responsibility_gradient
        );
        log.add_metric("policy_effectiveness", effectiveness);
        
        // Record W-cycle reflections
        for reflection in &self.w_reflections {
            log.add_reflection(reflection);
        }
        
        // Record biophysical load changes
        let original_load = original_state.calculate_biophysical_load();
        let current_load = self.micro_society.calculate_biophysical_load();
        log.add_metric("biophysical_load_delta", current_load - original_load);
        
        // Commit to sovereign blockchain
        self.micro_society.commit_log(log);
    }
}
