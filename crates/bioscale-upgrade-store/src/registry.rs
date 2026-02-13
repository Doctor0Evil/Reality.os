pub fn motor_cortex_stabilizer_v1() -> DefaultsTuple {
    let energy_costs = vec![
        EnergyCost {
            token: EnergyToken::Blood,
            joules: 5_000.0,
            fraction_of_daily_budget: 0.02,
        },
    ];
    let protein_demand = ProteinDemand {
        amino_acid_count: 1_200_000,
        extra_rmr_watts: 0.8,
        amortization: Duration::from_secs(4 * 3600),
    };
    let thermo_envelope = ThermodynamicEnvelope {
        max_delta_celsius: 0.3,
        max_core_celsius: 37.7,
        max_heart_rate_bpm: 110,
    };
    let ml_schedule = MlPassSchedule {
        min_interval: Duration::from_secs(600),
        max_continuous_window: Duration::from_secs(1200),
        cooldown: Duration::from_secs(2400),
    };
    let reversal = ReversalConditions {
        max_inflammation_score: 1.5,
        max_pain_score_vas: 2.0,
        max_performance_deviation_pct: 7.5,
    };
    (energy_costs, protein_demand, thermo_envelope, ml_schedule, reversal)
}
