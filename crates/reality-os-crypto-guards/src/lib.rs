#![forbid(unsafe_code)]

//! Crypto posture and threat guard layer for Reality.os.
//!
//! This crate wires CargoEnvDescriptor, ALNComplianceParticle, and the
//! Phoenix Neurostack router into a single BLAKE3/Argon2 veto surface
//! for BCI / ROD corridors only.
//!
//! IMPORTANT DOCTRINE:
//! - This guard is allowed to DENY or QUARANTINE *software* evolution paths.
//! - It MUST NOT roll back or downgrade any biophysical evolution,
//!   HostBudget records, or BCI state. Its only rollback is: "do not
//!   admit this upgrade / route into BCI/ROD corridors".

use std::fmt;
use std::time::SystemTime;

// --- Core host types (these live in other crates in your stack) -----------

/// Environment descriptor published by Reality.os; includes crypto posture
/// and supply-chain policy for this host.
#[derive(Clone, Debug)]
pub struct CargoEnvDescriptor {
    pub host_id: String,
    pub target_triple: String,
    pub ota_repo: String,
    pub blake3_allowed: bool,
    pub argon2_allowed: bool,
    pub active_pattern_set: CryptoPatternSetId,
}

/// Identifier for a versioned crypto pattern set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CryptoPatternSetId(pub String);

/// Minimal view of an ALN compliance particle; full struct lives in ALN crates.
#[derive(Clone, Debug)]
pub struct ALNComplianceParticle {
    pub did: String,
    pub clause_ids: Vec<String>,
    pub evidence_ids: Vec<String>,
    pub upgrade_hash_hint: String,
    pub timestamp: SystemTime,
}

/// Phoenix router threat surface classification for a given request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ThreatSurface {
    /// Generic software surface (logging, auth, non‑BCI services).
    GenericSoftware,
    /// BCI / EEG / HCI stacks, including visual corridors.
    BciCorridor,
    /// ROD‑sensitive corridors (Neurorights ROD, pain debt, danger metrics).
    RodCorridor,
    /// Combined BCI + ROD corridor.
    BciRodCorridor,
    /// Non‑host surfaces (e.g., remote services).
    External,
}

/// High‑level view of an upgrade/manfiest that may enter a corridor.
#[derive(Clone, Debug)]
pub struct CryptoScannableDescriptor {
    /// Human‑readable name.
    pub name: String,
    /// Source / crate / image identifiers.
    pub source_ids: Vec<String>,
    /// Arbitrary metadata fields that may contain hashes or encodings.
    pub metadata_fields: Vec<String>,
    /// Hints about crypto algorithms declared or used (e.g., "argon2id", "blake3").
    pub crypto_hints: Vec<String>,
}

// --- Signature patterns for BLAKE3 / Argon2 ------------------------------

/// Result of scanning a descriptor for crypto signatures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CryptoSignatureFinding {
    pub kind: CryptoSignatureKind,
    pub token: String,
    pub source_field: String,
}

/// Category of crypto signature detected.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CryptoSignatureKind {
    /// Encoded Argon2 PHC‑format hash: $argon2id$...$...
    Argon2Encoded,
    /// Raw or labeled Argon2 use (e.g., "argon2id" in crypto hints).
    Argon2Hint,
    /// 32‑byte hex/base64 candidate compatible with BLAKE3 or similar.
    Blake3Compatible,
    /// Explicit "blake3" hint or family label.
    Blake3Hint,
}

/// Lightweight scanner for BLAKE3 / Argon2 signatures in upgrade metadata.
#[derive(Clone, Debug, Default)]
pub struct CryptoSignatureScanner;

impl CryptoSignatureScanner {
    pub fn new() -> Self {
        Self
    }

