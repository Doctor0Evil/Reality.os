use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use bioscale_types::{HostBudget, BiophysicalFlowsSnapshot, ResponsibilityScalar};
use bioscale_upgrade_store::{UpgradeDecision, UpgradeDescriptor};
use bioscale_fairness_validator::{OuterFreedomModel, FairnessConstrainedUpgrade};
use sovereign_guards_core::{NeurorightsGuard, RegionProfile, GuardVerdict};
use organichain_types::OrganichainTx;

use crate::vampyric_upgrade::{
    VampiricObject,
    TenHexEvidenceBundle,
    EvolutionGiftBundle,
    VampiricProposedUpgrade,
    evaluate_vampiric_with_fairness,
};

// --- ALN-parsed .evo representation ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VampyricAdoptionEvo {
    // META
    pub schemaid: String,
    pub version: String,
    pub created_utc: String,
    pub hexstamp: String,

    // HOST
    pub host_did: String,
    pub primary_bostrom_addr: String,
    pub alt_bostrom_addr: Option<String>,
    pub bi_commitment: String,
    pub bi_epoch: u32,

    // EVOLUTION
    pub evo_id: String,
    pub evo_kind: String,
    pub created_utc_evo: String,
    pub planner_id: String,
    pub mode: String, // CONSERVATIVE / COPILOT

    // TRAIT
    pub trait_id: String,
    pub trait_version: String,
    pub blood_duty_cap: f32,
    pub protein_duty_cap: f32,
    pub bci_cap: f32,
    pub rod_cap: f32,
    pub lifeforce_band_min: f32,
    pub healthcare_allowance: bool,

    // EVIDENCE
    pub hex_1_neuro_floor: String,
    pub hex_2_atp_cap: String,
    pub hex_3_inflammation: String,
    pub hex_4_hrv_resilience: String,
    pub hex_5_eeg_coordination: String,
    pub hex_6_organ_corridor: String,
    pub hex_7_lifeforce_rod: String,
    pub hex_8_ecology: String,
    pub hex_9_formal_proof: String,
    pub hex_10_r_axis: String,

    // FAIRNESS
    pub r_axis_hash: String,
    pub pre_r: f64,
    pub post_r: f64,
    pub delta_r: f64,

    // NEURORIGHTS
    pub clause_ref_primary: String,
    pub clause_ref_secondary: Option<String>,
    pub nonconsensual_mod_forbidden: bool,
    pub data_export_forbidden: bool,
    pub right_to_refuse_anytime: bool,
    pub augmentation_continuity: bool,

    // JURISDICTION
    pub primary_region: String,
    pub secondary_region: Option<String>,
    pub tertiary_region: Option<String>,
    pub smart_profile: String,
    pub applied_profile: String,

    // COMPLIANCE
    pub legal_profile_hash: String,
    pub technical_profile_hash: String,
    pub biophysical_profile_hash: String,
    pub zk_attest_id: Option<String>,

    // ROLLBACK
    pub emergency_corridor_allowed: bool,
    pub emergency_clause_ref: Option<String>,
    pub host_authorized_downgrade: bool,

    // AUDIT
    pub organichain_txid: Option<String>,
    pub googolswarm_proof_id: Option<String>,
    pub git_commit_hash: String,
    pub hexstamp_local: String,
    pub hexstamp_global: Option<String>,

    // SIGN
    pub host_bi_signature: String,
    pub sovereign_kernel_signature: String,
    pub stake_multisig: String,
}

// --- Parsing and basic sanity checks ---

pub fn load_vampyric_adoption_evo<P: AsRef<Path>>(
    path: P,
) -> Result<VampyricAdoptionEvo, String> {
    let mut f = File::open(path).map_err(|e| format!("open .evo failed: {e}"))?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .map_err(|e| format!("read .evo failed: {e}"))?;

    // Assuming ALN has already been converted to JSON for host-local parsing,
    // or an ALN parser maps fields into this struct.
    serde_json::from_str::<VampyricAdoptionEvo>(&buf)
        .map_err(|e| format!("parse .evo JSON failed: {e}"))
}

// --- Conversion into VampiricProposedUpgrade ---

fn evo_to_vampiric_upgrade(
    evo: &VampyricAdoptionEvo,
    descriptor: UpgradeDescriptor,
) -> VampyricProposedUpgrade {
    let vo = VampiricObject {
        trait_id: evo.trait_id.clone(),
        blood_duty_cap: evo.blood_duty_cap,
        protein_duty_cap: evo.protein_duty_cap,
        bci_cap: evo.bci_cap,
        rod_cap: evo.rod_cap,
        lifeforce_band_min: evo.lifeforce_band_min,
    };

    let evidence = TenHexEvidenceBundle {
        hex_1_neuro_floor: evo.hex_1_neuro_floor.clone(),
        hex_2_atp_cap: evo.hex_2_atp_cap.clone(),
        hex_3_inflammation: evo.hex_3_inflammation.clone(),
        hex_4_hrv_resilience: evo.hex_4_hrv_resilience.clone(),
        hex_5_eeg_coordination: evo.hex_5_eeg_coordination.clone(),
        hex_6_organ_corridor: evo.hex_6_organ_corridor.clone(),
        hex_7_lifeforce_rod: evo.hex_7_lifeforce_rod.clone(),
        hex_8_ecology: evo.hex_8_ecology.clone(),
        hex_9_formal_proof: evo.hex_9_formal_proof.clone(),
        hex_10_r_axis: evo.hex_10_r_axis.clone(),
    };

    let bundle = EvolutionGiftBundle {
        host_did: evo.host_did.clone(),
        bostrom_addr: evo.primary_bostrom_addr.clone(),
        vampiric: vo,
        evidence,
        neurorights_clause_ref: evo.clause_ref_primary.clone(),
        r_axis_hash: evo.r_axis_hash.clone(),
    };

    VampyricProposedUpgrade::new(descriptor, bundle)
}

