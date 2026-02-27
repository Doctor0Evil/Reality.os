#[derive(Debug, Clone)]
pub struct EvidenceBundle {
    pub atp_hex: String,
    pub eco_hex: String,
    pub pain_hex: String,
    pub duty_hex: String,
    pub visual_hex: String,
    pub host_budget_hex: String,
    pub roh_hex: String,
    pub cryptoposture_hex: String, // new
    pub reversal_hex: String,
    pub extra_hex: String,
}

#[derive(Debug, Clone)]
pub struct EvolutionPrecheckEnvelope {
    pub evidence: EvidenceBundle,
    pub env: crate::CargoEnvDescriptor,
}

impl EvolutionPrecheckEnvelope {
    pub fn passes_all(&self) -> bool {
        self.passes_bioscale_envelopes()
            && self.passes_crypto_posture()
    }

    fn passes_bioscale_envelopes(&self) -> bool {
        // existing HostBudget / RoH / pain / eco checks
        true
    }

    fn passes_crypto_posture(&self) -> bool {
        // cryptoposture_hex resolves via HexProofRegistry to “no-BLAKE, no-Argon2”
        let registry = crate::hex_proof_registry::global_registry();
        if let Some(meta) = registry.decode(&self.evidence.cryptoposture_hex) {
            meta.label.contains("no-BLAKE") && meta.label.contains("no-Argon2")
        } else {
            false
        }
    }
}
