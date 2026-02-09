// Reality.os ReversalConditions and downgrade gating kernel.
//
// This module assumes the existing NewRow-Print! ALN core:
//
//   - CapabilityState, ConsentState, Role, Jurisdiction, PolicyStack
//   - EvidenceBundleId, ConsentTokenId
//   - Decision, DecisionReason
//   - CapabilityTransitionRequest
//
// as defined in `policy_engine/src/aln_core.rs`. It adds a
// Reality.os-specific layer that:
//
//   * Treats neuromorph evolution as strictly monotone by default.
//   * Forbids automatic downgrades or reversals.
//   * Allows capability downgrades ONLY when:
//       - An explicit, owner-signed reversal order exists.
//       - All softer mitigations are exhausted (no safer alternative).
//       - The composite PolicyStack still passes.
//   * Treats envelope outputs as advisory (recommend-only), never
//     sufficient to change CapabilityState on their own.
//

use serde::{Deserialize, Serialize};
use std::fmt;

/// Import the existing core types from your ALN kernel.
/// Adjust the path as needed to match your project layout.
use crate::aln_core::{
    CapabilityState,
    ConsentState,
    Decision,
    DecisionReason,
    PolicyStack,
    Role,
    Jurisdiction,
    EvidenceBundleId,
    ConsentTokenId,
    CapabilityTransitionRequest,
};

/// Reality.os-specific reversal policy configuration.
///
/// This corresponds to the SECTION,REVERSAL-POLICY shard you defined:
/// - `allow_neuromorph_reversal` is a non-waivable global default.
/// - `explicit_reversal_order` is an owner-signed order flag.
/// - `no_safer_alternative` is derived from Tier-2 logic after all
///   non-reversal mitigations (pause, tighten, rest) have been tried.[file:1]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReversalPolicy {
    /// Global flag: neuromorph evolution downgrades are forbidden unless
    /// this is explicitly set true at the policy level.
    ///
    /// In your current stack this SHOULD be `false` as the default and
    /// non-waivable for Reality.os.[file:2]
    pub allow_neuromorph_reversal: bool,

    /// Per-event flag: set true only by HostOrganicCPU / Owner via a
    /// signed decision recorded in `.stake.aln` and `.evolve.jsonl`.
    pub explicit_reversal_order: bool,

    /// Derived flag: Tier-2 envelope & control logic confirms that all
    /// non-reversal mitigations are exhausted (no safer alternative). [file:1]
    pub no_safer_alternative: bool,
}

impl ReversalPolicy {
    /// Convenience predicate matching the ALN condition:
    ///   can_revert_capability = explicit_reversal_order == true
    ///                            AND no_safer_alternative == true
    /// plus the global `allow_neuromorph_reversal` guard.
    pub fn can_revert_capability(&self) -> bool {
        self.allow_neuromorph_reversal
            && self.explicit_reversal_order
            && self.no_safer_alternative
    }
}

/// Advisory output from the biophysical envelope layer for a given
/// transition attempt.
///
/// This is the distilled, policy-relevant subset of the
/// SECTION,DECISIONS and SECTION,OUTPUTS flags:
///
///   - `requires_downgrade`: Tier-2 biophysics recommends a downgrade. [file:1]
///   - `request_capability_downgrade`: meta + owner gate allow the
///      envelope shard to REQUEST a downgrade, but still not apply it.[file:1]
///
/// Reality.os treats these purely as inputs to the decision, never as
/// direct authority to change `CapabilityState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvelopeDowngradeAdvice {
    /// True if risk-epoch hysteresis is satisfied and the envelope
    /// recommends downgrade for safety at the current tier.
    pub requires_downgrade: bool,

    /// True only if:
    ///   - `requires_downgrade` is true,
    ///   - shard `autodowngrade_enabled` is true,
    ///   - `owner_downgrade_approved` is true.[file:1]
    ///
    /// Even in this case, Reality.os still does NOT auto-downgrade;
    /// this flag only indicates a well-formed request for review.
    pub request_capability_downgrade: bool,
}

