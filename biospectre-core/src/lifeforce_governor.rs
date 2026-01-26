use std::cmp::Ordering;

/// Token balances for the host.
#[derive(Debug, Clone)]
pub struct HostTokens {
    pub blood: u64,
    pub protein: u64,
    pub sugar: u64,
    pub brain: u64,
    pub dw: u64, // Dracula_Wave tokens
}

/// Biophysical status proxies (software-only, derived from measurements).
#[derive(Debug, Clone)]
pub struct BiophysicalStatus {
    /// 0–1 proxy for hemodynamic stability (blood pressure, HR, anemia risk).
    pub blood_stability_01: f32,
    /// 0–1 proxy for protein reserve (nutrition, muscle mass, catabolism).
    pub protein_reserve_01: f32,
    /// 0–1 proxy for glycemic stability (glucose, hypoglycemia risk).
    pub sugar_stability_01: f32,
    /// 0–1 proxy for cognitive clarity (from N-stage, S_?, clarity score).
    pub brain_clarity_01: f32,
    /// 0–1 proxy for short-term resilience (HRV, N3 density, recovery).
    pub chi_resilience_01: f32,
    /// 0–1 proxy for long-term lifeforce trend (rolling multi-day curve).
    pub lifeforce_trend_01: f32,
}

/// Scalar lifeforce summary and risk band.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifeforceBand {
    Green,
    Yellow,
    Red,
}

#[derive(Debug, Clone)]
pub struct LifeforceState {
    pub lifeforce_scalar_01: f32,
    pub band: LifeforceBand,
}

/// Policy thresholds for DW usage.
#[derive(Debug, Clone)]
pub struct LifeforcePolicy {
    /// Minimum lifeforce required to allow DW at all.
    pub min_lifeforce_for_dw: f32,
    /// Minimum lifeforce required before DW may touch BLOOD.
    pub min_lifeforce_for_blood_spend: f32,
    /// Maximum fraction of BLOOD tokens that a single DW action may consume.
    pub max_blood_fraction_per_dw: f32,
    /// Minimum chi resilience for heavy DW actions.
    pub min_chi_for_heavy_dw: f32,
}

impl Default for LifeforcePolicy {
    fn default() -> Self {
        Self {
            min_lifeforce_for_dw: 0.40,
            min_lifeforce_for_blood_spend: 0.65,
            max_blood_fraction_per_dw: 0.08, // 8% of current BLOOD per event
            min_chi_for_heavy_dw: 0.55,
        }
    }
}

/// Result of a DW spend decision.
#[derive(Debug, Clone)]
pub struct DwSpendDecision {
    pub allowed: bool,
    pub reason: String,
    pub blood_spent: u64,
    pub protein_spent: u64,
    pub sugar_spent: u64,
    pub dw_spent: u64,
}

/// Core governor that computes lifeforce and mediates DW actions.
pub struct LifeforceGovernor {
    policy: LifeforcePolicy,
}

impl LifeforceGovernor {
    pub fn new(policy: LifeforcePolicy) -> Self {
        Self { policy }
    }

    /// Compute lifeforce scalar from biophysical status.
    pub fn compute_lifeforce(&self, status: &BiophysicalStatus) -> LifeforceState {
        // Weights emphasize blood and brain clarity.
        let wb = 0.35;
        let wp = 0.20;
        let ws = 0.20;
        let wc = 0.25;

        // Clamp inputs to [0,1].
        let b = status.blood_stability_01.clamp(0.0, 1.0);
        let p = status.protein_reserve_01.clamp(0.0, 1.0);
        let s = status.sugar_stability_01.clamp(0.0, 1.0);
        let c = status.brain_clarity_01.clamp(0.0, 1.0);

        let lifeforce_scalar = (b.powf(wb) * p.powf(wp) * s.powf(ws) * c.powf(wc))
            * status.lifeforce_trend_01.clamp(0.6, 1.1)
            * status.chi_resilience_01.clamp(0.7, 1.1);

        let lf_clamped = lifeforce_scalar.clamp(0.0, 1.0);

        let band = if lf_clamped >= 0.75 {
            LifeforceBand::Green
        } else if lf_clamped >= 0.50 {
            LifeforceBand::Yellow
        } else {
            LifeforceBand::Red
        };

        LifeforceState {
            lifeforce_scalar_01: lf_clamped,
            band,
        }
    }

