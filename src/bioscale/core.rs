use std::time::{Duration, Instant};

/// OrganicCpuState models host-side cognitive and eco load envelopes.
#[derive(Debug, Clone)]
pub struct OrganicCpuState {
    pub fatigue_index: f32,        // 0.0–1.0 normalized fatigue
    pub engagement_band: f32,      // 0.0–1.0 healthy engagement score
    pub roh_level: f32,            // 0.0–1.0 risk-of-harm composite
    pub duty_cycle_10min: f32,     // fraction of active decoding in last 10 min
    pub eco_impact_score: f32,     // 0.0–1.0 eco score (higher is better)
    pub avg_device_hours_reduced: f32, // hours/day displaced vs baseline
}

impl OrganicCpuState {
    pub fn is_within_safe_roh(&self) -> bool {
        self.roh_level <= 0.3 && self.fatigue_index <= 0.3
    }

    pub fn should_throttle(&self) -> bool {
        self.fatigue_index > 0.25 || self.duty_cycle_10min > 0.4
    }
}

/// BiofieldCommunicator exposes sparse, event-driven intents.
#[derive(Debug)]
pub struct BiofieldCommunicator {
    last_event_time: Option<Instant>,
    min_inter_event: Duration,
    pub intents_emitted: u64,
}

impl BiofieldCommunicator {
    pub fn new(min_inter_event_ms: u64) -> Self {
        Self {
            last_event_time: None,
            min_inter_event: Duration::from_millis(min_inter_event_ms),
            intents_emitted: 0,
        }
    }

    /// Called when EEG/sEMG front-end detects a candidate event.
    /// Returns true if the event is accepted and should trigger decoding.
    pub fn try_emit_intent(&mut self, now: Instant, host: &OrganicCpuState) -> bool {
        if !host.is_within_safe_roh() || host.should_throttle() {
            return false;
        }
        if let Some(last) = self.last_event_time {
            if now.duration_since(last) < self.min_inter_event {
                return false;
            }
        }
        self.last_event_time = Some(now);
        self.intents_emitted += 1;
        true
    }
}
