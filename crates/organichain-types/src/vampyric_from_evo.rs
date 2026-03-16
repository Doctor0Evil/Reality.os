use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::tx::{
    RegionTime,
    BiBinding,
    HostState,
    BciStream,
    EcoState,
    TokenState,
    EvolutionMeta,
    Governance,
    Neurorights,
    Honesty,
    Auth,
    OrganichainTx,
};

use vampyr::vampyric_upgrade::VampiricProposedUpgrade;
use vampyr::vampyric_adoption_evo::VampyricAdoptionEvo;

/// Helper to clamp floats into 0..=1 range where needed.
fn clamp01(x: f32) -> f32 {
    if !x.is_finite() {
        0.0
    } else if x < 0.0 {
        0.0
    } else if x > 1.0 {
        1.0
    } else {
        x
    }
}

/// Convert a VampyricAdoption.evo + VampyricProposedUpgrade into an OrganichainTx.
///
/// This function assumes the evo has already passed:
//  - NeurorightsGuard (multi-region, STRICTEST_WINS)
//  - Fairness gate (Δr ≥ 0, greed-free)
//  - HostBudget / EvolutionWindow safety checks
impl OrganichainTx {
    pub fn from_vampyric_adoption(
        evo: &VampyricAdoptionEvo,
        up: &VampyricProposedUpgrade,
        roh_before: f32,
        roh_after: f32,
        eco_before: f32,
        eco_after: f32,
        fear_before: f32,
        fear_after: f32,
        sanity_before: f32,
        sanity_after: f32,
        txid: String,
        prevtxid: Option<String>,
        session_id: String,
        location_bucket: String,
        blocknumber: Option<u64>,
        tx_hexstamp: String,
        host_bi_signature: String,
        organic_cpu_signature: String,
        stake_multisig: Vec<String>,
    ) -> Result<Self, String> {
        // 1. Region & time context: Phoenix / lab-local session bucket.
        let now = SystemTime::now();
        let time_bucket = chrono::DateTime::<chrono::Utc>::from(now).to_rfc3339();

        let region_time = RegionTime {
            location_bucket,
            time_bucket,
            session_id,
        };

        // 2. BI binding from .evo.
        let bi_binding = BiBinding {
            bi_commitment: evo.bi_commitment.clone(),
            bi_epoch: evo.bi_epoch,
        };

        // 3. Host state: RoH monotone, lifeforce band inferred from trait floor.
        let roh_b = clamp01(roh_before);
        let roh_a = clamp01(roh_after);
        if roh_a > roh_b + 1e-6 {
            return Err(format!(
                "RoH monotonicity violated in vampyric adoption tx: before={} after={}",
                roh_b, roh_a
            ));
        }
        if roh_a > 0.3 + 1e-6 {
            return Err(format!(
                "RoH ceiling 0.3 exceeded in vampyric adoption tx: after={}",
                roh_a
            ));
        }

        let lifeforce_band_before = if evo.lifeforce_band_min >= 0.7 {
            "SAFE".to_string()
        } else {
            "ALERT".to_string()
        };
        let lifeforce_band_after = lifeforce_band_before.clone();

        let host_state = HostState {
            roh_before: roh_b,
            roh_after: roh_a,
            roh_monotone_violation: false,
            biostatebits_before: "N/A".to_string(), // can be wired to real BioState later
            biostatebits_after: "N/A".to_string(),
            lifeforce_band_before,
            lifeforce_band_after,
        };

        // 4. BCI stream from trait caps.
        let bci_stream = BciStream {
            bci_index: clamp01(evo.bci_cap),
            draculawave_quality: 0.0, // placeholder; real impl can map DraculaWave metrics
        };

        // 5. Eco state from inputs.
        let eco_b = clamp01(eco_before);
        let eco_a = clamp01(eco_after);
        let eco_monotone_violation = eco_a > eco_b + 1e-6;

        let eco_state = EcoState {
            eco_load_before: eco_b,
            eco_load_after: eco_a,
            eco_monotone_violation,
        };

        // 6. Token state: FEAR/SANITY from inputs, KARMA from Δr.
        let tok = TokenState {
            fear_before: clamp01(fear_before),
            fear_after: clamp01(fear_after),
            sanity_before: clamp01(sanity_before),
            sanity_after: clamp01(sanity_after),
            karma_delta: evo.delta_r as f32,
        };

        // 7. Evolution meta: link back to evo_id.
        let evolution = EvolutionMeta {
            proposal_id: Some(evo.evo_id.clone()),
            effect_norm: up.descriptor.effect_norm(),
            tsafe_passed: true,
        };

        // 8. Governance invariants: nonsoul, spectral quant, abort flag false.
        let governance = Governance {
            soulmodeling_forbidden: true,
            non_interference_required: true,
            spectral_quant_active: true,
            abort_and_flush: false,
        };

        // 9. Neurorights flags: strict, non-commercial.
        let neurorights = Neurorights {
            mental_privacy: true,
            forbid_decision_use: true,
            non_commercial: true,
        };

        // 10. Honesty: promise type and fulfillment tied to r ≥ 0 and eco non-regression.
        let promise_type = if evo.healthcare_allowance {
            "HEALTHCARE_REGEN".to_string()
        } else {
            "VAMPYRIC_TRAIT_ADOPTION".to_string()
        };

        let promise_fulfilled = !eco_monotone_violation && evo.delta_r >= 0.0;
        let promise_violation_code = if promise_fulfilled {
            None
        } else if eco_monotone_violation {
            Some("ECO_REGRESSION_UNDER_TRAIT".to_string())
        } else {
            Some("DELTA_R_NEGATIVE".to_string())
        };

        let honesty = Honesty {
            promise_type,
            promise_fulfilled,
            promise_violation_code,
        };

        // 11. Auth: BI, OrganicCPU, multisig, tx hexstamp.
        let auth = Auth {
            host_bi_signature,
            organic_cpu_signature,
            stake_multisig,
            tx_hexstamp,
        };

        // 12. Assemble OrganichainTx.
        let tx = OrganichainTx {
            txid,
            prevtxid,
            blocknumber,
            region_time,
            bi_binding,
            host_state,
            bci_stream,
            eco_state,
            tokens: tok,
            evolution,
            governance,
            neurorights,
            honesty,
            auth,
        };

        // 13. Final structural sanity: non-financial and governance-safe.
        if !crate::tx::is_non_financial(&tx) {
            return Err("Non-financial invariant failed for vampyric adoption tx".into());
        }
        if !crate::tx::check_governance(&tx) {
            return Err("Governance invariant failed for vampyric adoption tx".into());
        }

        Ok(tx)
    }
}
