use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use serde::Deserialize;

use dream_spectre::{
    invariants::{all_invariants_ok, check_neuroshard_invariants},
    ledger::LedgerRow,
    model::EpochRow,
    rights::NeurorightsPolicy,
    shard_csv::load_csv,
};

/// CLI arguments for the invariant harness.
#[derive(Debug, Parser)]
struct Args {
    /// CSV file containing EpochRow records.
    #[arg(long)]
    epochs: PathBuf,

    /// CSV file containing LedgerRow records.
    #[arg(long)]
    ledger: PathBuf,

    /// JSON file containing NeurorightsPolicy (optional; defaults to strict_default).
    #[arg(long)]
    policy: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct PolicyFile {
    mental_privacy: bool,
    cognitive_liberty: bool,
    mental_integrity: bool,
    non_commercial_neural: bool,
    soul_non_addressable: bool,
}

fn load_policy(path: Option<PathBuf>) -> Result<NeurorightsPolicy> {
    if let Some(p) = path {
        let file = File::open(p)?;
        let reader = BufReader::new(file);
        let raw: PolicyFile = serde_json::from_reader(reader)?;
        Ok(NeurorightsPolicy {
            mental_privacy: raw.mental_privacy,
            cognitive_liberty: raw.cognitive_liberty,
            mental_integrity: raw.mental_integrity,
            non_commercial_neural: raw.non_commercial_neural,
            soul_non_addressable: raw.soul_non_addressable,
        })
    } else {
        Ok(NeurorightsPolicy::strict_default())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let epochs: Vec<EpochRow> = load_csv(&args.epochs)?;
    let ledger: Vec<LedgerRow> = load_csv(&args.ledger)?;
    let policy = load_policy(args.policy)?;

    let checks = check_neuroshard_invariants(&epochs, &ledger, &policy);
    let all_ok = all_invariants_ok(&checks);

    println!("all_invariants_ok={}", all_ok);
    for c in &checks {
        println!(
            "invariant name=\"{}\" ok={} message={}",
            c.name,
            c.ok,
            c.message.as_deref().unwrap_or("")
        );
    }

    // Exit code semantics: non-zero if any invariant failed.
    if all_ok {
        Ok(())
    } else {
        std::process::exit(1);
    }
}
