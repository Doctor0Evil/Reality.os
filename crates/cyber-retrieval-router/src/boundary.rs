use neurorights_firewall::{NeurorightsBound, NeurorightsEnvelope};
use cyber_retrieval_types::{PromptEnvelope, RawPrompt, normalize_prompt};
use crate::CyberRetrievalRouter;

pub async fn entry_from_http(
    router: CyberRetrievalRouter,
    user_did: String,
    aln: String,
    bostrom_address: String,
    text: String,
    source_client: String,
    session_id: String,
) -> Result<serde_json::Value, crate::RouterError> {
    let raw = RawPrompt {
        user_did: &user_did,
        text: &text,
        security_level: crate::SecurityLevel::Restricted,
        source_client: &source_client,
        session_id: &session_id,
        aln: &aln,
        bostrom_address: &bostrom_address,
    };
    let env: PromptEnvelope = normalize_prompt(raw);

    // Wrap with compile-time neurorights envelope.
    let bound: NeurorightsBound<PromptEnvelope, NeurorightsEnvelope> =
        NeurorightsBound::new(env);

    router.handle_citizen_request(bound).await
}
