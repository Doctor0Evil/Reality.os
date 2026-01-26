use biospectre_core::asset_eco_governor::*;
use biospectre_core::did_verification::DIDSignature;
use biospectre_core::qpudatashards::QPUDataShard;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for EcoGovernor integration with RealityOS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcoGovernorConfig {
    pub stakeholder_did: String,
    pub policy: UpgradePolicy,
    pub aln_shard_path: PathBuf,
}

/// EcoGovernor service for RealityOS
pub struct EcoGovernorService {
    config: EcoGovernorConfig,
    governor: AssetEcoGovernor,
}

impl EcoGovernorService {
    pub fn new(config: EcoGovernorConfig) -> Result<Self> {
        let governor = AssetEcoGovernor::new(config.policy, &config.stakeholder_did);
        Ok(Self {
            config,
            governor,
        })
    }

    /// Process an upgrade request with DID verification
    pub fn process_upgrade_request(
        &self,
        wallet: &mut AssetWallet,
        nightly_profile: &NightlyEcoProfile,
        integration: IntegrationType,
        eco_cost: u64,
        evolution_cost: u64,
        signature: DIDSignature,
    ) -> UpgradeDecision {
        self.governor.decide_upgrade(
            wallet,
            nightly_profile,
            integration,
            eco_cost,
            evolution_cost,
            Some(signature),
        )
    }

    /// Load and validate ALN shard for governance
    pub fn load_aln_shard(&self) -> Result<QPUDataShard> {
        let shard = QPUDataShard::load(&self.config.aln_shard_path)
            .context("Failed to load ALN shard")?;
        
        // Validate shard against schema
        if !shard.validate_schema("biospectre.eco_upgrade_profile") {
            return Err(anyhow::anyhow!("ALN shard schema validation failed"));
        }
        
        Ok(shard)
    }

    /// Apply upgrade decision to ALN shard
    pub fn apply_upgrade_decision(
        &self,
        shard: &mut QPUDataShard,
        decision: &UpgradeDecision,
    ) -> Result<()> {
        // Add decision to upgrade_decision_log
        let decision_entry = serde_json::json!({
            "decision_id": format!("upg-{}", chrono::Utc::now().timestamp()),
            "integration_type": decision.reason,
            "eco_cost": decision.reason,
            "evolution_cost": decision.reason,
            "allowed": decision.allowed,
            "reason": decision.reason,
            "did_signature": decision.did_signature.map(|sig| serde_json::json!(sig)),
        });
        
        // Update shard with new decision
        shard.update("upgrade_decision_log", decision_entry)?;
        
        // Save updated shard
        shard.save()?;
        
        Ok(())
    }
}