// --- Neurorights + multi-region smart-profile check ---

fn assemble_region_profile(evo: &VampyricAdoptionEvo) -> RegionProfile {
    let mut regions = vec![evo.primary_region.clone()];
    if let Some(r2) = &evo.secondary_region {
        regions.push(r2.clone());
    }
    if let Some(r3) = &evo.tertiary_region {
        regions.push(r3.clone());
    }

    RegionProfile {
        regions,
        smart_profile: evo.smart_profile.clone(), // e.g. STRICTEST_WINS
        applied_profile: evo.applied_profile.clone(),
        legal_profile_hash: evo.legal_profile_hash.clone(),
    }
}

fn check_neurorights_and_regions(
    evo: &VampyricAdoptionEvo,
    guard: &NeurorightsGuard,
) -> Result<(), String> {
    if !evo.nonconsensual_mod_forbidden
        || !evo.data_export_forbidden
        || !evo.right_to_refuse_anytime
        || !evo.augmentation_continuity
    {
        return Err("Neurorights flags not all true".into());
    }

    let profile = assemble_region_profile(evo);
    match guard.evaluate_profile(&profile) {
        GuardVerdict::Allowed => Ok(()),
        GuardVerdict::Denied(reason) => Err(format!("NeurorightsGuard denied: {reason}")),
    }
}

// --- Fairness gate: Δr >= 0 and greed-free outer freedom ---

fn check_fairness_bounds(evo: &VampyricAdoptionEvo) -> Result<ResponsibilityScalar, String> {
    if evo.delta_r < 0.0 {
        return Err(format!("Δr < 0 not allowed for vampyric adoption: {}", evo.delta_r));
    }

    let pre_r = ResponsibilityScalar::from(evo.pre_r);
    let post_r = ResponsibilityScalar::from(evo.post_r);

    if (post_r.value() - pre_r.value() - evo.delta_r).abs() > 1e-6 {
        return Err("r-axis consistency check failed".into());
    }

    Ok(pre_r)
}

// --- Main evaluation path ---

pub struct VampyricAdoptionContext<'a> {
    pub host_budget: &'a HostBudget,
    pub flows_snapshot: &'a BiophysicalFlowsSnapshot,
    pub outer_model: &'a dyn OuterFreedomModel,
    pub neurorights_guard: &'a NeurorightsGuard,
    pub descriptor: UpgradeDescriptor,
}

/// Load, validate, and, if admitted, mint EvolutionPoint + Organichain tx.
pub fn evaluate_and_mint_vampyric_adoption<P: AsRef<Path>>(
    path: P,
    ctx: &VampyricAdoptionContext<'_>,
) -> Result<OrganichainTx, String> {
    let evo = load_vampyric_adoption_evo(path)?;

    // 1. Neurorights + multi-region smart compliance.
    check_neurorights_and_regions(&evo, ctx.neurorights_guard)?;

    // 2. Fairness constraint: Δr ≥ 0 and internal sanity.
    let current_r = check_fairness_bounds(&evo)?;

    // 3. Construct VampiricProposedUpgrade from .evo + descriptor.
    let up = evo_to_vampiric_upgrade(&evo, ctx.descriptor.clone());

    // 4. Evaluate with fairness, HostBudget, and OuterFreedomModel.
    let now = SystemTime::now();
    let decision: UpgradeDecision = evaluate_vampiric_with_fairness(
        &up,
        ctx.host_budget,
        now,
        current_r,
        ctx.flows_snapshot,
        ctx.outer_model,
    );

    match decision {
        UpgradeDecision::Denied { reason } => Err(format!("Upgrade denied: {reason}")),
        UpgradeDecision::Allowed { .. } => {
            // 5. If allowed, mint an OrganichainTx representing the permanent EvolutionPoint.
            let tx = mint_organichain_tx_from_evo(&evo, &up)?;
            Ok(tx)
        }
    }
}

// --- Organichain tx minting ---

fn mint_organichain_tx_from_evo(
    evo: &VampyricAdoptionEvo,
    up: &VampiricProposedUpgrade,
) -> Result<OrganichainTx, String> {
    // This assumes you have the OrganichainTx struct as outlined earlier.
    // Only non-financial, biophysical and governance fields are filled.

    let tx = OrganichainTx::from_vampyric_adoption(evo, up)
        .map_err(|e| format!("OrganichainTx minting failed: {e}"))?;

    Ok(tx)
}
