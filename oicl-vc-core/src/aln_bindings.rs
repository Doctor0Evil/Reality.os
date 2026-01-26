use crate::types::{
    DeviceRole,
    JurisdictionCode,
    OfflineServiceClass,
    ProhibitedDataUse,
};
use crate::vc::{
    NeurorightsEnvelopeCredential,
    OfflineCompanionRightsCredential,
    NeurospaceRadiusCredential,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OfflineCompanionShard {
    pub right_to_offline_companion: bool, // righttoofflinecompanion!
    pub allow_essential_route: bool,
    pub allow_health_access: bool,
    pub allow_civic_basic: bool,
    pub allow_eco_leisure: bool,
}

impl From<&OfflineCompanionRightsCredential> for OfflineCompanionShard {
    fn from(vc: &OfflineCompanionRightsCredential) -> Self {
        let mut allow_essential = false;
        let mut allow_health = false;
        let mut allow_civic = false;
        let mut allow_eco = false;

        for c in &vc.offline_mode_permissions {
            match c {
                OfflineServiceClass::EssentialRoute => allow_essential = true,
                OfflineServiceClass::HealthAccess => allow_health = true,
                OfflineServiceClass::CivicBasic => allow_civic = true,
                OfflineServiceClass::EcoLeisure => allow_eco = true,
            }
        }

        Self {
            right_to_offline_companion: vc.right_to_offline_companion,
            allow_essential_route: allow_essential,
            allow_health_access: allow_health,
            allow_civic_basic: allow_civic,
            allow_eco_leisure: allow_eco,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NeurospaceRadiusShard {
    pub radius_meters: f32,               // neurospaceradius!meters
    pub permitted_device_roles: Vec<DeviceRole>,
    pub blocked_device_roles: Vec<DeviceRole>,
    pub jurisdiction: JurisdictionCode,
}

impl NeurospaceRadiusShard {
    pub fn from_vc(vc: &NeurospaceRadiusCredential, jurisdiction: JurisdictionCode) -> Self {
        Self {
            radius_meters: vc.radius_meters,
            permitted_device_roles: vc.permitted_device_roles.clone(),
            blocked_device_roles: vc.blocked_device_roles.clone(),
            jurisdiction,
        }
    }

    pub fn allow_device(&self, role: DeviceRole, jurisdiction: &JurisdictionCode, distance_m: f32) -> bool {
        if distance_m > self.radius_meters {
            return false;
        }
        if self.blocked_device_roles.contains(&role) {
            return false;
        }
        let jurisdiction_ok = jurisdiction.0.starts_with("US");
        let role_ok = self.permitted_device_roles.contains(&role);
        jurisdiction_ok && role_ok
    }
}

#[derive(Clone, Debug)]
pub struct NeurorightsShard {
    pub envelope_id: String,
    pub max_cognitive_load: f32,    // maxcognitiveload!
    pub max_legal_complexity: f32,  // maxlegalcomplexity!
    pub max_duty_cycle: f32,
    pub allow_affective_influence: bool,
    pub ban_affective_targeting: bool, // banaffectivetargeting!
}

impl From<&NeurorightsEnvelopeCredential> for NeurorightsShard {
    fn from(vc: &NeurorightsEnvelopeCredential) -> Self {
        let ban_affective = vc
            .prohibited_data_uses
            .iter()
            .any(|u| matches!(u, ProhibitedDataUse::NoAffectiveAds));

        Self {
            envelope_id: vc.envelope_id.clone(),
            max_cognitive_load: vc.max_cognitive_load,
            max_legal_complexity: vc.max_legal_complexity,
            max_duty_cycle: vc.max_duty_cycle,
            allow_affective_influence: vc.allow_affective_influence,
            ban_affective_targeting: ban_affective,
        }
    }
}
