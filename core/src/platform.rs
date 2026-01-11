use crate::events::{LedgerEvent, LedgerEventKind};
use crate::types::{AccessDecisionLog, Identity, Platform};
use chrono::Utc;
use uuid::Uuid;

pub struct FairnessStats {
    pub platform_id: Uuid,
    pub identity_id: Uuid,
    pub penalty_rate: f64,
}

pub struct FairnessAuditor;

impl FairnessAuditor {
    /// Very light K‑S‑inspired divergence check between a platform penalty rate and global mean.
    pub fn detect_unfairness(
        platform_rate: f64,
        global_mean: f64,
        eco_impact_score: f64,
        current_karma: f64,
        threshold_multiplier: f64,
    ) -> bool {
        if eco_impact_score <= 0.8 {
            return false;
        }
        if current_karma <= 0.8 {
            return false;
        }

        platform_rate > global_mean * threshold_multiplier
    }

    pub fn downgrade_platform_trust(platform: &mut Platform, delta: f64) -> (f64, LedgerEvent) {
        let previous = platform.trust_score;
        let new = (platform.trust_score + delta).clamp(0.0, 1.0);
        platform.trust_score = new;
        platform.updated_at = Utc::now();

        let ev = LedgerEvent {
            id: Uuid::new_v4(),
            kind: LedgerEventKind::PlatformFairnessAudit {
                platform_id: platform.id,
                previous_trust: previous,
                new_trust: new,
                details: "Automatic fairness downgrade due to abnormal penalty rate".into(),
            },
            created_at: Utc::now(),
        };
        (new, ev)
    }
}

pub struct ContributionNormalizer;

impl ContributionNormalizer {
    /// Map multi‑platform contributions to unified contribution_score ΔC.
    pub fn map_github_contribution(merged_prs: u32, ceim_tools_commits: u32) -> f64 {
        let pr_val = (merged_prs as f64) * 0.05;
        let tools_val = (ceim_tools_commits as f64) * 0.1;
        pr_val + tools_val
    }

    pub fn map_ai_chat_contribution(accepted_designs: u32, accepted_code_snippets: u32) -> f64 {
        let design_val = (accepted_designs as f64) * 0.08;
        let code_val = (accepted_code_snippets as f64) * 0.03;
        design_val + code_val
    }

    pub fn map_device_cluster_contribution(active_nodes: u32, months: u32) -> f64 {
        let base = 0.04;
        let scale = 0.05;
        (active_nodes as f64 * months as f64) * (base + scale)
    }

    pub fn map_internal_research_contribution(validated_pipelines: u32) -> f64 {
        (validated_pipelines as f64) * 0.06
    }

    /// Weight by real eco impact (e.g. PFBS tons reduced).
    pub fn weight_by_impact(base_delta: f64, pfbs_tons_reduced: f64) -> f64 {
        let weight = (pfbs_tons_reduced / 10.0).min(2.0).max(0.1);
        base_delta * weight
    }

    pub fn apply_to_identity(identity: &mut Identity, delta_c: f64) {
        identity.contribution_score += delta_c.max(0.0);
        identity.updated_at = Utc::now();
    }
}
