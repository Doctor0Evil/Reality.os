use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

// ---------- Core shared types ----------

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SleepStage {
    Wake,
    N1,
    N2,
    N3,
    Rem,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PsychRiskBand {
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IdentityMassBand {
    Light,
    Medium,
    Heavy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ImaginationBoundsMode {
    IdentityMatch,
    LockedLow,
    LockedMed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DreamActionType {
    ZoneTransition,
    HapticsSoften,
    SafeTeleport,
}

#[derive(Clone, Debug)]
pub struct ConsciousnessState {
    pub subject_id: String,
    pub sleep_stage: SleepStage,
    pub psych_risk_band: PsychRiskBand,
    pub psych_risk_score: f32, // [0,1]
    pub enstasis_score: f32,   // [0,1]
    pub safety_mode: bool,
    pub recording_enabled: bool,
    pub deep_sleep_token: f32, // [0,1]
}

impl ConsciousnessState {
    pub fn new(subject_id: impl Into<String>) -> Self {
        ConsciousnessState {
            subject_id: subject_id.into(),
            sleep_stage: SleepStage::Unknown,
            psych_risk_band: PsychRiskBand::Low,
            psych_risk_score: 0.0,
            enstasis_score: 1.0,
            safety_mode: false,
            recording_enabled: false,
            deep_sleep_token: 0.0,
        }
    }
}

// ---------- Neurorights & policy flags ----------

#[derive(Clone, Debug)]
pub struct NeurorightsFlags {
    pub mental_privacy: bool,
    pub cognitive_liberty: bool,
    pub mental_integrity: bool,
    pub noncommercial_neural_data: bool,
    pub no_punitive_xr: bool,
}

impl NeurorightsFlags {
    pub fn all_on() -> Self {
        NeurorightsFlags {
            mental_privacy: true,
            cognitive_liberty: true,
            mental_integrity: true,
            noncommercial_neural_data: true,
            no_punitive_xr: true,
        }
    }

    pub fn all_valid(&self) -> bool {
        self.mental_privacy
            && self.cognitive_liberty
            && self.mentalintegrity()
            && self.noncommercial_neural_data
            && self.no_punitive_xr
    }

    fn mentalintegrity(&self) -> bool {
        self.mental_integrity
    }
}

#[derive(Clone, Debug)]
pub struct SessionPolicySurface {
    pub neurorights_flags: NeurorightsFlags,
    pub max_haptics_low_arousal: f32,
    pub max_haptics_rem: f32,
    pub enable_dream_logging: bool,
    pub enable_redacted_audit: bool,
}

impl SessionPolicySurface {
    pub fn default() -> Self {
        SessionPolicySurface {
            neurorights_flags: NeurorightsFlags::all_on(),
            max_haptics_low_arousal: 0.3,
            max_haptics_rem: 0.2,
            enable_dream_logging: true,
            enable_redacted_audit: true,
        }
    }
}

// ---------- 1. Dream locus totem ----------

#[derive(Clone, Debug)]
pub struct DreamLocusTotem {
    pub public_subject_id: String,
    pub public_node_id: String,
    pub neurorights_flags: NeurorightsFlags,
}

impl DreamLocusTotem {
    pub fn new(
        public_subject_id: impl Into<String>,
        public_node_id: impl Into<String>,
        flags: NeurorightsFlags,
    ) -> Self {
        DreamLocusTotem {
            public_subject_id: public_subject_id.into(),
            public_node_id: public_node_id.into(),
            neurorights_flags: flags,
        }
    }

    /// Render-safe info: never secrets, balances, or dream content.
    pub fn public_descriptor(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("subject_id".into(), self.public_subject_id.clone());
        map.insert("node_id".into(), self.public_node_id.clone());
        map.insert(
            "neurorights_mental_privacy".into(),
            self.neurorights_flags.mental_privacy.to_string(),
        );
        map.insert(
            "neurorights_cognitive_liberty".into(),
            self.neurorights_flags.cognitive_liberty.to_string(),
        );
        map.insert(
            "neurorights_mental_integrity".into(),
            self.neurorights_flags.mental_integrity.to_string(),
        );
        map.insert(
            "neurorights_noncommercial_neural_data".into(),
            self.neurorights_flags.noncommercial_neural_data.to_string(),
        );
        map.insert(
            "neurorights_no_punitive_xr".into(),
            self.neurorights_flags.no_punitive_xr.to_string(),
        );
        map
    }
}

// ---------- 2. Lucid anchor stone ----------

#[derive(Clone, Debug)]
pub struct LucidAnchorStoneConfig {
    pub max_psych_risk_for_manual_anchor: f32,
    pub auto_anchor_psych_risk_threshold: f32,
    pub allowed_sleep_stages: Vec<SleepStage>,
}

#[derive(Clone, Debug)]
pub struct LucidAnchorStone {
    pub config: LucidAnchorStoneConfig,
}

impl LucidAnchorStone {
    pub fn default() -> Self {
        LucidAnchorStone {
            config: LucidAnchorStoneConfig {
                max_psych_risk_for_manual_anchor: 0.8,
                auto_anchor_psych_risk_threshold: 0.9,
                allowed_sleep_stages: vec![SleepStage::N2, SleepStage::N3, SleepStage::Rem],
            },
        }
    }

    /// Pure state decision: should we jump to a safe scene?
    pub fn should_trigger_safe_jump(&self, state: &ConsciousnessState) -> bool {
        let stage_ok = self
            .config
            .allowed_sleep_stages
            .iter()
            .any(|s| *s == state.sleep_stage);
        stage_ok && state.psych_risk_score >= self.config.auto_anchor_psych_risk_threshold
    }
}

// ---------- 3. Safe room capsule ----------

#[derive(Clone, Debug)]
pub struct SafeRoomCapsule {
    pub max_luminance: f32,
    pub max_sound_level: f32,
    pub max_haptics_intensity: f32,
    pub allow_experiments: bool,
    pub allow_agentic_ai: bool,
}

impl SafeRoomCapsule {
    pub fn default_low_arousal() -> Self {
        SafeRoomCapsule {
            max_luminance: 0.2,
            max_sound_level: 0.2,
            max_haptics_intensity: 0.2,
            allow_experiments: false,
            allow_agentic_ai: false,
        }
    }

    pub fn clamp_haptics(&self, requested: f32) -> f32 {
        requested.min(self.max_haptics_intensity).max(0.0)
    }
}

// ---------- 4. Memory corridor key ----------

#[derive(Clone, Debug)]
pub struct MemoryCorridorKey {
    pub key_id: String,
    pub corridor_root_hash: String,
    pub allowed_event_kinds: Vec<DreamActionType>,
}

impl MemoryCorridorKey {
    pub fn new(
        key_id: impl Into<String>,
        corridor_root_hash: impl Into<String>,
        allowed_event_kinds: Vec<DreamActionType>,
    ) -> Self {
        MemoryCorridorKey {
            key_id: key_id.into(),
            corridor_root_hash: corridor_root_hash.into(),
            allowed_event_kinds,
        }
    }

    pub fn can_view_event_type(&self, t: DreamActionType) -> bool {
        self.allowed_event_kinds.contains(&t)
    }
}

// ---------- 5. Dream passage arch ----------

#[derive(Clone, Debug)]
pub struct DreamPassageArch {
    pub required_consent_flag: bool,
    pub max_psych_risk_for_crossing: f32,
    pub allowed_sleep_stages: Vec<SleepStage>,
}

impl DreamPassageArch {
    pub fn default() -> Self {
        DreamPassageArch {
            required_consent_flag: true,
            max_psych_risk_for_crossing: 0.6,
            allowed_sleep_stages: vec![SleepStage::N2, SleepStage::N3, SleepStage::Rem],
        }
    }

    pub fn can_cross(
        &self,
        has_consent: bool,
        state: &ConsciousnessState,
    ) -> bool {
        if self.required_consent_flag && !has_consent {
            return false;
        }
        if state.psych_risk_score > self.max_psych_risk_for_crossing {
            return false;
        }
        self.allowed_sleep_stages
            .iter()
            .any(|s| *s == state.sleep_stage)
    }
}

// ---------- 6. Dreamweave anchor thread ----------

#[derive(Clone, Debug)]
pub struct DreamweaveAnchorThread {
    pub scene_id: String,
    pub session_id: String,
    pub safety_event_ids: Vec<String>,
}

impl DreamweaveAnchorThread {
    pub fn new(scene_id: impl Into<String>, session_id: impl Into<String>) -> Self {
        DreamweaveAnchorThread {
            scene_id: scene_id.into(),
            session_id: session_id.into(),
            safety_event_ids: Vec::new(),
        }
    }

    pub fn bind_safety_event(&mut self, event_id: impl Into<String>) {
        self.safety_event_ids.push(event_id.into());
    }
}

// ---------- 7. Safe-zone waypoint beacon ----------

#[derive(Clone, Debug)]
pub struct SafeZoneWaypointBeacon {
    pub safe_scene_id: String,
    pub auto_teleport_psych_risk_threshold: f32,
    pub teleport_latency_ms: u64,
}

impl SafeZoneWaypointBeacon {
    pub fn default(safe_scene_id: impl Into<String>) -> Self {
        SafeZoneWaypointBeacon {
            safe_scene_id: safe_scene_id.into(),
            auto_teleport_psych_risk_threshold: 0.85,
            teleport_latency_ms: 1500,
        }
    }

    pub fn should_auto_teleport(&self, state: &ConsciousnessState) -> bool {
        state.psych_risk_score >= self.auto_teleport_psych_risk_threshold
    }
}

// ---------- 8. Dream ledger obelisk ----------

#[derive(Clone, Debug)]
pub struct DreamActionLedgerEvent {
    pub event_id: String,
    pub action_type: DreamActionType,
    pub subject_id: String,
    pub from_zone_id: Option<String>,
    pub to_zone_id: Option<String>,
    pub psych_risk_band: PsychRiskBand,
    pub sleep_stage: SleepStage,
    pub neurorights_invariants_held: bool,
    pub timestamp_unix_ms: u128,
}

impl DreamActionLedgerEvent {
    pub fn new(
        action_type: DreamActionType,
        state: &ConsciousnessState,
        from_zone_id: Option<String>,
        to_zone_id: Option<String>,
        neurorights_invariants_held: bool,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let eid = format!(
            "dre-{}-{:x}",
            state.subject_id,
            now
        );
        DreamActionLedgerEvent {
            event_id: eid,
            action_type,
            subject_id: state.subject_id.clone(),
            from_zone_id,
            to_zone_id,
            psych_risk_band: state.psych_risk_band,
            sleep_stage: state.sleep_stage,
            neurorights_invariants_held,
            timestamp_unix_ms: now,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DreamLedgerObelisk {
    pub scene_id: String,
    pub events: Vec<DreamActionLedgerEvent>,
}

impl DreamLedgerObelisk {
    pub fn new(scene_id: impl Into<String>) -> Self {
        DreamLedgerObelisk {
            scene_id: scene_id.into(),
            events: Vec::new(),
        }
    }

    pub fn append(&mut self, event: DreamActionLedgerEvent) {
        self.events.push(event);
    }

    pub fn redacted_view(&self) -> Vec<HashMap<String, String>> {
        self.events
            .iter()
            .map(|e| {
                let mut m = HashMap::new();
                m.insert("event_id".into(), e.event_id.clone());
                m.insert(
                    "action_type".into(),
                    match e.action_type {
                        DreamActionType::ZoneTransition => "zone-transition".to_string(),
                        DreamActionType::HapticsSoften => "haptics-soften".to_string(),
                        DreamActionType::SafeTeleport => "safe-teleport".to_string(),
                    },
                );
                m.insert("subject_id".into(), e.subject_id.clone());
                m.insert("sleep_stage".into(), format!("{:?}", e.sleep_stage));
                m.insert(
                    "psych_risk_band".into(),
                    format!("{:?}", e.psych_risk_band),
                );
                m
            })
            .collect()
    }
}

// ---------- 9. Sleep-stage sigil ring ----------

#[derive(Clone, Debug)]
pub struct SleepStageSigilRing {
    pub last_update_ms: u128,
    pub latency_bound_ms: u128,
}

impl SleepStageSigilRing {
    pub fn new(latency_bound_ms: u128) -> Self {
        SleepStageSigilRing {
            last_update_ms: 0,
            latency_bound_ms,
        }
    }

    pub fn should_refresh(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        now.saturating_sub(self.last_update_ms) >= self.latency_bound_ms
    }
}

// ---------- 10. Psych-risk gauge orb ----------

#[derive(Clone, Debug)]
pub struct PsychRiskGaugeOrb {
    pub soften_threshold: f32,
    pub pause_threshold: f32,
}

impl PsychRiskGaugeOrb {
    pub fn default() -> Self {
        PsychRiskGaugeOrb {
            soften_threshold: 0.5,
            pause_threshold: 0.8,
        }
    }

    pub fn haptic_soften_factor(&self, risk_score: f32) -> f32 {
        if risk_score >= self.pause_threshold {
            0.0
        } else if risk_score >= self.soften_threshold {
            0.4
        } else {
            1.0
        }
    }
}

// ---------- 11. Governance rule tablet ----------

#[derive(Clone, Debug)]
pub struct GovernanceRuleTablet {
    pub session_policy_surface: SessionPolicySurface,
}

impl GovernanceRuleTablet {
    pub fn new(surface: SessionPolicySurface) -> Self {
        GovernanceRuleTablet {
            session_policy_surface: surface,
        }
    }

    pub fn public_rules_view(&self) -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert(
            "mental_privacy".into(),
            self.session_policy_surface
                .neurorights_flags
                .mental_privacy
                .to_string(),
        );
        m.insert(
            "cognitive_liberty".into(),
            self.session_policy_surface
                .neurorights_flags
                .cognitive_liberty
                .to_string(),
        );
        m.insert(
            "no_punitive_xr".into(),
            self.session_policy_surface
                .neurorights_flags
                .no_punitive_xr
                .to_string(),
        );
        m.insert(
            "max_haptics_low_arousal".into(),
            format!(
                "{:.3}",
                self.session_policy_surface.max_haptics_low_arousal
            ),
        );
        m.insert(
            "max_haptics_rem".into(),
            format!("{:.3}", self.session_policy_surface.max_haptics_rem),
        );
        m
    }
}

// ---------- 12. Neurorights seal sigil ----------

#[derive(Clone, Debug)]
pub struct NeurorightsSealSigil {
    pub flags: NeurorightsFlags,
}

impl NeurorightsSealSigil {
    pub fn new(flags: NeurorightsFlags) -> Self {
        NeurorightsSealSigil { flags }
    }

    pub fn can_start_session(&self) -> bool {
        self.flags.all_valid()
    }
}

// ---------- 13. Session-status HUD band ----------

#[derive(Clone, Debug)]
pub struct SessionStatusHudBand {
    pub last_render_ms: u128,
    pub max_latency_ms: u128,
}

impl SessionStatusHudBand {
    pub fn new(max_latency_ms: u128) -> Self {
        SessionStatusHudBand {
            last_render_ms: 0,
            max_latency_ms,
        }
    }

    pub fn should_update(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        now.saturating_sub(self.last_render_ms) >= self.max_latency_ms
    }

    pub fn encode_status(&self, state: &ConsciousnessState) -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("subject_id".into(), state.subject_id.clone());
        m.insert("sleep_stage".into(), format!("{:?}", state.sleep_stage));
        m.insert(
            "psych_risk_band".into(),
            format!("{:?}", state.psych_risk_band),
        );
        m.insert(
            "safety_mode".into(),
            state.safety_mode.to_string(),
        );
        m.insert(
            "recording".into(),
            state.recording_enabled.to_string(),
        );
        m
    }
}

// ---------- 14. Stage-gated XR gate ----------

#[derive(Clone, Debug)]
pub struct StageGatedXrGate {
    pub min_deep_sleep_token: f32,
    pub min_enstasis_score: f32,
}

impl StageGatedXrGate {
    pub fn default() -> Self {
        StageGatedXrGate {
            min_deep_sleep_token: 0.5,
            min_enstasis_score: 0.5,
        }
    }

    pub fn enabled(&self, state: &ConsciousnessState) -> bool {
        let deep_ok = state.deep_sleep_token >= self.min_deep_sleep_token;
        let es_ok = state.enstasis_score >= self.min_enstasis_score;
        deep_ok && es_ok
    }
}

// ---------- 15. Psychrisk safety envelope ----------

#[derive(Clone, Debug)]
pub struct PsychriskSafetyEnvelope {
    pub max_intensity_low: f32,
    pub max_intensity_high: f32,
}

impl PsychriskSafetyEnvelope {
    pub fn default() -> Self {
        PsychriskSafetyEnvelope {
            max_intensity_low: 0.6,
            max_intensity_high: 0.3,
        }
    }

    pub fn clamp_intensity(&self, risk_score: f32, requested: f32) -> f32 {
        let limit = if risk_score < 0.5 {
            self.max_intensity_low
        } else {
            self.max_intensity_high
        };
        requested.min(limit).max(0.0)
    }
}

// ---------- 16. Identity-mass band ----------

#[derive(Clone, Debug)]
pub struct IdentityMassProfile {
    pub base_band: IdentityMassBand,
}

impl IdentityMassProfile {
    pub fn effective_band(
        &self,
        mode: ImaginationBoundsMode,
    ) -> IdentityMassBand {
        match mode {
            ImaginationBoundsMode::IdentityMatch => self.base_band,
            ImaginationBoundsMode::LockedLow => IdentityMassBand::Light,
            ImaginationBoundsMode::LockedMed => IdentityMassBand::Medium,
        }
    }
}

// ---------- 17. Imagination bounds mode ----------

#[derive(Clone, Debug)]
pub struct ImaginationBoundsProfile {
    pub mode: ImaginationBoundsMode,
}

// ---------- 18. Solid haptics envelope ----------

#[derive(Clone, Debug)]
pub struct SolidHapticsEnvelope {
    pub low_arousal_max: f32,
    pub rem_max: f32,
}

impl SolidHapticsEnvelope {
    pub fn default() -> Self {
        SolidHapticsEnvelope {
            low_arousal_max: 0.3,
            rem_max: 0.2,
        }
    }

    pub fn effective_intensity(
        &self,
        band: IdentityMassBand,
        state: &ConsciousnessState,
        requested: f32,
    ) -> f32 {
        let base_factor = match band {
            IdentityMassBand::Light => 0.4,
            IdentityMassBand::Medium => 0.7,
            IdentityMassBand::Heavy => 1.0,
        };

        let mut intensity = requested * base_factor;

        intensity = match state.sleep_stage {
            SleepStage::N2 | SleepStage::N3 => intensity.min(self.low_arousal_max),
            SleepStage::Rem => intensity.min(self.rem_max),
            _ => intensity.min(1.0),
        };

        intensity.max(0.0)
    }
}

// ---------- 19. Height-latency calibrator ----------

#[derive(Clone, Debug)]
pub struct HeightLatencyCalibrator {
    pub framerate_hz: f32,
    pub latency_ms: f32,
    pub frame_offset: i32,
}

impl HeightLatencyCalibrator {
    pub fn new(framerate_hz: f32, latency_ms: f32, frame_offset: i32) -> Self {
        HeightLatencyCalibrator {
            framerate_hz,
            latency_ms,
            frame_offset,
        }
    }

    pub fn normalized_elevation(&self, raw_height_m: f32) -> f32 {
        let latency_factor = (self.latency_ms / 50.0).clamp(0.5, 1.5);
        raw_height_m / latency_factor
    }
}

// ---------- 20. Object respawn budget ----------

#[derive(Clone, Debug)]
pub struct ObjectRespawnBudget {
    pub max_impulse_budget: f32,
    pub accumulated_impulse: f32,
}

impl ObjectRespawnBudget {
    pub fn new(max_impulse_budget: f32) -> Self {
        ObjectRespawnBudget {
            max_impulse_budget,
            accumulated_impulse: 0.0,
        }
    }

    pub fn add_impulse(&mut self, impulse: f32) -> bool {
        self.accumulated_impulse += impulse.max(0.0);
        self.accumulated_impulse >= self.max_impulse_budget
    }

    pub fn reset(&mut self) {
        self.accumulated_impulse = 0.0;
    }
}

// ---------- 21. Panic-teleport anchor ----------

#[derive(Clone, Debug)]
pub struct PanicTeleportAnchor {
    pub target_safe_scene_id: String,
    pub max_teleport_time_ms: u64,
    pub psych_risk_trigger: f32,
}

impl PanicTeleportAnchor {
    pub fn new(target_safe_scene_id: impl Into<String>) -> Self {
        PanicTeleportAnchor {
            target_safe_scene_id: target_safe_scene_id.into(),
            max_teleport_time_ms: 2000,
            psych_risk_trigger: 0.9,
        }
    }

    pub fn should_teleport(
        &self,
        panic_gesture_detected: bool,
        state: &ConsciousnessState,
    ) -> bool {
        panic_gesture_detected || state.psych_risk_score >= self.psych_risk_trigger
    }
}

// ---------- 22. Doorway frame ----------

#[derive(Clone, Debug)]
pub struct DoorwayFrame {
    pub from_zone_id: String,
    pub to_zone_id: String,
    pub requires_consent: bool,
}

impl DoorwayFrame {
    pub fn new(from_zone_id: impl Into<String>, to_zone_id: impl Into<String>) -> Self {
        DoorwayFrame {
            from_zone_id: from_zone_id.into(),
            to_zone_id: to_zone_id.into(),
            requires_consent: true,
        }
    }

    pub fn can_transition(
        &self,
        state: &ConsciousnessState,
        has_consent: bool,
    ) -> bool {
        if self.requires_consent && !has_consent {
            return false;
        }
        !matches!(state.sleep_stage, SleepStage::Wake)
    }
}

// ---------- 23. Window parallax viewport ----------

#[derive(Clone, Debug)]
pub struct WindowParallaxViewport {
    pub target_scene_id: String,
}

impl WindowParallaxViewport {
    pub fn new(target_scene_id: impl Into<String>) -> Self {
        WindowParallaxViewport {
            target_scene_id: target_scene_id.into(),
        }
    }

    /// Render-only: always blocks input.
    pub fn accepts_input(&self) -> bool {
        false
    }
}

// ---------- 24. Locked gateway ----------

#[derive(Clone, Debug)]
pub struct LockedGateway {
    pub privileged_zone_id: String,
    pub required_capability_token: String,
    pub min_sleep_reserve: f32,
    pub max_psych_risk: f32,
}

impl LockedGateway {
    pub fn new(
        privileged_zone_id: impl Into<String>,
        required_capability_token: impl Into<String>,
    ) -> Self {
        LockedGateway {
            privileged_zone_id: privileged_zone_id.into(),
            required_capability_token: required_capability_token.into(),
            min_sleep_reserve: 0.5,
            max_psych_risk: 0.5,
        }
    }

    pub fn can_enter(
        &self,
        state: &ConsciousnessState,
        supplied_token: &str,
        neurorights: &NeurorightsFlags,
    ) -> bool {
        if supplied_token != self.required_capability_token {
            return false;
        }
        if !neurorights.no_punitive_xr {
            return false;
        }
        if state.enstasis_score < self.min_sleep_reserve {
            return false;
        }
        if state.psych_risk_score > self.max_psych_risk {
            return false;
        }
        true
    }
}

// ---------- 25. Dream action ledger event (typed) ----------

#[derive(Clone, Debug)]
pub struct DreamActionLedger {
    pub events: Vec<DreamActionLedgerEvent>,
}

impl DreamActionLedger {
    pub fn new() -> Self {
        DreamActionLedger { events: Vec::new() }
    }

    pub fn log_event(&mut self, event: DreamActionLedgerEvent) {
        self.events.push(event);
    }
}

// ---------- Example glue for Reality.os ----------

pub struct RealityOsDreamPrimitives {
    pub safe_room_capsule: SafeRoomCapsule,
    pub lucid_anchor: LucidAnchorStone,
    pub panic_anchor: PanicTeleportAnchor,
    pub solid_haptics: SolidHapticsEnvelope,
    pub safety_envelope: PsychriskSafetyEnvelope,
}

impl RealityOsDreamPrimitives {
    pub fn new(safe_scene_id: impl Into<String>) -> Self {
        RealityOsDreamPrimitives {
            safe_room_capsule: SafeRoomCapsule::default_low_arousal(),
            lucid_anchor: LucidAnchorStone::default(),
            panic_anchor: PanicTeleportAnchor::new(safe_scene_id),
            solid_haptics: SolidHapticsEnvelope::default(),
            safety_envelope: PsychriskSafetyEnvelope::default(),
        }
    }

    /// Combined haptic clamp: safe-room, psychrisk envelope, and solid envelope.
    pub fn clamp_haptics_chain(
        &self,
        band: IdentityMassBand,
        state: &ConsciousnessState,
        requested: f32,
        in_safe_room: bool,
    ) -> f32 {
        let base = if in_safe_room {
            self.safe_room_capsule.clamp_haptics(requested)
        } else {
            requested
        };
        let env_clamped = self.safety_envelope.clamp_intensity(state.psych_risk_score, base);
        self.solid_haptics.effective_intensity(band, state, env_clamped)
    }

    /// Decide if any safety primitive should teleport to safe zone.
    pub fn should_route_to_safe_zone(
        &self,
        panic_gesture: bool,
        state: &ConsciousnessState,
    ) -> bool {
        self.panic_anchor
            .should_teleport(panic_gesture, state)
            || self.lucid_anchor
                .should_trigger_safe_jump(state)
    }
}

fn main() {
    // Example wiring for local testing.
    let mut state = ConsciousnessState::new("subject-xyz");
    state.sleep_stage = SleepStage::N2;
    state.psych_risk_score = 0.7;
    state.psych_risk_band = PsychRiskBand::High;
    state.enstasis_score = 0.8;
    state.deep_sleep_token = 0.9;
    state.safety_mode = true;

    let primitives = RealityOsDreamPrimitives::new("scene.safe-room.global");

    let requested_force = 0.9;
    let clamped_force = primitives.clamp_haptics_chain(
        IdentityMassBand::Medium,
        &state,
        requested_force,
        false,
    );
    println!("Requested haptics: {}, clamped: {}", requested_force, clamped_force);

    let teleport = primitives.should_route_to_safe_zone(true, &state);
    println!("Should route to safe zone: {}", teleport);
}
