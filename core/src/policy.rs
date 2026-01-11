pub mod bill_of_rights;

use crate::events::LedgerEvent;
use crate::karma::KarmaEngine;
use crate::types::{
    EnvironmentView, Identity, IdentityFlags, IdentityState, IdentityStatus, NetworkView,
    PlatformView,
};

pub struct MultiViewEngine;

pub struct IncidentDecision {
    pub block_negative_trust_update: bool,
    pub route_to_review: bool,
    pub apply_negative_update: bool,
}

impl MultiViewEngine {
    pub fn evaluate(
        network: &NetworkView,
        platform: &PlatformView,
        env: &EnvironmentView,
        identity: &Identity,
        flags: &IdentityFlags,
        status: &mut IdentityStatus,
    ) -> (IncidentDecision, Option<LedgerEvent>) {
        let network_anomaly = network.ip_anomaly || network.device_anomaly || network.score > 0.7;
        let platform_anomaly = platform.bulk_delete
            || platform.abnormal_logins
            || platform.automation_abuse
            || platform.score > 0.7;
        let environmental_harm = env.delta_pollution > 0.0 && env.score > 0.5;

        // If inconsistent (network + platform, but no environmental harm), assume possible false positive or simulation.
        if network_anomaly && platform_anomaly && !environmental_harm {
            let ev = KarmaEngine::transition_state(status, IdentityState::UnderReview, "Inconsistent anomalies across views");
            return (
                IncidentDecision {
                    block_negative_trust_update: true,
                    route_to_review: true,
                    apply_negative_update: false,
                },
                Some(ev),
            );
        }

        if environmental_harm {
            let ev = KarmaEngine::transition_state(status, IdentityState::UnderReview, "Confirmed environmental harm");
            return (
                IncidentDecision {
                    block_negative_trust_update: false,
                    route_to_review: true,
                    apply_negative_update: true,
                },
                Some(ev),
            );
        }

        (
            IncidentDecision {
                block_negative_trust_update: false,
                route_to_review: false,
                apply_negative_update: false,
            },
            None,
        )
    }

    pub fn detect_under_attack_pattern(
        flags: &IdentityFlags,
        network: &NetworkView,
        platform: &PlatformView,
    ) -> bool {
        if !flags.neuro_linked {
            return false;
        }
        // Heuristic: frequent IP swaps + abnormal AR/VR automation (encoded as automation_abuse).
        (network.ip_anomaly || network.device_anomaly) && platform.automation_abuse
    }
}
