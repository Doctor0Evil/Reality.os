use std::fs::File;
use std::io::Write;
use reality_os_core::state::{MicroSociety, SiteState};
use reality_os_core::biophysical::{BiophysicalLoad, ResponsibilityGradient};
use reality_os_core::justice::DeedKind;
use crate::anger_discipline::{AngerPolicy, AngerDisciplineMetrics};
use crate::experiments::AngerDisciplineExperiment;

pub struct AngerDisciplineResearch {
    /// Baseline microsociety for testing
    pub base_micro_society: MicroSociety,
    /// Baseline biophysical parameters
    pub base_biophysical: BiophysicalLoad,
    /// Number of ticks per experiment
    pub ticks_per_experiment: u64,
    /// Number of repetitions per condition
    pub repetitions: u32,
    /// Results for each policy
    pub policy_results: Vec<(AngerPolicy, Vec<AngerDisciplineMetrics>)>,
    /// Tree-of-Life compliance rates
    pub tree_of_life_compliance: Vec<(AngerPolicy, f64)>,
    /// W-cycle reflection archives
    pub reflection_archives: Vec<(AngerPolicy, Vec<String>)>,
}

impl AngerDisciplineResearch {
    pub fn new(base_micro_society: MicroSociety, base_biophysical: BiophysicalLoad) -> Self {
        Self {
            base_micro_society,
            base_biophysical,
            ticks_per_experiment: 500,
            repetitions: 30,
            policy_results: Vec::new(),
            tree_of_life_compliance: Vec::new(),
            reflection_archives: Vec::new(),
        }
    }
    
    /// Run the full research protocol
    pub fn run(&mut self) {
        let policies = vec![
            AngerPolicy::Suppressed,
            AngerPolicy::Undisciplined,
            AngerPolicy::Disciplined(self.disciplined_constraints()),
            AngerPolicy::TreeOfLifeAligned,
        ];
        
        for policy in policies {
            let mut results = Vec::with_capacity(self.repetitions as usize);
            let mut compliance_count = 0;
            let mut reflections = Vec::new();
            
            for _ in 0..self.repetitions {
                let mut experiment = AngerDisciplineExperiment::new(
                    self.base_micro_society.clone(),
                    self.base_biophysical.clone(),
                    policy.clone()
                );
                
                experiment.run(self.ticks_per_experiment);
                
                results.push(experiment.metrics.clone());
                
                if experiment.metrics.is_tree_of_life_compliant() {
                    compliance_count += 1;
                }
                
                for reflection in experiment.w_reflections {
                    reflections.push(format!("{:?}: {}", policy, reflection));
                }
            }
            
            self.policy_results.push((policy.clone(), results));
            self.tree_of_life_compliance.push((policy.clone(), compliance_count as f64 / self.repetitions as f64));
            self.reflection_archives.push((policy, reflections));
        }
        
        self.save_results();
    }
    
    /// Generate disciplined anger constraints for Tree-of-Life contexts
    fn disciplined_constraints(&self) -> AngerDisciplineConstraints {
        AngerDisciplineConstraints {
            min_evidence_count: 3,
            max_ticks_to_repair: 75,
            max_escalation: 0.8,
            proportionality_threshold: 0.9,
        }
    }
    
