use std::time::Duration;

/// Normalized host envelope derived from ETW + Job Objects under ALN contracts.
#[derive(Debug, Clone)]
pub struct Win10HostBudget {
    /// Fraction of wall-clock in which CPU was non-idle for the BCI/app jobs, [0,1].
    pub cpu_duty_cycle_1s: f32,
    /// Reliability-over-headroom ratio (used / reserved budget), [0,1].
    pub roh_score: f32,
    /// Estimated ETW tracing overhead relative to total CPU, [0,1].
    pub etw_overhead: f32,
    /// Current CPU cap applied to the Job Object hosting neural/UX helpers, [0,1].
    pub job_cpu_cap: f32,
    /// Sliding-window fatigue index fused from EMG/EEG/behavior, [0,1].
    pub fatigue_index: f32,
    /// Jurisdictional flag for mental-privacy scope (e.g., GDPR, HIPAA, APPI).
    pub privacy_scope: PrivacyScope,
}

/// Regional mental-privacy profiles; pure software labels.
#[derive(Debug, Clone, Copy)]
pub enum PrivacyScope {
    GdprEu,
    HipaaUs,
    AppiJapan,
    Other,
}

impl Win10HostBudget {
    /// Enforce RoH ≤ 0.3 and fatigue-aware throttling before scheduling work.
    pub fn can_schedule_slot(&self, _slot: Duration) -> bool {
        // Hard neurorights/eco invariant: keep RoH below 0.3.
        if self.roh_score > 0.3 {
            return false;
        }
        // Tighten CPU cap when fatigue is high (simple policy).
        let max_cap = if self.fatigue_index > 0.8 { 0.2 } else { 0.5 };
        self.job_cpu_cap <= max_cap && self.etw_overhead <= 0.01
    }
}
