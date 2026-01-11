use clap::{Parser, Subcommand};
use reality_core::RealityManifest;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "reality-lab")]
#[command(about = "Local analysis of Reality.os / Dreamscape.os / XR-Grid clones")]
struct Args {
    /// Path to the xr-lab workspace created by reality-orchestrator
    #[arg(long, default_value = "~/xr-lab")]
    workspace: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// List cloned projects and basic metadata
    List,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let workspace = shellexpand::tilde(&args.workspace).to_string();
    let workspace_path = PathBuf::from(workspace);

    match args.command {
        Command::List => {
            list_projects(&workspace_path)?;
        }
    }

    Ok(())
}

fn list_projects(workspace: &PathBuf) -> anyhow::Result<()> {
    println!("Reality Lab workspace: {}", workspace.display());
    if !workspace.exists() {
        println!("Workspace does not exist yet. Run `reality-orchestrator clone-all` first.");
        return Ok(());
    }

    // Lightweight, filesystem-based inspection that is git-host agnostic.
    for entry in std::fs::read_dir(workspace)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            println!("- {}", entry.file_name().to_string_lossy());
        }
    }

    Ok(())
}
