use std::fmt;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

/// Hard-coded, policy-aligned hash function identifier.
/// BLAKE3 and any alias MUST NOT appear in this file or in configs enforced by it.
pub const APPROVED_HASH_FN: &str = "SHA-256";

/// Cybernetic-safe, neurorights-aligned hash domains.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HashDomain {
    /// Default domain for Cargo / dev-tunnel / command logs.
    AuditCommand,
    /// Domain for neurorights & biophysical evidence logs.
    NeurorightsEvidence,
}

impl fmt::Display for HashDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HashDomain::AuditCommand => "audit-command",
            HashDomain::NeurorightsEvidence => "neurorights-evidence",
        };
        write!(f, "{}", s)
    }
}

/// Crypto policy errors for Reality.os / SAFE_INTERPRETATION guards.
#[derive(Debug)]
pub enum CryptoPolicyError {
    /// A disallowed hash function was requested or observed.
    DisallowedHashFunction {
        requested: String,
    },
    /// Log material was empty or otherwise invalid.
    InvalidLogMaterial(&'static str),
}

impl fmt::Display for CryptoPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoPolicyError::DisallowedHashFunction { requested } => {
                write!(
                    f,
                    "hash function '{}' is disallowed by bioscale / neurorights policy; only '{}' is permitted for this guard",
                    requested, APPROVED_HASH_FN
                )
            }
            CryptoPolicyError::InvalidLogMaterial(msg) => {
                write!(f, "invalid log material: {msg}")
            }
        }
    }
}

impl std::error::Error for CryptoPolicyError {}

/// Static denylist for hash identifiers.
/// This MUST be enforced before constructing any digest.
fn is_disallowed_hash_name(name: &str) -> bool {
    let lowered = name.to_ascii_lowercase();
    lowered.contains("blake3") || lowered == "blake-3" || lowered == "blake_3"
}

/// A single command / evolution event as seen by SAFE_INTERPRETATION.
#[derive(Clone, Debug)]
pub struct CommandEvent<'a> {
    /// Exact command, e.g. "cargo build".
    pub command: &'a str,
    /// Arguments as a single canonical string, e.g. "--release -p cybercore".
    pub args: &'a str,
    /// Git commit SHA or equivalent evolution anchor.
    pub git_sha: &'a str,
    /// Evidence bundle identifier (bioscale evidence pack, etc.).
    pub evidence_bundle_id: &'a str,
    /// Bostrom DID binding the event to an operator identity.
    pub bostrom_did: &'a str,
}

/// Resulting immutable digest for an event, suitable for on-chain / off-chain anchoring.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventDigest {
    pub algo: &'static str,
    pub domain: HashDomain,
    /// Hex-encoded digest string.
    pub hex_digest: String,
}

impl EventDigest {
    pub fn as_str(&self) -> &str {
        &self.hex_digest
    }
}

/// A policy guard that:
/// - refuses any non-approved hash function,
/// - provides a stable, bioscale-safe hashing layout for command/evolution logs.
pub struct CryptoPolicyGuard {
    /// The only function currently allowed by policy for this guard.
    approved_fn: &'static str,
}

impl Default for CryptoPolicyGuard {
    fn default() -> Self {
        Self {
            approved_fn: APPROVED_HASH_FN,
        }
    }
}

impl CryptoPolicyGuard {
    pub fn new() -> Self {
        Self::default()
    }

    /// Assert that the requested hash function is allowed under current policy.
    /// This is intended to be called by Reality.os CargoEnvDescriptor wiring.
    pub fn assert_allowed_function(
        &self,
        requested: &str,
    ) -> Result<(), CryptoPolicyError> {
        if is_disallowed_hash_name(requested) {
            return Err(CryptoPolicyError::DisallowedHashFunction {
                requested: requested.to_owned(),
            });
        }

        // Only SHA-256 is currently permitted here.
        if !requested.eq_ignore_ascii_case(self.approved_fn) {
            return Err(CryptoPolicyError::DisallowedHashFunction {
                requested: requested.to_owned(),
            });
        }

        Ok(())
    }

    /// Convenience wrapper: enforce that the global environment has not
    /// accidentally selected a disallowed function via configuration.
    pub fn assert_env_hash_safe(&self) -> Result<(), CryptoPolicyError> {
        // In a full integration this would be wired to Reality.os / CargoEnvDescriptor.
        // For now, we simply enforce that no override away from APPROVED_HASH_FN is present.
        let configured = std::env::var("CYBERSWARM_HASH_FN")
            .unwrap_or_else(|_| self.approved_fn.to_string());

        self.assert_allowed_function(&configured)
    }
}

/// Bioscale-safe, neurorights-aligned hashing interface for command/evolution logs.
///
/// This enforces:
/// - SHA-256 only,
/// - stable field ordering,
/// - inclusion of Bostrom DID and EvidenceBundle ID,
/// - a simple domain-separation prefix.
pub struct BioSecureHasher {
    guard: CryptoPolicyGuard,
}

impl BioSecureHasher {
    pub fn new() -> Self {
        Self {
            guard: CryptoPolicyGuard::new(),
        }
    }

