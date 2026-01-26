// ──────────────────────────────────────────────────────────────
// Bioscale Token Dynamics: OrganicCpu-Coupled Budgets
// Blood / Brain / Protein / Eco tokens with biophysical corridors
// ──────────────────────────────────────────────────────────────

use aln_core::*;
use didkit::DID;
use serde::{Serialize, Deserialize};

use crate::audit::{
    BioTokenKind,
    OrganicCpuRouterThresholds,
};

/// Per-DID biophysical token balances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BioTokenBalances {
    pub owner_did: DID,
    pub blood_tokens: i64,
    pub brain_tokens: i64,
    pub protein_tokens: i64,
    pub eco_tokens: i64,
    pub inclusion_tokens: i64,
}

/// Corridor configuration: permissive but bounded ranges.
/// These numbers are *per-host* and can be tuned by you.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiophysicalCorridors {
    /// Target “sport” zone (where you like to train).
    pub comfort_low: f32,   // e.g., 0.2 (20% effort)
    pub comfort_high: f32,  // e.g., 0.6 (60% effort)
    /// Challenging but acceptable zone (where you push limits).
    pub challenge_high: f32, // e.g., 0.85 (85% effort)
    /// Absolute ceiling for safety; never exceeded.
    pub hard_ceiling: f32,  // e.g., 1.0 (100% configured capacity)
}

/// Dynamic state for a single organic_cpu host.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganicCpuDynamicState {
    pub router: OrganicCpuRouterThresholds,
    pub corridors: BiophysicalCorridors,
    /// Rolling exposure in [0,1] for current session.
    pub current_effort: f32,
    /// Longitudinal fatigue marker (0 = fresh, 1 = exhausted).
    pub fatigue_index: f32,
    /// User-consented “sport” mode flag (true = higher corridor allowed).
    pub sport_mode: bool,
}

/// Token dynamics engine: couples thresholds and balances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDynamicsEngine {
    pub host_state: OrganicCpuDynamicState,
    pub balances: BioTokenBalances,
}

impl TokenDynamicsEngine {
    pub fn new(
        host_state: OrganicCpuDynamicState,
        balances: BioTokenBalances,
    ) -> Self {
        TokenDynamicsEngine { host_state, balances }
    }

    /// Compute normalized effort from requested spend in tokens.
    /// This is local logic: you decide the mapping.
    fn compute_effort(&self, brain_spend: i64, blood_spend: i64) -> f32 {
        let brain_norm = if self.host_state.router.max_brain_tokens_per_hour > 0 {
            (brain_spend as f32
                / self.host_state.router.max_brain_tokens_per_hour as f32)
                .clamp(0.0, 1.0)
        } else {
            0.0
        };

        let blood_norm = if self.host_state.router.max_blood_tokens_per_day > 0 {
            (blood_spend as f32
                / self.host_state.router.max_blood_tokens_per_day as f32)
                .clamp(0.0, 1.0)
        } else {
            0.0
        };

        // Simple composite; can be refined later.
        (brain_norm * 0.6) + (blood_norm * 0.4)
    }

    /// Check if a proposed effort is within corridor (sport-aware).
    fn check_corridor(&self, proposed_effort: f32) -> Result<(), &'static str> {
        let c = &self.host_state.corridors;

        if proposed_effort > c.hard_ceiling {
            return Err("Requested effort exceeds hard safety ceiling");
        }

        if self.host_state.sport_mode {
            // In sport mode, allow up to challenge_high, but warn if near ceiling.
            if proposed_effort > c.challenge_high {
                return Err("Requested effort exceeds configured challenge corridor");
            }
        } else {
            // Non-sport mode: keep within comfort band.
            if proposed_effort > c.comfort_high {
                return Err("Requested effort exceeds comfort corridor; enable sport_mode explicitly");
            }
        }