    /// Scan a descriptor and return all suspicious crypto signature findings.
    pub fn scan(&self, desc: &CryptoScannableDescriptor) -> Vec<CryptoSignatureFinding> {
        let mut findings = Vec::new();

        // 1. Argon2 PHC‑style strings in metadata.
        for field in &desc.metadata_fields {
            for token in Self::tokenize(field) {
                if let Some(f) = Self::classify_argon2_token(&token, field) {
                    findings.push(f);
                    continue;
                }
                if let Some(f) = Self::classify_hash_token(&token, field) {
                    findings.push(f);
                }
            }
        }

        // 2. Crypto hints.
        for hint in &desc.crypto_hints {
            let lower = hint.to_ascii_lowercase();
            if lower.contains("argon2") {
                findings.push(CryptoSignatureFinding {
                    kind: CryptoSignatureKind::Argon2Hint,
                    token: hint.clone(),
                    source_field: "crypto_hints".to_string(),
                });
            }
            if lower.contains("blake3") || lower.contains("blake-3") {
                findings.push(CryptoSignatureFinding {
                    kind: CryptoSignatureKind::Blake3Hint,
                    token: hint.clone(),
                    source_field: "crypto_hints".to_string(),
                });
            }
        }

        findings
    }

    fn tokenize(text: &str) -> Vec<String> {
        text.split(|c: char| !c.is_ascii_alphanumeric() && c != '$' && c != '+' && c != '/' && c != '=')
            .filter(|t| !t.is_empty())
            .map(|t| t.to_string())
            .collect()
    }

    fn classify_argon2_token(token: &str, source: &str) -> Option<CryptoSignatureFinding> {
        if !token.starts_with("$argon2") {
            return None;
        }
        let has_variant = token.starts_with("$argon2id$")
            || token.starts_with("$argon2i$")
            || token.starts_with("$argon2d$");

        if !has_variant {
            return None;
        }

        Some(CryptoSignatureFinding {
            kind: CryptoSignatureKind::Argon2Encoded,
            token: token.to_string(),
            source_field: source.to_string(),
        })
    }

    fn classify_hash_token(token: &str, source: &str) -> Option<CryptoSignatureFinding> {
        if token.len() < 32 || token.len() > 256 {
            return None;
        }

        let is_hex = token.chars().all(|c| c.is_ascii_hexdigit());
        let is_b64 = token.chars().all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=');

        if !is_hex && !is_b64 {
            return None;
        }

        // Heuristic: 64 hex chars or 44 base64 chars strongly suggest 32‑byte hash.
        if (is_hex && token.len() == 64) || (is_b64 && token.len() == 44 && token.ends_with('=')) {
            return Some(CryptoSignatureFinding {
                kind: CryptoSignatureKind::Blake3Compatible,
                token: token.to_string(),
                source_field: source.to_string(),
            });
        }

        None
    }
}

// --- Veto guard for BCI / ROD corridors -----------------------------------

/// Decision produced by the crypto veto guard.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CryptoVetoDecision {
    /// No relevant signatures, or posture allows them for this surface.
    Allowed,
    /// Signatures present but only for non‑BCI/ROD surfaces; log & continue.
    QuarantinedNonBiophysical {
        findings: Vec<CryptoSignatureFinding>,
    },
    /// Signatures violate posture for BCI/ROD corridor; deny routing.
    DeniedForBciRod {
        findings: Vec<CryptoSignatureFinding>,
        reason: String,
    },
}

impl fmt::Display for CryptoVetoDecision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoVetoDecision::Allowed => write!(f, "Allowed"),
            CryptoVetoDecision::QuarantinedNonBiophysical { findings } => {
                write!(f, "QuarantinedNonBiophysical ({} findings)", findings.len())
            }
            CryptoVetoDecision::DeniedForBciRod { findings, reason } => {
                write!(
                    f,
                    "DeniedForBciRod ({} findings): {}",
                    findings.len(),
                    reason
                )
            }
        }
    }
}

/// Trait implemented by crypto posture guards that can veto BLAKE3/Argon2
/// use on BCI / ROD corridors.
///
/// NOTE: This trait ONLY controls routing and software upgrade admission.
/// It MUST NOT modify or roll back biophysical state or evolution data.
pub trait CryptoVetoGuard {
    fn evaluate_crypto_veto(
        &self,
        env: &CargoEnvDescriptor,
        aln: &ALNComplianceParticle,
        surface: &ThreatSurface,
        desc: &CryptoScannableDescriptor,
    ) -> CryptoVetoDecision;
}

/// Default implementation: BCI/ROD corridors deny if forbidden signatures
/// appear and env posture forbids them; non‑biophysical surfaces are
/// quarantined but not denied.
#[derive(Clone, Debug)]
pub struct DefaultCryptoVetoGuard {
    scanner: CryptoSignatureScanner,
}

