use anyhow::{Context, Result};
use ed25519_dalek::{PublicKey, Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::str::FromStr;

/// DID signature verification result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DIDVerificationResult {
    Valid,
    Invalid,
    MissingSignature,
    InvalidFormat,
}

/// DID signature verification structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DIDSignature {
    pub did: String,
    pub signature: String,
    pub message: String,
}

/// Verify a DID signature against a public key
pub fn verify_did_signature(
    signature: &DIDSignature,
    public_key: &PublicKey,
) -> Result<DIDVerificationResult> {
    // Parse the signature
    let signature_bytes = hex::decode(&signature.signature)
        .context("Invalid hex signature format")?;
    if signature_bytes.len() != 64 {
        return Ok(DIDVerificationResult::InvalidFormat);
    }
    let signature = Signature::from_slice(&signature_bytes)
        .context("Invalid signature format")?;

    // Hash the message
    let mut hasher = Keccak256::new();
    hasher.update(signature.message.as_bytes());
    let message_hash = hasher.finalize();

    // Verify the signature
    if public_key.verify(&message_hash, &signature).is_ok() {
        Ok(DIDVerificationResult::Valid)
    } else {
        Ok(DIDVerificationResult::Invalid)
    }
}

/// Extract public key from DID identifier
pub fn did_to_public_key(did: &str) -> Result<PublicKey> {
    // In practice, this would map DID to a public key via a DID resolver
    // For this implementation, we'll use a simplified approach
    // (In production, would connect to a DID resolver service)
    
    // Extract the actual public key part from the DID
    // This is a simplified example - in reality would use a DID resolver
    let public_key_str = match did {
        "did:ion:EiD8J2b3K8k9Q8x9L7m2n4p1q5r6s7t8u9v0w1x2y3z4A5B6C7D8E9F0" => {
            "038a3d0a0e6b5e0d5b3c2a1f0e9d8c7b6a5f4e3d2c1b0a9f8e7d6c5b4a3f2e1d0c9b8a7"
        }
        "bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7" => {
            "03f1a9d5b3c2e1f0d9c8b7a6f5e4d3c2b1a0f9e8d7c6b5a4f3e2d1c0b9a8f7e6d5c4b3a2"
        }
        _ => return Err(anyhow::anyhow!("Unknown DID format")),
    };

    // Parse the public key
    let public_key_bytes = hex::decode(public_key_str)?;
    PublicKey::from_bytes(&public_key_bytes)
        .context("Invalid public key format")
}
