use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigstoreBundle {
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "verificationMaterial")]
    pub verification_material: VerificationMaterial,
    #[serde(rename = "dsseEnvelope")]
    pub dsse_envelope: DsseEnvelope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationMaterial {
    #[serde(rename = "timestampVerificationData")]
    pub timestamp_verification_data: Option<TimestampVerificationData>,
    pub certificate: CertificateWrapper,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampVerificationData {
    #[serde(rename = "rfc3161Timestamps")]
    pub rfc3161_timestamps: Vec<Rfc3161Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rfc3161Timestamp {
    #[serde(rename = "signedTimestamp")]
    pub signed_timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateWrapper {
    #[serde(rename = "rawBytes")]
    pub raw_bytes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DsseEnvelope {
    pub payload: String,
    #[serde(rename = "payloadType")]
    pub payload_type: String,
    pub signatures: Vec<DsseSignature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DsseSignature {
    pub sig: String,
}

/// In-toto Statement v1 used by Sigstore for releases.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InTotoStatement {
    #[serde(rename = "_type")]
    pub typ: String,
    pub subject: Vec<Subject>,
    #[serde(rename = "predicateType")]
    pub predicate_type: String,
    pub predicate: ReleasePredicate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub uri: String,
    pub digest: DigestMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigestMap {
    #[serde(default)]
    #[serde(rename = "sha1")]
    pub sha1: Option<String>,
    #[serde(default)]
    #[serde(rename = "sha256")]
    pub sha256: Option<String>,
}

/// Release metadata (in-toto release predicate v0.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleasePredicate {
    #[serde(rename = "databaseId")]
    pub database_id: Option<String>,
    #[serde(rename = "ownerId")]
    pub owner_id: Option<String>,
    #[serde(rename = "packageId")]
    pub package_id: Option<String>,
    pub purl: Option<String>,
    pub repository: Option<String>,
    #[serde(rename = "repositoryId")]
    pub repository_id: Option<String>,
    pub tag: Option<String>,
}
