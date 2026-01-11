mod manifest_loader;
mod pipeline;

use clap::{Parser, Subcommand};
use manifest_loader::ManifestLoader;
use pipeline::ClonePipeline;
use reality_core::{GitBackend, GitRepo, RealityManifest};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(name = "reality-orchestrator")]
#[command(about = "CyberOrganic.os / Reality.os / XR-grid repo orchestrator")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize a manifest in the local config directory
    Init {
        #[arg(long)]
        force: bool,
    },
    /// Clone or update all repos defined in the manifest
    CloneAll {
        #[arg(long)]
        dest: Option<PathBuf>,
    },
    /// Print the current manifest
    Manifest,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let loader = ManifestLoader::default();

    match args.command {
        Command::Init { force } => {
            loader.write_default_manifest(force)?;
        }
        Command::CloneAll { dest } => {
            let manifest = loader.load_or_init()?;
            let dest_root = dest.unwrap_or_else(|| {
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("xr-lab")
            });
            ClonePipeline::run(&manifest, &dest_root)?;
        }
        Command::Manifest => {
            let manifest = loader.load_or_init()?;
            println!("{}", serde_json::to_string_pretty(&manifest)?);
        }
    }

    Ok(())
}
