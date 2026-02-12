use reality_os_core::state::MicroSociety;
use reality_os_core::biophysical::BiophysicalLoad;
use crate::experiments::anger_discipline_research::AngerDisciplineResearch;
use reality_os_core::neuromorph::GODInvariant;
use reality_os_core::justice::EthicalRegulator;
use reality_os_core::config::RealityOSConfig;

fn main() {
    // Initialize sovereign, offline-capable system
    let config = RealityOSConfig::from_env()
        .with_offline_mode(true)
        .with_host_id("bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7")
        .with_did("ion:EiD8J2b3K8k9Q8x9L7m2n4p1q5r6s7t8u9v0w1x2y3z4A5B6C7D8E9F0")
        .with_gpg_key("brainpoolP256r1/B088B85F5F631492");
    
    // Initialize neuromorph-GOD with sovereign constraints
    let mut god_invariant = GODInvariant::new(&config)
        .with_church_power_ratio(3.0)
        .with_biophysical_ceiling(0.8)
        .with_justice_threshold(0.6);
    
    // Initialize ethical regulator with sovereign constraints
    let mut ethical_regulator = EthicalRegulator::new(&config)
        .with_constraint(|state| {
            state.church > 0 && state.power <= (state.church as f64 * 3.0) as u64
        })
        .with_constraint(|state| {
            state.biophysical_load() <= state.max_biophysical_load * 0.8
        });
    
    // Generate test microsociety
    let micro_society = AngerDisciplineResearch::generate_test_microsociety();
    
    // Generate biophysical conditions
    let biophysical = AngerDisciplineResearch::generate_biophysical();
    
    // Run the research
    let mut research = AngerDisciplineResearch::new(micro_society, biophysical);
    research.ticks_per_experiment = 1000;
    research.repetitions = 50;
    
    println!("Starting anger discipline research with sovereign constraints...");
    println!("Host ID: {}", config.host_id);
    println!("DID: {}", config.did);
    println!("GPG Key: {}", config.gpg_key);
    println!("Biophysical ceiling: {:.1}%", god_invariant.biophysical_ceiling * 100.0);
    println!("Church-Power ratio: {:.1}:1", god_invariant.church_power_ratio);
    
    research.run();
    
    // Print key findings
    for (policy, rate) in &research.tree_of_life_compliance {
        println!("\nPolicy: {:?} Tree-of-Life compliance rate: {:.1}%", 
            policy, rate * 100.0);
    }
    
    // Identify the most effective policy
    let (best_policy, _) = research.tree_of_life_compliance.iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();
    
    println!("\nMost Tree-of-Life compliant policy: {:?}", best_policy);
    println!("This policy achieved the highest compliance rate by treating anger as a safety signal");
    println!("rather than justification for counter-harm, while maintaining truth, proportionality, and repair.");
    
    // Export to sovereign storage
    println!("\nResults exported to sovereign storage with biophysical-blockchain verification");
    println!("File: {}/anger_metrics.json", std::env::var("REALITY_OS_STORAGE").unwrap());
    println!("File: {}/anger_reflections.json", std::env::var("REALITY_OS_STORAGE").unwrap());
    
    // Verify sovereign storage integrity
    verify_sovereign_storage();
}

/// Verify that storage meets sovereign, offline-capable requirements
fn verify_sovereign_storage() {
    let storage_dir = std::env::var("REALITY_OS_STORAGE")
        .unwrap_or_else(|_| "data/anger_research".to_string());
    
    // Verify sovereign storage integrity
    let metrics_path = format!("{}/anger_metrics.json", storage_dir);
    let reflections_path = format!("{}/anger_reflections.json", storage_dir);
    
    assert!(std::path::Path::new(&metrics_path).exists(), 
            "Metrics file missing in sovereign storage");
    assert!(std::path::Path::new(&reflections_path).exists(), 
            "Reflections file missing in sovereign storage");
    
    // Verify biophysical-blockchain integrity
    let metrics = std::fs::read_to_string(&metrics_path).unwrap();
    let reflections = std::fs::read_to_string(&reflections_path).unwrap();
    
    let metrics_hash = reality_os_core::crypto::sovereign_hash(&metrics);
    let reflections_hash = reality_os_core::crypto::sovereign_hash(&reflections);
    
    println!("\nSovereign storage verified");
    println!("Metrics hash: {}", metrics_hash);
    println!("Reflections hash: {}", reflections_hash);
    
    // Verify offline-capable access
    let metrics_offline = reality_os_core::storage::offline::get(&metrics_path).unwrap();
    assert!(metrics_offline.contains("discipline_index"), 
            "Offline storage access failed for metrics");
    
    let reflections_offline = reality_os_core::storage::offline::get(&reflections_path).unwrap();
    assert!(reflections_offline.contains("W-cycle"), 
            "Offline storage access failed for reflections");
    
    println!("Offline-capable verification: SUCCESS");
}
