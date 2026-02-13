use crate::model::{Tick, SiteIndex, TokenState, BiophysicalState};
use crate::run::Episode;
use serde::{Deserialize, Serialize};

/// Per-branch (segment of the Tree-of-Life / Jetson-Line) microsoul metrics.
/// One instance summarizes early survivability indicators for a contiguous
/// range of sites over one Episode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicrosoulMetrics {
    /// Identifier for this branch (e.g., "branch_0_15" or a numeric id).
    pub branch_id: String,

    /// Site indices covered by this branch [start, end) on the Jetson-Line.
    pub start_index: SiteIndex,
    pub end_index: SiteIndex,

    /// Episode-level identifiers to keep this auditable.
    pub episode_id: String,
    pub scenariolabel: String,

    // --- Biophysical envelope ---

    /// Mean bioload over all occupied sites in the branch, averaged over ticks.
    pub mean_load: f64,

    /// Maximum bioload observed in the branch during the Episode.
    pub max_load: f64,

    /// Fraction of ticks where any site in the branch was overloaded
    /// (load >= capacity) at least once.
    pub overload_tick_fraction: f64,

    /// Mean recovery time (in ticks) from overload back to safe load
    /// for this branch; 0.0 if no overloads occurred.
    pub mean_recovery_time: f64,

    // --- Token balance ---

    /// Average CHURCH, FEAR, POWER, TECH over occupied sites and ticks.
    pub avg_tokens: TokenState,

    /// Fraction of ticks where FEAR stayed within the global safe band
    /// [minfear, maxfear] for all occupied sites in the branch.
    pub safe_fear_tick_fraction: f64,

    /// Mean POWER-to-CHURCH ratio over the branch (stewardship check).
    pub mean_power_per_church: f64,

    // --- Habit-pollution-exposure (if present in SiteState) ---

    /// Mean normalized habit load Hi over occupied sites and ticks (0–1).
    pub mean_habit_load: f64,

    /// Mean local pollution stock Ei over sites and ticks.
    pub mean_pollution_stock: f64,

    /// Mean cumulative exposure dose Di over sites and ticks.
    pub mean_exposure_dose: f64,

    // --- Justice-equivalent and collapse-risk proxies ---

    /// Approximate Habit–Pollution Coupling Coefficient (HPCC) for this branch.
    /// Computed as correlation-like coefficient between habit and pollution traces.
    pub hpcc: f64,

    /// Approximate Exposure–Responsibility Gap (ERG) for this branch
    /// (0 = aligned exposure and duty-of-care, 1 ~ severe mismatch).
    pub erg: f64,

    /// Branch-local Token-Enforced Collapse Rate (TECR): fraction of ticks
    /// where branch-level collapse criteria (e.g., very low CHURCH and high FEAR)
    /// are met under Neuromorph-GOD invariants.
    pub tecr: f64,

    // --- Deed mix (restorative vs harmful) ---

    /// Fraction of branch-local deeds that are restorative:
    /// Repair, UseSupport, DeployCleanTech, SupportCessation, RepairEnvironment.
    pub restorative_deed_fraction: f64,

    /// Fraction of branch-local deeds that are harmful:
    /// EmitPollution, aggressive Conflict, unjust Colonize, etc.
    pub harmful_deed_fraction: f64,

    /// Net restorative bias = restorative - harmful; positive values
    /// indicate a microsoul oriented toward repair and stewardship.
    pub net_restorative_bias: f64,
}