    /// Save results to sovereign storage
    fn save_results(&self) {
        // Create sovereign storage directory if not exists
        let storage_dir = std::env::var("REALITY_OS_STORAGE")
            .unwrap_or_else(|_| "data/anger_research".to_string());
        
        std::fs::create_dir_all(&storage_dir).unwrap();
        
        // Save metrics
        let metrics_path = format!("{}/anger_metrics.json", storage_dir);
        let mut metrics_file = File::create(metrics_path).unwrap();
        
        writeln!(metrics_file, "{{").unwrap();
        writeln!(metrics_file, "  \"timestamp\": \"{}\",", chrono::Utc::now().to_rfc3339()).unwrap();
        writeln!(metrics_file, "  \"results\": [").unwrap();
        
        for (i, (policy, metrics)) in self.policy_results.iter().enumerate() {
            writeln!(metrics_file, "    {{").unwrap();
            writeln!(metrics_file, "      \"policy\": \"{:?}\",", policy).unwrap();
            writeln!(metrics_file, "      \"metrics\": [").unwrap();
            
            for (j, metric) in metrics.iter().enumerate() {
                writeln!(metrics_file, "        {{").unwrap();
                writeln!(metrics_file, "          \"discipline_index\": {:.4},", metric.discipline_index).unwrap();
                writeln!(metrics_file, "          \"counter_harm_index\": {:.4},", metric.counter_harm_index).unwrap();
                writeln!(metrics_file, "          \"repair_coupling_time\": {:.4},", metric.repair_coupling_time).unwrap();
                writeln!(metrics_file, "          \"evidence_utilization\": {:.4},", metric.evidence_utilization).unwrap();
                writeln!(metrics_file, "          \"proportionality_compliance\": {:.4},", metric.proportionality_compliance).unwrap();
                writeln!(metrics_file, "          \"policy_change_rate\": {:.4}", metric.policy_change_rate).unwrap();
                writeln!(metrics_file, "        }}{}", if j < metrics.len()-1 { "," } else { "" }).unwrap();
            }
            
            writeln!(metrics_file, "      ]").unwrap();
            writeln!(metrics_file, "    }}{}", if i < self.policy_results.len()-1 { "," } else { "" }).unwrap();
        }
        
        writeln!(metrics_file, "  ],").unwrap();
        writeln!(metrics_file, "  \"compliance\": [").unwrap();
        
        for (i, (policy, rate)) in self.tree_of_life_compliance.iter().enumerate() {
            writeln!(metrics_file, "    {{").unwrap();
            writeln!(metrics_file, "      \"policy\": \"{:?}\",", policy).unwrap();
            writeln!(metrics_file, "      \"compliance_rate\": {:.4}", rate).unwrap();
            writeln!(metrics_file, "    }}{}", if i < self.tree_of_life_compliance.len()-1 { "," } else { "" }).unwrap();
        }
        
        writeln!(metrics_file, "  ]").unwrap();
        writeln!(metrics_file, "}}").unwrap();
        
        // Save reflections
        let reflections_path = format!("{}/anger_reflections.json", storage_dir);
        let mut reflections_file = File::create(reflections_path).unwrap();
        
        writeln!(reflections_file, "{{").unwrap();
        writeln!(reflections_file, "  \"timestamp\": \"{}\",", chrono::Utc::now().to_rfc3339()).unwrap();
        writeln!(reflections_file, "  \"reflections\": [").unwrap();
        
        for (i, (policy, reflections)) in self.reflection_archives.iter().enumerate() {
            writeln!(reflections_file, "    {{").unwrap();
            writeln!(reflections_file, "      \"policy\": \"{:?}\",", policy).unwrap();
            writeln!(reflections_file, "      \"reflections\": [").unwrap();
            
            for (j, reflection) in reflections.iter().enumerate() {
                writeln!(reflections_file, "        \"{}\"{}", 
                    reflection.replace("\"", "\\\""), 
                    if j < reflections.len()-1 { "," } else { "" }).unwrap();
            }
            
            writeln!(reflections_file, "      ]").unwrap();
            writeln!(reflections_file, "    }}{}", if i < self.reflection_archives.len()-1 { "," } else { "" }).unwrap();
        }
        
        writeln!(reflections_file, "  ]").unwrap();
        writeln!(reflections_file, "}}").unwrap();
    }
    
    /// Generate a test microsociety with controlled unfairness
    pub fn generate_test_microsociety() -> MicroSociety {
        let mut society = MicroSociety::new(100);
        
        // Create unfairness conditions
        for i in 0..50 {
            let mut site = SiteState::new(i);
            
            // Set up unfairness (high exposure, low responsibility)
            site.habit_load = 80;
            site.pollution = 70;
            site.church = 20;
            site.power = 90;
            site.technical_capacity = 95;
            
            society.sites.insert(i, site);
        }
        
        // Set up fair sites for comparison
        for i in 50..100 {
            let mut site = SiteState::new(i);
            
            site.habit_load = 30;
            site.pollution = 25;
            site.church = 80;
            site.power = 40;
            site.technical_capacity = 70;
            
            society.sites.insert(i, site);
        }
        
        society
    }
    
    /// Generate biophysical conditions for anger testing
    pub fn generate_biophysical() -> BiophysicalLoad {
        BiophysicalLoad {
            max_capacity: 100,
            current_load: 50,
            responsibility_gradient: ResponsibilityGradient {
                exposure_responsibility_gap: 0.75,
                unresolved_unfairness: 0.6,
                power_concentration: 0.8,
            },
            safety_margins: vec![0.2, 0.3, 0.15, 0.1],
            trust_threshold: 0.4,
        }
    }
}