/// Extended transition request that Reality.os evaluates.
///
/// This wraps the existing `CapabilityTransitionRequest` with:
///   - Envelope downgrade advice (recommend-only).
///   - Reversal policy flags for neuromorph evolution.
///   - A marker indicating whether this transition is considered
///     a neuromorph evolution downgrade for sovereignty purposes.[file:1][file:2]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealityTransitionRequest {
    pub core: CapabilityTransitionRequest,
    pub envelope_advice: EnvelopeDowngradeAdvice,
    pub reversal_policy: ReversalPolicy,

    /// True if, in ALN, this transition changes the neuromorph
    /// evolution state (e.g., undoing a previously authorized
    /// capability increase).[file:1]
    ///
    /// This gives you the freedom to permit certain operational
    /// downgrades (e.g., temporarily stepping down a tool) while
    /// still forbidding evolution rollback without full ReversalConditions.
    pub is_neuromorph_evolution_downgrade: bool,
}

/// Reality.os-specific decision reasons, extending the base kernel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RealityDecisionReason {
    /// Transition accepted; core DecisionReason::Allowed applies.
    Allowed,

    /// Core engine denied due to consent, policy stack, or evidence.
    CoreDenied(DecisionReason),

    /// Downgrade requested without a valid Owner/Host signature in
    /// `.stake.aln` / consent token chain.[file:2]
    DeniedMissingOwnerSignature,

    /// Attempted neuromorph evolution downgrade when global policy
    /// forbids reversal (allow_neuromorph_reversal == false).[file:1]
    DeniedNeuromorphReversalForbidden,

    /// Attempted neuromorph evolution downgrade without
    /// `explicit_reversal_order` and `no_safer_alternative` both true.[file:1]
    DeniedReversalConditionsNotMet,

    /// Attempted envelope-driven downgrade with no accompanying
    /// owner approval (request_capability_downgrade == false).
    DeniedNoOwnerDowngradeApproval,

    /// Generic fallback for patterns that should not occur under
    /// the current ALN / shard semantics.
    DeniedUnknown,
}

impl fmt::Display for RealityDecisionReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RealityDecisionReason::*;
        match self {
            Allowed => write!(f, "Allowed"),
            CoreDenied(r) => write!(f, "CoreDenied({:?})", r),
            DeniedMissingOwnerSignature => write!(f, "DeniedMissingOwnerSignature"),
            DeniedNeuromorphReversalForbidden => {
                write!(f, "DeniedNeuromorphReversalForbidden")
            }
            DeniedReversalConditionsNotMet => {
                write!(f, "DeniedReversalConditionsNotMet")
            }
            DeniedNoOwnerDowngradeApproval => {
                write!(f, "DeniedNoOwnerDowngradeApproval")
            }
            DeniedUnknown => write!(f, "DeniedUnknown"),
        }
    }
}

/// Combined Reality.os decision.
///
/// This preserves the base `Decision` for compatibility with existing
/// code, but adds a RealityDecisionReason for explicit audit trails
/// in `.donutloop.aln` and Googolswarm proofs.[file:3]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RealityDecision {
    pub core: Decision,
    pub reality_reason: RealityDecisionReason,
}

impl RealityDecision {
    pub fn allow() -> Self {
        Self {
            core: Decision::allow(),
            reality_reason: RealityDecisionReason::Allowed,
        }
    }

    pub fn deny(core_reason: DecisionReason, reality_reason: RealityDecisionReason) -> Self {
        Self {
            core: Decision::deny(core_reason),
            reality_reason,
        }
    }
}

/// Helper: detect whether the transition is a plain upgrade, plain
/// downgrade, or lateral move in capability.
fn classify_transition(from: CapabilityState, to: CapabilityState) -> i32 {
    use CapabilityState::*;
    let rank = |s: CapabilityState| -> i32 {
        match s {
            CapModelOnly => 0,
            CapLabBench => 1,
            CapControlledHuman => 2,
            CapGeneralUse => 3,
        }
    };

    rank(to) - rank(from)
}

