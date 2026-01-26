use crate::did::OiclDid;
use crate::types::{
    AccessibilityRole,
    InteractionMode,
    EcoPreference,
    DeviceRole,
    JurisdictionCode,
    OfflineServiceClass,
    OfflineReconciliationPolicy,
    ProhibitedDataUse,
};

#[derive(Clone, Debug)]
pub struct AugmentedCitizenProfileCredential {
    pub host_did: OiclDid,
    pub jurisdiction: JurisdictionCode,
    pub accessibility_role: AccessibilityRole,
    pub interaction_mode_preference: InteractionMode,
    pub non_exclusion_flag: bool,
    pub eco_preference: EcoPreference,
}

#[derive(Clone, Debug)]
pub struct NeurorightsEnvelopeCredential {
    pub envelope_id: String,
    pub max_cognitive_load: f32,    // 0.0–1.0
    pub max_legal_complexity: f32,  // 0.0–1.0
    pub max_duty_cycle: f32,        // 0.0–1.0
    pub allow_affective_influence: bool,
    pub prohibited_data_uses: Vec<ProhibitedDataUse>,
    pub jurisdiction: JurisdictionCode,
}

#[derive(Clone, Debug)]
pub struct OfflineCompanionRightsCredential {
    pub right_to_offline_companion: bool,
    pub offline_mode_permissions: Vec<OfflineServiceClass>,
    pub reconciliation_policy: OfflineReconciliationPolicy,
}

#[derive(Clone, Debug)]
pub struct NeurospaceRadiusCredential {
    pub radius_meters: f32,
    pub permitted_device_roles: Vec<DeviceRole>,
    pub blocked_device_roles: Vec<DeviceRole>,
    pub discovery_policy: DiscoveryPolicy,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiscoveryPolicy {
    RadiusOnly,
    RadiusPlusLineOfSight,
    ManualPairing,
}