    /// Decide whether a DW action can proceed and how to distribute its cost.
    /// `intensity_01` ~ 0–1 indicating how strong the DW event is.
    pub fn decide_dw_spend(
        &self,
        tokens: &mut HostTokens,
        status: &BiophysicalStatus,
        intensity_01: f32,
    ) -> DwSpendDecision {
        let lf_state = self.compute_lifeforce(status);

        if tokens.dw == 0 {
            return DwSpendDecision {
                allowed: false,
                reason: "No Dracula_Wave (DW) tokens available.".to_string(),
                blood_spent: 0,
                protein_spent: 0,
                sugar_spent: 0,
                dw_spent: 0,
            };
        }

        if lf_state.lifeforce_scalar_01 < self.policy.min_lifeforce_for_dw {
            return DwSpendDecision {
                allowed: false,
                reason: format!(
                    "Lifeforce {:.2} below minimum {:.2} for any DW activity.",
                    lf_state.lifeforce_scalar_01, self.policy.min_lifeforce_for_dw
                ),
                blood_spent: 0,
                protein_spent: 0,
                sugar_spent: 0,
                dw_spent: 0,
            };
        }

        // Determine baseline metabolic costs from intensity.
        let intensity = intensity_01.clamp(0.0, 1.0);

        // Base metabolic needs in token units (tunable).
        let base_protein_need = (5.0 * intensity).round() as u64;
        let base_sugar_need = (8.0 * intensity).round() as u64;

        let mut protein_spent = base_protein_need.min(tokens.protein);
        let mut sugar_spent = base_sugar_need.min(tokens.sugar);
        let mut blood_spent = 0_u64;

        // If PROTEIN or SUGAR are insufficient, consider BLOOD as fallback.
        let protein_deficit = base_protein_need.saturating_sub(protein_spent);
        let sugar_deficit = base_sugar_need.saturating_sub(sugar_spent);
        let metabolic_deficit = protein_deficit + sugar_deficit;

        if metabolic_deficit > 0 {
            // Only allow BLOOD fallback when lifeforce and chi are high enough.
            if lf_state.lifeforce_scalar_01 >= self.policy.min_lifeforce_for_blood_spend
                && status.chi_resilience_01 >= self.policy.min_chi_for_heavy_dw
                && !matches!(lf_state.band, LifeforceBand::Red)
            {
                let max_blood_allowed =
                    (tokens.blood as f32 * self.policy.max_blood_fraction_per_dw).floor() as u64;
                blood_spent = metabolic_deficit.min(max_blood_allowed);
            }
        }

        // Decide if we can cover the DW cost without overstepping lifeforce.
        if metabolic_deficit > 0 && blood_spent == 0 {
            return DwSpendDecision {
                allowed: false,
                reason: "Insufficient PROTEIN/SUGAR and BLOOD fallback disallowed by lifeforce bands."
                    .to_string(),
                blood_spent: 0,
                protein_spent: 0,
                sugar_spent: 0,
                dw_spent: 0,
            };
        }

        // At this point, we allow one DW token spend and deduct resources.
        tokens.dw = tokens.dw.saturating_sub(1);
        tokens.protein = tokens.protein.saturating_sub(protein_spent);
        tokens.sugar = tokens.sugar.saturating_sub(sugar_spent);
        tokens.blood = tokens.blood.saturating_sub(blood_spent);

        // Build reason string with band context.
        let band_str = match lf_state.band {
            LifeforceBand::Green => "green",
            LifeforceBand::Yellow => "yellow",
            LifeforceBand::Red => "red",
        };

        let reason = format!(
            "DW event allowed at intensity {:.2}; lifeforce {:.2} ({}) band, \
             protein_spent={}, sugar_spent={}, blood_spent={}.",
            intensity,
            lf_state.lifeforce_scalar_01,
            band_str,
            protein_spent,
            sugar_spent,
            blood_spent
        );

        DwSpendDecision {
            allowed: true,
            reason,
            blood_spent,
            protein_spent,
            sugar_spent,
            dw_spent: 1,
        }
    }
}
