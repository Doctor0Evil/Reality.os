use std::time::{Duration, Instant};

/// Normalized scalar in [0,1].
#[derive(Clone, Copy, Debug)]
pub struct Scalar01(f32);

impl Scalar01 {
    pub fn new(x: f32) -> Option<Self> {
        if (0.0..=1.0).contains(&x) { Some(Self(x)) } else { None }
    }
    pub fn get(self) -> f32 { self.0 }
}

/// Rolling duty-cycle over a fixed window.
#[derive(Clone, Debug)]
pub struct DutyCycle {
    window: Duration,
    active_ms: u64,
    last_tick: Instant,
    is_active: bool,
}

impl DutyCycle {
    pub fn new(window: Duration) -> Self {
        Self { window, active_ms: 0, last_tick: Instant::now(), is_active: false }
    }

    pub fn set_active(&mut self, active: bool) {
        self.tick();
        self.is_active = active;
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick);
        self.last_tick = now;
        if self.is_active {
            self.active_ms = self.active_ms.saturating_add(dt.as_millis() as u64);
            if self.active_ms > self.window.as_millis() as u64 {
                self.active_ms = self.window.as_millis() as u64;
            }
        } else {
            let w = self.window.as_millis() as u64;
            if self.active_ms > dt.as_millis() as u64 {
                self.active_ms -= dt.as_millis() as u64;
            } else {
                self.active_ms = 0;
            }
            if self.active_ms > w {
                self.active_ms = w;
            }
        }
    }

    pub fn current(&self) -> Scalar01 {
        let w = self.window.as_millis() as f32;
        let d = (self.active_ms as f32 / w).clamp(0.0, 1.0);
        Scalar01(d)
    }
}

/// Neuro safety envelope for one channel.
#[derive(Clone, Copy, Debug)]
pub struct NeuroSafetyEnvelope {
    pub d_max: Scalar01,   // max duty-cycle
    pub f_warn: Scalar01,  // fatigue warn
    pub f_stop: Scalar01,  // fatigue hard stop
    pub r_max: Scalar01,   // risk ceiling
    pub roh_max: Scalar01, // RoH ceiling (e.g., 0.3)
}

/// Live host budget snapshot (CPU-only view).
#[derive(Clone, Copy, Debug)]
pub struct HostBudgetSnapshot {
    pub cpu_util: Scalar01,       // normalized CPU load
    pub eco_impact: Scalar01,     // EcoImpactScore
    pub fatigue_index: Scalar01,  // current fatigue
    pub risk_index: Scalar01,     // modeled risk
    pub roh: Scalar01,            // latency / target
}

/// Admission control decision.
pub enum ChannelDecision {
    Admit,
    Throttle,
    Block,
}

pub fn admit_channel(
    env: &NeuroSafetyEnvelope,
    duty: Scalar01,
    host: &HostBudgetSnapshot,
) -> ChannelDecision {
    if host.roh.get() > env.roh_max.get()
        || duty.get() > env.d_max.get()
        || host.fatigue_index.get() > env.f_stop.get()
        || host.risk_index.get() > env.r_max.get()
    {
        ChannelDecision::Block
    } else if host.fatigue_index.get() > env.f_warn.get() {
        ChannelDecision::Throttle
    } else {
        ChannelDecision::Admit
    }
}
