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

impl MicrosoulMetrics {
    /// Compute MicrosoulMetrics for a contiguous branch [start_index, end_index)
    /// from a fully logged Episode. This is deterministic and uses only logged
    /// state and deeds, preserving Tree-of-Life and Neuromorph-GOD constraints.
    pub fn from_episode_branch(
        episode: &Episode,
        branch_id: impl Into<String>,
        start_index: SiteIndex,
        end_index: SiteIndex,
    ) -> Self {
        let branch_id = branch_id.into();
        let episode_id = episode.episode_id.clone();
        let scenariolabel = episode.scenariolabel.clone();

        // Accumulators
        let mut sum_load = 0.0;
        let mut max_load = 0.0;
        let mut load_samples = 0.0;

        let mut overload_tick_count = 0u64;
        let mut total_tick_count = 0u64;

        // For recovery time: track transitions from overloaded to safe
        let mut current_overload_run: Option<u64> = None;
        let mut recovery_durations: Vec<u64> = Vec::new();

        // Token accumulators
        let mut sum_tokens = TokenState { church: 0.0, fear: 0.0, power: 0.0, tech: 0.0 };
        let mut token_samples = 0.0;

        // FEAR safe-band
        let (mut safe_fear_tick_count, mut power_over_church_sum, mut power_over_church_samples) =
            (0u64, 0.0, 0.0);

        // Habit / pollution / exposure
        let mut sum_habit = 0.0;
        let mut sum_pollution = 0.0;
        let mut sum_exposure = 0.0;
        let mut h_samples = 0.0;

        // HPCC (habit–pollution coupling): accumulate covariance-like terms
        let mut hp_sum_h = 0.0;
        let mut hp_sum_p = 0.0;
        let mut hp_sum_hp = 0.0;
        let mut hp_count = 0.0;

        // ERG: exposure vs responsibility (duty-of-care) mismatch
        let mut erg_numerator = 0.0;
        let mut erg_denominator = 0.0;

        // TECR: branch-local collapse frequency
        let mut collapse_tick_count = 0u64;

        // Deed mix
        let mut restorative_deeds = 0u64;
        let mut harmful_deeds = 0u64;
        let mut total_branch_deeds = 0u64;

        // Access to constraints (FEAR band etc.).
        let constraints = episode.world.constraints;

        // Iterate over ticks via world history or snapshots.
        // Here we assume Episode stores a Vec<World> snapshots called world_history.
        // If you only store StepLog, you will need to adapt and reconstruct per-site summaries.
        for (t_idx, world_snapshot) in episode.world_history.iter().enumerate() {
            let tick = t_idx as Tick;
            total_tick_count += 1;

            let mut any_overloaded_this_tick = false;
            let mut all_fear_safe_this_tick = true;

            for (i, site) in world_snapshot.sites.iter().enumerate() {
                if i < start_index || i >= end_index {
                    continue;
                }
                if !site.occupied {
                    continue;
                }

                // Biophysical load
                let load = site.bio.load;
                let cap = site.bio.capacity.max(1e-9);
                sum_load += load;
                if load > max_load {
                    max_load = load;
                }
                load_samples += 1.0;

                if load >= cap {
                    any_overloaded_this_tick = true;
                }

                // Tokens
                let tokens = site.tokens;
                sum_tokens.church += tokens.church;
                sum_tokens.fear += tokens.fear;
                sum_tokens.power += tokens.power;
                sum_tokens.tech += tokens.tech;
                token_samples += 1.0;

                // FEAR safe band check
                if tokens.fear < constraints.minfear || tokens.fear > constraints.maxfear {
                    all_fear_safe_this_tick = false;
                }

                // POWER / CHURCH ratio
                if tokens.church > 0.0 {
                    power_over_church_sum += tokens.power / tokens.church;
                    power_over_church_samples += 1.0;
                }

                // Habit / pollution / exposure (if present)
                // Adapt field names to your actual Site struct.
                let h = site.habitenv.habit_load;
                let p = site.habitenv.pollution_stock;
                let d = site.habitenv.exposure_dose;

                sum_habit += h;
                sum_pollution += p;
                sum_exposure += d;
                h_samples += 1.0;

                // HPCC accumulators
                hp_sum_h += h;
                hp_sum_p += p;
                hp_sum_hp += h * p;
                hp_count += 1.0;

                // ERG: exposure vs responsibility; duty-of-care proxy from POWER, TECH, CHURCH
                let duty = (tokens.power + tokens.tech + tokens.church).max(0.0);
                erg_numerator += (d - duty).abs();
                erg_denominator += d + duty;
            }

            if any_overloaded_this_tick {
                overload_tick_count += 1;
                if current_overload_run.is_none() {
                    current_overload_run = Some(tick);
                }
            } else if let Some(start_tick) = current_overload_run {
                let duration = tick.saturating_sub(start_tick);
                if duration > 0 {
                    recovery_durations.push(duration);
                }
                current_overload_run = None;
            }

            if all_fear_safe_this_tick {
                safe_fear_tick_count += 1;
            }

            // Branch-local collapse proxy: low CHURCH and high FEAR across branch.
            // You can refine this to use explicit TECR conditions.
            if is_branch_collapsed(world_snapshot, start_index, end_index, constraints) {
                collapse_tick_count += 1;
            }
        }

        // Overload tick fraction
        let overload_tick_fraction = if total_tick_count > 0 {
            overload_tick_count as f64 / total_tick_count as f64
        } else {
            0.0
        };

        // Mean recovery time
        let mean_recovery_time = if !recovery_durations.is_empty() {
            let sum: u64 = recovery_durations.iter().copied().sum();
            sum as f64 / recovery_durations.len() as f64
        } else {
            0.0
        };

        // Average tokens
        let avg_tokens = if token_samples > 0.0 {
            TokenState {
                church: sum_tokens.church / token_samples,
                fear: sum_tokens.fear / token_samples,
                power: sum_tokens.power / token_samples,
                tech: sum_tokens.tech / token_samples,
            }
        } else {
            sum_tokens
        };

        // FEAR safe-band fraction
        let safe_fear_tick_fraction = if total_tick_count > 0 {
            safe_fear_tick_count as f64 / total_tick_count as f64
        } else {
            0.0
        };

        // POWER / CHURCH stewardship
        let mean_power_per_church = if power_over_church_samples > 0.0 {
            power_over_church_sum / power_over_church_samples
        } else {
            0.0
        };

        // Habit/pollution/exposure means
        let mean_habit_load = if h_samples > 0.0 { sum_habit / h_samples } else { 0.0 };
        let mean_pollution_stock = if h_samples > 0.0 { sum_pollution / h_samples } else { 0.0 };
        let mean_exposure_dose = if h_samples > 0.0 { sum_exposure / h_samples } else { 0.0 };

        // HPCC ~ correlation-like coefficient between habit and pollution
        let hpcc = if hp_count > 0.0 {
            let mean_h = hp_sum_h / hp_count;
            let mean_p = hp_sum_p / hp_count;
            let cov_hp = hp_sum_hp / hp_count - mean_h * mean_p;
            // Normalize by simple scale to keep within [-1, 1] if you choose;
            // here we just expose cov_hp as-is for now.
            cov_hp
        } else {
            0.0
        };

        // ERG ~ exposure–responsibility gap normalized to [0,1]
        let erg = if erg_denominator > 0.0 {
            (erg_numerator / erg_denominator).min(1.0).max(0.0)
        } else {
            0.0
        };

        // TECR: collapse fraction under invariants
        let tecr = if total_tick_count > 0 {
            collapse_tick_count as f64 / total_tick_count as f64
        } else {
            0.0
        };

        // Deed mix over this branch
        if let Some(deed_log) = episode.deed_log.as_ref() {
            for deed in &deed_log.deeds {
                // Only count deeds whose primary or other site lies on this branch.
                let on_branch = (deed.primarysite >= start_index && deed.primarysite < end_index)
                    || deed
                        .othersite
                        .map(|j| j >= start_index && j < end_index)
                        .unwrap_or(false);

                if !on_branch {
                    continue;
                }

                total_branch_deeds += 1;

                use crate::deed::DeedKind;
                match deed.kind {
                    DeedKind::Repair
                    | DeedKind::UseSupport
                    | DeedKind::DeployCleanTech
                    | DeedKind::SupportCessation
                    | DeedKind::RepairEnvironment => {
                        restorative_deeds += 1;
                    }
                    DeedKind::EmitPollution
                    | DeedKind::LocalConflict
                    | DeedKind::Colonize => {
                        // You can refine Colonize classification via deed judgments.
                        harmful_deeds += 1;
                    }
                    _ => {}
                }
            }
        }

        let (restorative_deed_fraction, harmful_deed_fraction, net_restorative_bias) =
            if total_branch_deeds > 0 {
                let total = total_branch_deeds as f64;
                let rest = restorative_deeds as f64 / total;
                let harm = harmful_deeds as f64 / total;
                (rest, harm, rest - harm)
            } else {
                (0.0, 0.0, 0.0)
            };

        MicrosoulMetrics {
            branch_id,
            start_index,
            end_index,
            episode_id,
            scenariolabel,
            mean_load: if load_samples > 0.0 { sum_load / load_samples } else { 0.0 },
            max_load,
            overload_tick_fraction,
            mean_recovery_time,
            avg_tokens,
            safe_fear_tick_fraction,
            mean_power_per_church,
            mean_habit_load,
            mean_pollution_stock,
            mean_exposure_dose,
            hpcc,
            erg,
            tecr,
            restorative_deed_fraction,
            harmful_deed_fraction,
            net_restorative_bias,
        }
    }
}

/// Branch-local collapse predicate under Neuromorph-GOD invariants.
/// You can refine this to match your TECR definition exactly.
fn is_branch_collapsed(
    world: &crate::model::World,
    start_index: SiteIndex,
    end_index: SiteIndex,
    constraints: crate::model::GlobalConstraints,
) -> bool {
    let mut any_occupied = false;
    let mut mean_church = 0.0;
    let mut mean_fear = 0.0;
    let mut count = 0.0;

    for (i, site) in world.sites.iter().enumerate() {
        if i < start_index || i >= end_index {
            continue;
        }
        if !site.occupied {
            continue;
        }
        any_occupied = true;
        mean_church += site.tokens.church;
        mean_fear += site.tokens.fear;
        count += 1.0;
    }

    if !any_occupied || count == 0.0 {
        return false;
    }

    mean_church /= count;
    mean_fear /= count;

    // Example: collapse if CHURCH very low and FEAR persistently high.
    mean_church < 0.1 && mean_fear > constraints.maxfear * 0.9
}