impl DefaultCryptoVetoGuard {
    pub fn new() -> Self {
        Self {
            scanner: CryptoSignatureScanner::new(),
        }
    }

    fn is_bci_rod(surface: &ThreatSurface) -> bool {
        matches!(
            surface,
            ThreatSurface::BciCorridor
                | ThreatSurface::RodCorridor
                | ThreatSurface::BciRodCorridor
        )
    }

    fn env_forbids_blake(env: &CargoEnvDescriptor) -> bool {
        !env.blake3_allowed
    }

    fn env_forbids_argon2(env: &CargoEnvDescriptor) -> bool {
        !env.argon2_allowed
    }
}

impl CryptoVetoGuard for DefaultCryptoVetoGuard {
    fn evaluate_crypto_veto(
        &self,
        env: &CargoEnvDescriptor,
        aln: &ALNComplianceParticle,
        surface: &ThreatSurface,
        desc: &CryptoScannableDescriptor,
    ) -> CryptoVetoDecision {
        let findings = self.scanner.scan(desc);

        if findings.is_empty() {
            return CryptoVetoDecision::Allowed;
        }

        // If this is NOT a BCI/ROD surface, just quarantine (log + tag),
        // but do not deny. This keeps generic software evolution intact.
        if !Self::is_bci_rod(surface) {
            return CryptoVetoDecision::QuarantinedNonBiophysical { findings };
        }

        // BCI/ROD: we must enforce env posture. If posture forbids Blake/Argon2
        // and we see relevant signatures, we deny routing to protect the
        // biophysical shell. We DO NOT alter any existing evolution data.
        let mut has_forbidden_blake = false;
        let mut has_forbidden_argon2 = false;

        for f in &findings {
            match f.kind {
                CryptoSignatureKind::Blake3Compatible | CryptoSignatureKind::Blake3Hint => {
                    if Self::env_forbids_blake(env) {
                        has_forbidden_blake = true;
                    }
                }
                CryptoSignatureKind::Argon2Encoded | CryptoSignatureKind::Argon2Hint => {
                    if Self::env_forbids_argon2(env) {
                        has_forbidden_argon2 = true;
                    }
                }
            }
        }

        if has_forbidden_blake || has_forbidden_argon2 {
            let mut reason_parts = Vec::new();
            if has_forbidden_blake {
                reason_parts.push("BLAKE3 signatures forbidden on BCI/ROD corridor");
            }
            if has_forbidden_argon2 {
                reason_parts.push("Argon2 signatures forbidden on BCI/ROD corridor");
            }

            let reason = format!(
                "Crypto posture violation for host {} (pattern set {}), ALN DID {}: {}",
                env.host_id,
                env.active_pattern_set.0,
                aln.did,
                reason_parts.join(" + ")
            );

            CryptoVetoDecision::DeniedForBciRod { findings, reason }
        } else {
            // Either env posture allows these primitives OR they were not
            // relevant to BCI/ROD restrictions. Allow routing to proceed.
            CryptoVetoDecision::Allowed
        }
    }
}

// --- Integration helpers --------------------------------------------------

/// Helper: run crypto veto before calling any BCI/ROD routing/evolution code.
/// This function is intended to be called by the Phoenix Neurostack router
/// just before it invokes bioscale / neurorights gates for a BCI/ROD action.
///
/// It returns `Result<(), CryptoVetoDecision>`:
/// - `Ok(())`  => caller may proceed with bioscale + neurorights evaluation.
/// - `Err(dec)` => caller MUST NOT perform actuation; dec describes why.
pub fn pre_bci_rod_crypto_veto<G: CryptoVetoGuard>(
    guard: &G,
    env: &CargoEnvDescriptor,
    aln: &ALNComplianceParticle,
    surface: &ThreatSurface,
    desc: &CryptoScannableDescriptor,
) -> Result<(), CryptoVetoDecision> {
    let decision = guard.evaluate_crypto_veto(env, aln, surface, desc);
    match decision {
        CryptoVetoDecision::Allowed | CryptoVetoDecision::QuarantinedNonBiophysical { .. } => {
            // In both cases, we allow the pipeline to continue:
            // - Allowed: no issue.
            // - QuarantinedNonBiophysical: tag & log upstream, but not a BCI/ROD operation.
            Ok(())
        }
        CryptoVetoDecision::DeniedForBciRod { .. } => Err(decision),
    }
}
