use crate::CargoEnvDescriptor;

#[derive(Clone, Debug)]
pub enum EnvValidationError {
    MissingBioscaleAbi,
    NoDefaultEvidence,
    HostBudgetInsufficient,
    NeurorightsReversalMissing,
    DevTunnelOtaForbidden,
    CommandNotWhitelisted(String),
}

pub trait CargoEnvValidator {
    fn validate_env(&self, desc: &CargoEnvDescriptor) -> Result<(), EnvValidationError>;
}

pub struct PhoenixNeurostackEnvValidator;

impl CargoEnvValidator for PhoenixNeurostackEnvValidator {
    fn validate_env(&self, desc: &CargoEnvDescriptor) -> Result<(), EnvValidationError> {
        if !desc.neurorights.bioscale_abi_present {
            return Err(EnvValidationError::MissingBioscaleAbi);
        }
        if desc.neurorights.default_evidence.sequences.is_empty() {
            return Err(EnvValidationError::NoDefaultEvidence);
        }
        if !desc.neurorights.reversal_supported {
            return Err(EnvValidationError::NeurorightsReversalMissing);
        }
        if !desc.dev_tunnel.ota_updates_enabled {
            return Err(EnvValidationError::DevTunnelOtaForbidden);
        }
        if !desc
            .dev_tunnel
            .allowlist_commands
            .iter()
            .any(|c| c == "cargo check")
        {
            return Err(EnvValidationError::CommandNotWhitelisted(
                "cargo check".into(),
            ));
        }
        Ok(())
    }
}
