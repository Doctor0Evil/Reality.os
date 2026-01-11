use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectKind {
    RealityOs,
    DreamscapeOs,
    XrGridInfrastructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSpec {
    pub name: String,
    pub repo_name: String,
    pub default_branch: String,
    pub private: bool,
}
