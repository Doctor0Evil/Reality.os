pub mod virus;

pub use crate::virus::{
    ImpactProfile,
    ThreatClass,
    VirusSignature,
    VirusCheckResult,
    evaluate_against_signatures,
};
