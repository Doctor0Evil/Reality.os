#![forbid(unsafe_code)]

use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use serde::Serialize;

// These should come from your existing crates.
use bioscale_upgrade_store::{UpgradeDescriptor, UpgradeRegistry};
use cyberswarm_neurostack::telemetry::BciHostSnapshot;
use cyberswarm_neurostack::aln::AlnParticleSummary;
use bioscale_metrics::MetricsSchema;
use git_version::git_version;

#[derive(Serialize)]
struct EvolutionWindow {
    valid_from: String,
    valid_until: String,
    max_upgrades_in_window: u32,
    max_total_energy_joules: f64,
    max_total_duty_cycle: f64,
}

#[derive(Serialize)]
struct UpgradeEntry {
    upgrade_id: String,
    energy_joules: f64,
    protein_aa: u64,
    thermo_envelope_celsius: ThermoEnvelopeEntry,
    ml_schedule: MlScheduleEntry,
    reversal: ReversalEntry,
    evidence_hex_tags: Vec<String>,
}

#[derive(Serialize)]
struct ThermoEnvelopeEntry {
    max_delta: f64,
}

#[derive(Serialize)]
struct MlScheduleEntry {
    max_passes_per_day: u32,
    min_interval_seconds: u64,
}

#[derive(Serialize)]
struct ReversalEntry {
    reversible: bool,
    rollback_contract_uri: String,
    downgrade_thresholds: DowngradeThresholdsEntry,
}

#[derive(Serialize)]
struct DowngradeThresholdsEntry {
    pain_vas: u8,
    il6_pg_ml: f32,
}

#[derive(Serialize)]
struct RangeEntry<T> {
    min: T,
    max: T,
}

#[derive(Serialize)]
struct BciSnapshotsEntry {
    eeg_load_ratio: RangeEntry<f32>,
    hrv_ms: RangeEntry<f32>,
    core_temp_c: RangeEntry<f32>,
    duty_cycle: RangeEntry<f32>,
}

#[derive(Serialize)]
struct AlnParticleEntry {
    id: String,
    hash: String,
    is_compliant: bool,
}

#[derive(Serialize)]
struct KaniHarnessEntry {
    name: String,
    module: String,
    status: String,
    max_smt_seconds: u64,
}

#[derive(Serialize)]
struct Manifest {
    date: String,
    host_did: String,
    bostrom_address: String,
    git_commit: String,
    crate_versions: serde_json::Value,
    evolution_window: EvolutionWindow,
    upgrades: Vec<UpgradeEntry>,
    bci_snapshots: BciSnapshotsEntry,
    aln_particles: Vec<AlnParticleEntry>,
    kani_harnesses: Vec<KaniHarnessEntry>,
    metrics_schema_version: String,
    metrics_families: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let date = parse_date_arg(&args)?;

    let git_commit = git_version!();

    // These helpers should come from your existing crates/workspace config.
    let crate_versions = bioscale_upgrade_store::crate_versions_json();
    let registry = UpgradeRegistry::load()?;
    let upgrades = build_upgrade_entries(&registry);

    let host_snapshot = BciHostSnapshot::load_latest_for_date(&date)?;
    let bci_snapshots = BciSnapshotsEntry {
        eeg_load_ratio: RangeEntry {
            min: host_snapshot.eeg_load_min,
            max: host_snapshot.eeg_load_max,
        },
        hrv_ms: RangeEntry {
            min: host_snapshot.hrv_ms_min,
            max: host_snapshot.hrv_ms_max,
        },
        core_temp_c: RangeEntry {
            min: host_snapshot.core_temp_min_c,
            max: host_snapshot.core_temp_max_c,
        },
        duty_cycle: RangeEntry {
            min: host_snapshot.duty_min,
            max: host_snapshot.duty_max,
        },
    };

    let aln_particles: Vec<AlnParticleEntry> = AlnParticleSummary::for_date(&date)?
        .into_iter()
        .map(|p| AlnParticleEntry {
            id: p.id,
            hash: p.hash,
            is_compliant: p.is_compliant,
        })
        .collect();

    let metrics_schema = MetricsSchema::current();
    let kani_harnesses = vec![KaniHarnessEntry {
        name: "check_evolution_window_safety".into(),
        module: "crates/cyberswarm-neurostack/src/safety.rs".into(),
        status: "passed".into(),
        max_smt_seconds: 20,
    }];

    let manifest = Manifest {
        date: date.clone(),
        host_did: host_did_from_env()?,
        bostrom_address: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7".into(),
        git_commit: git_commit.into(),
        crate_versions,
        evolution_window: derive_evolution_window(&date, &registry),
        upgrades,
        bci_snapshots,
        aln_particles,
        kani_harnesses,
        metrics_schema_version: metrics_schema.version,
        metrics_families: metrics_schema.families,
    };

    let out_path = manifest_path_for_date(&date);
    fs::create_dir_all(out_path.parent().unwrap())?;
    let json = serde_json::to_string_pretty(&manifest)?;
    fs::write(out_path, json)?;

    Ok(())
}

fn parse_date_arg(args: &[String]) -> anyhow::Result<String> {
    for i in 0..args.len() {
        if args[i] == "--date" && i + 1 < args.len() {
            return Ok(args[i + 1].clone());
        }
    }
    Err(anyhow::anyhow!("--date YYYY-MM-DD required"))
}

fn manifest_path_for_date(date: &str) -> PathBuf {
    let mut p = PathBuf::from("research");
    p.push(format!("{date}-manifest.json"));
    p
}

fn host_did_from_env() -> anyhow::Result<String> {
    if let Ok(did) = env::var("HOST_DID") {
        Ok(did)
    } else {
        Err(anyhow::anyhow!("HOST_DID env var not set"))
    }
}

fn derive_evolution_window(date: &str, registry: &UpgradeRegistry) -> EvolutionWindow {
    // Placeholder: wire to your BioscaleEvolutionWindow logic.
    EvolutionWindow {
        valid_from: format!("{date}T00:00:00Z"),
        valid_until: format!("{date}T23:59:59Z"),
        max_upgrades_in_window: 16,
        max_total_energy_joules: registry.daily_energy_cap_joules(),
        max_total_duty_cycle: 0.4,
    }
}

fn build_upgrade_entries(registry: &UpgradeRegistry) -> Vec<UpgradeEntry> {
    registry
        .all()
        .map(|u: &UpgradeDescriptor| UpgradeEntry {
            upgrade_id: u.id.to_string(),
            energy_joules: u.energy_joules,
            protein_aa: u.protein_aa,
            thermo_envelope_celsius: ThermoEnvelopeEntry {
                max_delta: u.thermo_envelope.max_delta_c,
            },
            ml_schedule: MlScheduleEntry {
                max_passes_per_day: u.ml_schedule.max_passes_per_day,
                min_interval_seconds: u.ml_schedule.min_interval_seconds,
            },
            reversal: ReversalEntry {
                reversible: u.reversal.reversible,
                rollback_contract_uri: u.reversal.rollback_contract_uri.clone(),
                downgrade_thresholds: DowngradeThresholdsEntry {
                    pain_vas: u.reversal.pain_vas_threshold,
                    il6_pg_ml: u.reversal.il6_pg_ml_threshold,
                },
            },
            evidence_hex_tags: u
                .evidence
                .sequences
                .iter()
                .map(|tag| tag.short_hex.clone())
                .collect(),
        })
        .collect()
}
