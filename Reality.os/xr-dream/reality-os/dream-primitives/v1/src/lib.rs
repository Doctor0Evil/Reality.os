use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A minimal, lab-grade primitive representing a cloned cyber-organic workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneWorkspace {
    pub id: Uuid,
    pub root: String,
    pub projects: Vec<CloneProject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneProject {
    pub name: String,
    pub path: String,
}
