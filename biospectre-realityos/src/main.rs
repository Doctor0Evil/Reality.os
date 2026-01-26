use biospectre_realityos::eco_governor_integration::*;
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), anyhow::Error> {
// Load configuration
let config_path = PathBuf::from(env::var("REALITYOS_CONFIG")?
.unwrap_or("config/realityos/eco_governor.toml".to_string()));
let config = EcoGovernorConfig::load(&config_path)?;

// Initialize EcoGovernorService
let service = EcoGovernorService::new(config)?;

// Register service with RealityOS
let realityos = realityos::Runtime::new()?;
realityos.register_service("eco_governor", service)?;

// Start the service
realityos.start()?;

Ok(())
}