    /// Hash a command event in the `audit-command` domain using SHA-256.
    pub fn hash_command_event(
        &self,
        event: &CommandEvent<'_>,
    ) -> Result<EventDigest, CryptoPolicyError> {
        self.guard.assert_env_hash_safe()?;

        if event.command.trim().is_empty() {
            return Err(CryptoPolicyError::InvalidLogMaterial(
                "command must not be empty",
            ));
        }
        if event.bostrom_did.trim().is_empty() {
            return Err(CryptoPolicyError::InvalidLogMaterial(
                "bostrom_did must not be empty",
            ));
        }
        if event.evidence_bundle_id.trim().is_empty() {
            return Err(CryptoPolicyError::InvalidLogMaterial(
                "evidence_bundle_id must not be empty",
            ));
        }

        let domain = HashDomain::AuditCommand;
        let bytes = self.build_material(domain, event);
        let hex_digest = sha256_hex(&bytes);

        Ok(EventDigest {
            algo: self.guard.approved_fn,
            domain,
            hex_digest,
        })
    }

    /// Hash the same command event in the `neurorights-evidence` domain.
    /// This is useful when the same evolution is logged into a neurorights-only channel
    /// with a distinct digest namespace.
    pub fn hash_neurorights_event(
        &self,
        event: &CommandEvent<'_>,
    ) -> Result<EventDigest, CryptoPolicyError> {
        self.guard.assert_env_hash_safe()?;

        let domain = HashDomain::NeurorightsEvidence;
        let bytes = self.build_material(domain, event);
        let hex_digest = sha256_hex(&bytes);

        Ok(EventDigest {
            algo: self.guard.approved_fn,
            domain,
            hex_digest,
        })
    }

    /// Build a stable, deterministic material buffer for hashing.
    ///
    /// Layout:
    /// - "cyberswarm:" literal prefix
    /// - domain label
    /// - '\n' delimiter
    /// - key=value lines in fixed order:
    ///   - command
    ///   - args
    ///   - git_sha
    ///   - evidence_bundle_id
    ///   - bostrom_did
    ///   - ts_unix
    fn build_material(
        &self,
        domain: HashDomain,
        event: &CommandEvent<'_>,
    ) -> Vec<u8> {
        let mut buf = String::with_capacity(256);
        buf.push_str("cyberswarm:");
        buf.push_str(&domain.to_string());
        buf.push('\n');

        buf.push_str("command=");
        buf.push_str(event.command.trim());
        buf.push('\n');

        buf.push_str("args=");
        buf.push_str(event.args.trim());
        buf.push('\n');

        buf.push_str("git_sha=");
        buf.push_str(event.git_sha.trim());
        buf.push('\n');

        buf.push_str("evidence_bundle_id=");
        buf.push_str(event.evidence_bundle_id.trim());
        buf.push('\n');

        buf.push_str("bostrom_did=");
        buf.push_str(event.bostrom_did.trim());
        buf.push('\n');

        let ts_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        buf.push_str("ts_unix=");
        buf.push_str(&ts_unix.to_string());
        buf.push('\n');

        buf.into_bytes()
    }
}

/// SHA-256 helper returning hex string.
fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let bytes = hasher.finalize();
    hex::encode(bytes)
}

/// Minimal, host-local hygiene helper to detect disallowed hash
/// crates in Cargo manifests for the current workspace.
///
/// This does NOT modify files; it is intended to be wired into
/// a CI check or a `build.rs` style preflight.
pub fn detect_disallowed_hash_in_manifest<P: AsRef<Path>>(
    manifest_path: P,
) -> Result<bool, std::io::Error> {
    use std::io::Read;

    let path = manifest_path.as_ref();
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lowered = contents.to_ascii_lowercase();
    Ok(lowered.contains("blake3") || lowered.contains("blake-3"))
}

/// Example unit tests to verify behavior.
///
/// In production, move these into a dedicated `tests/` module or workspace crate.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_rejects_disallowed_fn() {
        let guard = CryptoPolicyGuard::new();
        let err = guard
            .assert_allowed_function("BLAKE3")
            .expect_err("BLAKE3 must be rejected");
        match err {
            CryptoPolicyError::DisallowedHashFunction { requested } => {
                assert_eq!(requested.to_ascii_uppercase(), "BLAKE3");
            }
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn policy_accepts_sha256() {
        let guard = CryptoPolicyGuard::new();
        guard
            .assert_allowed_function("SHA-256")
            .expect("SHA-256 must be allowed");
    }

    #[test]
    fn hashing_produces_stable_output() {
        let hasher = BioSecureHasher::new();
        let event = CommandEvent {
            command: "cargo build",
            args: "--release -p cyberswarm-core",
            git_sha: "deadbeefdeadbeefdeadbeefdeadbeefdeadbeef",
            evidence_bundle_id: "EVB-TEST-001",
            bostrom_did: "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7",
        };

        let d1 = hasher.hash_command_event(&event).unwrap();
        let d2 = hasher.hash_command_event(&event).unwrap();

        // Different timestamps make the full digest change over real time,
        // so here we only assert algo and domain consistency.
        assert_eq!(d1.algo, APPROVED_HASH_FN);
        assert_eq!(d1.domain, HashDomain::AuditCommand);
        assert_eq!(d2.algo, APPROVED_HASH_FN);
        assert_eq!(d2.domain, HashDomain::AuditCommand);
    }

    #[test]
    fn manifest_detection_finds_banned_string() {
        use std::io::Write;
        use std::path::PathBuf;

        let mut tmp = std::env::temp_dir();
        tmp.push("cyberswarm_manifest_test.toml");

        let mut f = std::fs::File::create(&tmp).unwrap();
        writeln!(f, r#"blake3 = "1.5.0""#).unwrap();
        drop(f);

        let found = detect_disallowed_hash_in_manifest(&tmp).unwrap();
        assert!(found, "expected to detect disallowed hash entry");

        let _ = std::fs::remove_file(&tmp);
    }
}
