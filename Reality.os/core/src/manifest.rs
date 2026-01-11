use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityManifest {
    pub id: Uuid,
    pub version: String,
    pub projects: Vec<RealityProject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityProject {
    pub name: String,
    pub description: String,
    pub kind: super::ProjectKind,
    pub repos: Vec<RealityRepo>,
    pub targets: Vec<RealityTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityRepo {
    pub name: String,
    pub url: String,
    pub branch: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityTarget {
    pub name: String,
    pub kind: super::TargetKind,
    pub profile: String,
}
