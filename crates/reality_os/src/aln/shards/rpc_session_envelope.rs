use serde::{Deserialize, Serialize};
use crate::did::DidUri;
use chrono::{DateTime, Utc};

/// Allowed RPC methods for a session (append-only whitelist)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AllowedRpcMethod {
    // Read-only methods
    QueryAccount,
    QueryBalance,
    QueryTx,
    QueryBlock,
    QueryValidatorSet,
    QueryEcoMetrics,
    QueryNeuroChannel,
    // Append-only methods
    SubmitTx,
    SubmitNeuroCalibration,
    SubmitConsentGrant,
    SubmitHealthCorridor,
    // Emergency methods (always allowed if RoH permits)
    EmergencyNeuroStop,
    EmergencyImplantShutdown,
}

/// RPC Session Envelope: host-sovereign, forward-only decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcSessionEnvelope {
    /// Unique session ID (UUID v7)
    pub session_id: String,
    /// Citizen DID
    pub citizen_did: DidUri,
    /// Host device DID (phone, gateway, STM32H7 node)
    pub host_device_did: DidUri,
    /// Endpoint ID from EndpointRegistry
    pub endpoint_id: String,
    /// Allowed methods (whitelist)
    pub allowed_methods: Vec<AllowedRpcMethod>,
    /// Rate limit (requests per minute)
    pub rate_limit_per_min: u32,
    /// Energy budget for this session (millijoules)
    pub energy_budget_mj: u32,
    /// Privacy level (no raw biophysical export unless consented)
    pub privacy_level: PrivacyLevel,
    /// Session start time
    pub start_time: DateTime<Utc>,
    /// Session expiry (forward-only, cannot be extended)
    pub expiry_time: DateTime<Utc>,
    /// ROW anchor height when this envelope was committed
    pub row_anchor_height: u64,
    /// SovereigntyCore signature hash
    pub sovereignty_core_signature: String,
    /// Reason for denial (if denied)
    pub denial_reason: Option<String>,
    /// Forward-only: true if this is the active session
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivacyLevel {
    /// No biophysical data export
    NoBiophysicalExport,
    /// Aggregated/anonymized biophysical data only
    AggregatedOnly,
    /// Raw biophysical data with explicit consent
    RawWithConsent,
}

impl RpcSessionEnvelope {
    /// Check if a method is allowed in this session
    pub fn is_method_allowed(&self, method: &AllowedRpcMethod) -> bool {
        self.allowed_methods.contains(method)
    }

    /// Check if session is still valid (not expired)
    pub fn is_valid(&self) -> bool {
        self.is_active && Utc::now() < self.expiry_time
    }

    /// Check if energy budget is exceeded
    pub fn is_energy_budget_exceeded(&self, used_mj: u32) -> bool {
        used_mj >= self.energy_budget_mj
    }

    /// Create a denial envelope (forward-only, never mutates approval)
    pub fn create_denial(
        base: &RpcSessionEnvelope,
        reason: String,
        height: u64,
    ) -> RpcSessionEnvelope {
        RpcSessionEnvelope {
            session_id: base.session_id.clone(),
            citizen_did: base.citizen_did.clone(),
            host_device_did: base.host_device_did.clone(),
            endpoint_id: base.endpoint_id.clone(),
            allowed_methods: vec![],
            rate_limit_per_min: 0,
            energy_budget_mj: 0,
            privacy_level: PrivacyLevel::NoBiophysicalExport,
            start_time: base.start_time,
            expiry_time: base.start_time,
            row_anchor_height: height,
            sovereignty_core_signature: String::new(),
            denial_reason: Some(reason),
            is_active: false,
        }
    }
}

/// Session manager (append-only log of all envelopes)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RpcSessionManager {
    pub envelopes: Vec<RpcSessionEnvelope>,
    pub current_session_id: Option<String>,
}

impl RpcSessionManager {
    /// Get the current active session
    pub fn get_active_session(&self) -> Option<&RpcSessionEnvelope> {
        self.envelopes
            .iter()
            .find(|e| e.is_active && e.is_valid())
    }

    /// Append a new envelope (forward-only)
    pub fn append_envelope(&mut self, envelope: RpcSessionEnvelope) {
        // Deactivate previous sessions from same citizen+host
        for e in self.envelopes.iter_mut() {
            if e.citizen_did == envelope.citizen_did
                && e.host_device_did == envelope.host_device_did
            {
                e.is_active = false;
            }
        }
        self.envelopes.push(envelope);
    }
}