        Ok(())
    }

    /// Apply a single high-intensity “interval” spend.
    /// This does not forbid strong efforts, it enforces your configured corridors.
    pub fn apply_interval(
        &mut self,
        brain_spend: i64,
        blood_spend: i64,
        protein_support: i64,
    ) -> Result<(), &'static str> {
        if brain_spend < 0 || blood_spend < 0 || protein_support < 0 {
            return Err("Negative spends are not allowed");
        }

        if self.balances.brain_tokens < brain_spend
            || self.balances.blood_tokens < blood_spend
            || self.balances.protein_tokens < protein_support
        {
            return Err("Insufficient token balances for requested interval");
        }

        let proposed_effort = self.compute_effort(brain_spend, blood_spend);
        self.check_corridor(proposed_effort)?;

        // Update balances
        self.balances.brain_tokens -= brain_spend;
        self.balances.blood_tokens -= blood_spend;
        self.balances.protein_tokens -= protein_support;

        // Update host effort and fatigue in a graded way.
        let alpha = 0.5; // weighting for current interval
        self.host_state.current_effort =
            (1.0 - alpha) * self.host_state.current_effort + alpha * proposed_effort;

        // Simple fatigue model: more effort -> higher fatigue
        let fatigue_delta = proposed_effort * 0.1;
        self.host_state.fatigue_index =
            (self.host_state.fatigue_index + fatigue_delta).clamp(0.0, 1.0);

        Ok(())
    }

    /// Recover function: uses Eco and Protein tokens to bring fatigue down.
    pub fn apply_recovery(&mut self, eco_spend: i64, protein_spend: i64) -> Result<(), &'static str> {
        if eco_spend < 0 || protein_spend < 0 {
            return Err("Negative spends are not allowed");
        }

        if self.balances.eco_tokens < eco_spend
            || self.balances.protein_tokens < protein_spend
        {
            return Err("Insufficient recovery balances");
        }

        self.balances.eco_tokens -= eco_spend;
        self.balances.protein_tokens -= protein_spend;

        // Recovery is more efficient at lower fatigue.
        let base_recovery = 0.15;
        let fatigue_factor = 1.0 - self.host_state.fatigue_index;
        let recovery = base_recovery * fatigue_factor;

        self.host_state.fatigue_index =
            (self.host_state.fatigue_index - recovery).clamp(0.0, 1.0);

        Ok(())
    }

    /// Explicit user action: enable sport mode to allow higher corridors.
    /// This should be tied to a DID-signed ALN consent contract upstream.
    pub fn enable_sport_mode(&mut self) {
        self.host_state.sport_mode = true;
    }

    /// Explicit user action: disable sport mode and drift back to comfort band.
    pub fn disable_sport_mode(&mut self) {
        self.host_state.sport_mode = false;
    }

    /// Suggest next-interval intensity from current state.
    /// This never forces behavior; it is a recommendation.
    pub fn suggested_effort(&self) -> f32 {
        let c = &self.host_state.corridors;

        if self.host_state.sport_mode {
            // In sport mode, aim between comfort_high and challenge_high, but
            // adjust down if fatigue is high.
            let span = c.challenge_high - c.comfort_high;
            let fatigue_scale = 1.0 - self.host_state.fatigue_index;
            c.comfort_high + span * fatigue_scale
        } else {
            // In regular mode, stay in comfort band.
            let mid = (c.comfort_low + c.comfort_high) * 0.5;
            let fatigue_scale = 1.0 - self.host_state.fatigue_index;
            mid * fatigue_scale
        }
    }
}

/// Helper: default corridors tuned for “sport but safe”.
pub fn default_corridors() -> BiophysicalCorridors {
    BiophysicalCorridors {
        comfort_low: 0.2,
        comfort_high: 0.6,
        challenge_high: 0.85,
        hard_ceiling: 1.0,
    }
}

/// Helper: construct a default OrganicCpu state for offline/local profiles.
pub fn default_host_state(router: OrganicCpuRouterThresholds) -> OrganicCpuDynamicState {
    OrganicCpuDynamicState {
        router,
        corridors: default_corridors(),
        current_effort: 0.0,
        fatigue_index: 0.0,
        sport_mode: false,
    }
}
