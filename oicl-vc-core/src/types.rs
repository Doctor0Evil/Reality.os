#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JurisdictionCode(pub String); // e.g., "US-AZ-PHX"

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessibilityRole {
    None,
    LowVision,
    LowMobility,
    Neurodivergent,
    HearingImpaired,
    MultiModal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteractionMode {
    Voice,
    Text,
    Gaze,
    Gesture,
    Hybrid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcoPreference {
    Neutral,
    EcoPriority,
    TimePriority,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceRole {
    CivicTerminal,
    TransitHub,
    EmergencyService,
    PersonalDevice,
    AdTech,
    NonCivicProfiling,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OfflineServiceClass {
    EssentialRoute,
    HealthAccess,
    CivicBasic,
    EcoLeisure,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OfflineReconciliationPolicy {
    LocalOnly,
    DeferredSync,
    ImmediateSyncWhenAvailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProhibitedDataUse {
    NoAffectiveAds,
    NoBehavioralScoring,
    NoCrossContextTracking,
}