/// Reality.os evaluation of a capability transition, including
/// neuromorph evolution monotonicity and owner-gated downgrades.
///
/// This function is pure and side-effect free; it can be model-checked
/// and used as the kernel that higher layers call from the sovereign
/// kernel / hybrid bootloader shell.[file:2][file:3]
pub fn evaluate_reality_transition(req: &RealityTransitionRequest) -> RealityDecision {
    use DecisionReason as CoreReason;
    use RealityDecisionReason as RReason;

    // 1. Let the existing ALN kernel evaluate baseline safety:
    //    consent, PolicyStack, evidence, role-based upgrade rules, etc.
    let core_decision = req.core.evaluate();
    if !core_decision.allowed {
        return RealityDecision::deny(core_decision.reason, RReason::CoreDenied(core_decision.reason));
    }

    // At this point, the base engine allows the transition in principle.
    // Reality.os adds stricter rules for downgrades and neuromorph reversal.

    let delta = classify_transition(req.core.from, req.core.to);

    // 2. Upgrades and lateral moves: accept as-is, since core has
    //    already enforced RoH <= 0.30, PolicyStack, and consent.[file:2]
    if delta >= 0 {
        return RealityDecision::allow();
    }

    // 3. For any downgrade, envelopes are advisory ONLY.
    //
    //    If there is no well-formed, owner-approved downgrade request
    //    (request_capability_downgrade == true), then denial is strict.
    if !req.envelope_advice.request_capability_downgrade {
        // Envelopes may still tighten/soften within the current tier;
        // they simply cannot change CapabilityState.
        return RealityDecision::deny(
            CoreReason::DeniedIllegalDowngradeByNonRegulator,
            RReason::DeniedNoOwnerDowngradeApproval,
        );
    }

    // 4. Downgrade is requested and owner-approved at the envelope level.
    //    Distinguish between:
    //      - Operational downgrades (non-evolution).
    //      - Neuromorph evolution downgrades (true reversals).
    if !req.is_neuromorph_evolution_downgrade {
        // Operational downgrade (e.g., step-down of an app or tool),
        // still subject to PolicyStack and consent (already checked).
        return RealityDecision::allow();
    }

    // 5. Neuromorph evolution downgrade: apply strict ReversalConditions.
    //
    // Global default should be: allow_neuromorph_reversal == false.
    if !req.reversal_policy.allow_neuromorph_reversal {
        return RealityDecision::deny(
            CoreReason::DeniedUnknown,
            RReason::DeniedNeuromorphReversalForbidden,
        );
    }

    // Require explicit owner order AND no safer alternative.
    if !req.reversal_policy.can_revert_capability() {
        return RealityDecision::deny(
            CoreReason::DeniedUnknown,
            RReason::DeniedReversalConditionsNotMet,
        );
    }

    // Optionally, an extra PolicyStack pass can be enforced here if you
    // want revalidation just before applying a neuromorph reversal.
    if !req.core.policystack.all_pass() {
        return RealityDecision::deny(
            CoreReason::DeniedPolicyStackFailure,
            RReason::CoreDenied(CoreReason::DeniedPolicyStackFailure),
        );
    }

    // 6. All conditions satisfied: accept the neuromorph evolution
    //    downgrade as an explicit, last-resort, owner-signed reversal.
    RealityDecision::allow()
}

/// Optional helper: convenience constructor for a RealityTransitionRequest
/// from core fields, used when wiring from ALN / SECTION,REVERSAL-POLICY
/// and envelope shards.
impl RealityTransitionRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        core: CapabilityTransitionRequest,
        envelope_advice: EnvelopeDowngradeAdvice,
        reversal_policy: ReversalPolicy,
        is_neuromorph_evolution_downgrade: bool,
    ) -> Self {
        Self {
            core,
            envelope_advice,
            reversal_policy,
            is_neuromorph_evolution_downgrade,
        }
    }
}
