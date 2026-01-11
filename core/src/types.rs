use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IdentityType {
    Human,
    AiAgent,
    GithubOrg,
    DeviceCluster,
    AugmentedCitizen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityFlags {
    pub neuro_linked: bool,
    pub data_sensitivity_high: bool,
    pub under_attack_risk: bool,
    pub cognitive_safety_required: bool,
    pub protected: bool,
}

impl Default for IdentityFlags {
    fn default() -> Self {
        Self {
            neuro_linked: false,
            data_sensitivity_high: false,
            under_attack_risk: false,
            cognitive_safety_required: false,
            protected: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub id: Uuid,
    pub identity_type: IdentityType,
    pub flags: IdentityFlags,
    /// eco impact score ∈ [0,1]
    pub eco_impact_score: f64,
    /// current karma ∈ [0,1]
    pub current_karma: f64,
    /// security trust score ∈ [0,1]
    pub security_trust_score: f64,
    /// contribution score (open + private)
    pub contribution_score: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlatformKind {
    Github,
    AiChat,
    IoTCluster,
    InternalResearch,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Platform {
    pub id: Uuid,
    pub name: String,
    pub kind: PlatformKind,
    /// dynamic platform trust score ∈ [0,1]
    pub trust_score: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionKind {
    Allow,
    Deny,
    Throttle,
    PenalizeKarma(f64),
    AdjustTrust(f64),
    RouteToReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessDecisionLog {
    pub id: Uuid,
    pub platform_id: Uuid,
    pub identity_id: Uuid,
    pub decision: DecisionKind,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkView {
    pub identity_id: Uuid,
    pub ip_anomaly: bool,
    pub device_anomaly: bool,
    pub score: f64,
    pub observed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformView {
    pub identity_id: Uuid,
    pub bulk_delete: bool,
    pub abnormal_logins: bool,
    pub automation_abuse: bool,
    pub score: f64,
    pub observed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentView {
    pub identity_id: Uuid,
    /// Δ pollution load (e.g. PFBS tons)
    pub delta_pollution: f64,
    pub evidence_hash: String,
    pub score: f64,
    pub observed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityState {
    Normal,
    UnderReview,
    UnderAttack,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityStatus {
    pub identity_id: Uuid,
    pub state: IdentityState,
    pub last_transition: DateTime<Utc>,
}
