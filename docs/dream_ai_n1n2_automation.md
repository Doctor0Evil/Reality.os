# dream_ai_n1n2_automation

dream_ai_n1n2_automation is a neurorights‑safe automation layer for N1/N2 light sleep that runs low‑intensity DreamAI scaffolding instead of full dream authoring. It is designed to be controlled by AI chats using ALN shards and state‑only tunnels, so augmented users can build and debug sleep‑XR systems without running heavy local toolchains or paid CI. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/c026ce21-b4eb-4a13-b098-785bba851c29/dreamscapes-and-xr-gaming-with-C7jqxNuHQWOXbqJfhdO_Og.md)

## 1. Goals and guarantees

- Keep all interventions **gentle** (no narrative writing, only overlays and stabilization). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/c026ce21-b4eb-4a13-b098-785bba851c29/dreamscapes-and-xr-gaming-with-C7jqxNuHQWOXbqJfhdO_Og.md)
- Operate only in N1/N2 windows validated against PSG timing and micro‑arousal safety limits. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/21531c1f-d10a-47d2-a944-d9e2101c1c22/dream-spectre-can-formalize-n1-L3agt9_OREenVEwrdm2i_w.md)
- Enforce hard neurorights: mentalprivacy, cognitiveliberty, nopunitivexr, soulnonaddressable. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)
- Expose a small, deterministic state interface that any AI chat (Mistral, Qwen, Perplexity, Grok, Deepseek) can use safely. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)

## 2. Core state model

All runtime state is encoded in CSV‑style `.aln` QPU.Datashards and mirrored in Rust structs. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ce486c90-2e6f-4d13-8ab0-f99109a75497/let-s-begin-defining-parameter-5o2YqjJCTdKx2m9mLrP5cA.md)

### 2.1 Subject state

- `sleepstage ∈ {wake,N1,N2,N3,REM}` (PSG or validated wearable staging). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/21531c1f-d10a-47d2-a944-d9e2101c1c22/dream-spectre-can-formalize-n1-L3agt9_OREenVEwrdm2i_w.md)
- `dreammode ∈ {none,nonlucid,prelucid,lucid,nightmare}` (high‑level tag, no raw content). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)
- `psychriskscore ∈ [0,1]` distress/arousal index \(R\). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)
- `sleeptoken ∈ [0,1]` sleep depth/pressure \(S\). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)
- `enstasisscore ∈ [0,1]` inner stability \(Es\). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)
- `offlineselfonly` (bool, non‑waivable) local N1/N2 automation only. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)
- `wakingexclusionzone` (bool, non‑waivable) blocks visuals in wake. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)

### 2.2 Automation state

- `dreamautonomybudget ∈ [0,1]` max gentle edits per 10 minutes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)
- `gentleeditdensity ∈ [0,1]` current micro‑intervention density. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)
- `dreamaiidleratio ∈ [0,1]` fraction of time with no edits (observe‑only). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)
- `n1scenecoherenceindex ∈ [0,1]` N1 scene continuity score. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)
- `n2fragmentationscore ∈ [0,1]` N2 fragmentation score. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)

### 2.3 XR node and consent

- `xrvisibility ∈ {visualon,visualoff}` visual channel state. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)
- `npclatencybudget` (ms) max reaction time for DreamAI. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)
- `safehapticenvelopeid` referencing low‑intensity N1/N2 profile. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/20edc2c7-9cfc-42e8-aef4-4be123bf34d3/xr-dreamscape-creating-many-lo-QupKQHe.TcmNYxfVJpjl7A.md)
- `AutomationConsentProfileId`, `automationtrustlevel ∈ {observe,assist,coach}`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)

## 3. Safety math and gates

### 3.1 Eligibility function

The main safety scalar is

\[
E = S \cdot (1 - R) \cdot Es
\]

where all inputs are clamped to \([0,1]\). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)

Reference rule in ALN:

```aln
rule,rule,expression,ComputeEligibilityE,eligibilityE sleeptoken 1.0 - psychriskscore enstasisscore,string,readonly,Safety eligibility ES1-REs
```

A session is eligible for N1/N2 automation when:

- `sleepstage ∈ {N1,N2}`
- `eligibilityE ≥ 0.4` (default threshold). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)

### 3.2 N1/N2 gate and wake exclusion

```aln
rule,rule,condition,N1N2Gate,sleepstage in N1,N2 and eligibilityE 0.4,string,readonly,Gate for N1N2 automation
rule,rule,condition,WakeNoVisuals,sleepstage wake and wakingexclusionzone true,string,readonly,Force visualoff
```

These rules are mirrored as pure Rust functions (no side effects), and every engine must reproduce the same outputs for the same inputs. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)

### 3.3 Intervention budget

- `dreamautonomybudget` is a per‑epoch cap derived from \(E\) and PSG‑measured micro‑arousal density. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)
- `gentleeditdensity` is updated as normalized edits per 10 minutes; new edits are blocked when density exceeds budget. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)

Rust example (already specified in your stack):

```rust
pub fn update_gentle_edit_density(
    automation: &mut AutomationState,
    edits_in_window: u32,
    window_seconds: u32,
    max_edits_per_10min: u32,
) {
    if window_seconds == 0 || max_edits_per_10min == 0 {
        automation.gentleeditdensity = 0.0;
        return;
    }
    let edits_per_10min =
        edits_in_window as f32 * 600.0 / window_seconds as f32;
    let density =
        (edits_per_10min / max_edits_per_10min as f32).clamp(0.0, 1.0);
    automation.gentleeditdensity = density;
}
```

This matches the ALN semantics and keeps interventions within clinically safe limits. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)

## 4. Behavior‑tree automation

dream_ai_n1n2_automation orchestrates a fixed set of safety‑audited behavior‑tree nodes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/d1daa70e-d522-42ba-9e9d-1ed5b7ca5371/the-paradigm-shift-from-photor-ca9cumL_SaSWQ8HTd23h7g.md)

### 4.1 Gating and brakes

- `SleepStageGateNode`  
  Only runs when `N1N2Gate` is true and micro‑arousal veto is false. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)
- `PsychRiskBrakeNode`  
  Scales intensity by \(1 - R\) and clamps to zero when risk is high. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)
- `MicroArousalCoolDownNode`  
  Inserts cooldown windows after arousals. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)
- `DreamAutonomyBudgetLimiterNode`  
  Enforces `dreamautonomybudget` and `gentleeditdensity` caps. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)

### 4.2 Low‑mass scaffolding objects

Allowed object types are intentionally simple, non‑narrative: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/c8108b62-2f83-4f0c-8dc7-a24ab3e5530f/dream-gaming-use-cases-and-hel-TcNV8XnDToWjCNTQ9ILKjw.md)

- `HypnoGuideBeacon`
- `BreathSyncHalo`
- `SoftAnchorPad`
- `NoiseDampeningShell`
- `SafeReturnTrail`

AI chats can *only* request placements of these objects with intensities; the runtime clamps final intensity using `PsychRiskBrakeNode` and budgets. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/c026ce21-b4eb-4a13-b098-785bba851c29/dreamscapes-and-xr-gaming-with-C7jqxNuHQWOXbqJfhdO_Og.md)

## 5. Neurorights and audit

The neurorights layer is encoded as non‑waivable flags and enforced at runtime. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ce486c90-2e6f-4d13-8ab0-f99109a75497/let-s-begin-defining-parameter-5o2YqjJCTdKx2m9mLrP5cA.md)

### 5.1 Guards

```aln
guard,policy,flag,mentalprivacy,true,bool,nonwaivable,No dreamcontentinnerspeech fields
guard,policy,flag,cognitiveliberty,true,bool,nonwaivable,Refusal never penalized
guard,policy,flag,mentalintegrity,true,bool,nonwaivable,Only low-intensity scaffolding
guard,policy,flag,nopunitivexr,true,bool,nonwaivable,No punitive mechanics
guard,policy,flag,soulnonaddressable,true,bool,nonwaivable,Souls never modeled
```

- No raw dream text, imagery, or inner speech is ever stored in N1/N2 shards. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)
- Refusing DreamAI or sleep‑XR cannot change access, pricing, or services. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ce486c90-2e6f-4d13-8ab0-f99109a75497/let-s-begin-defining-parameter-5o2YqjJCTdKx2m9mLrP5cA.md)
- No metrics may be used for punishment, monetization, or social scoring; violations are hash‑logged. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ce486c90-2e6f-4d13-8ab0-f99109a75497/let-s-begin-defining-parameter-5o2YqjJCTdKx2m9mLrP5cA.md)

### 5.2 Ledger

A redacted ledger records only infrastructure‑level events: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)

- `logid`, `eventtype ∈ {N1N2-start,N1N2-stop,MicroEdit,Brake,Cooldown,Audit}`
- `psychriskscoresnapshot`, `gentleeditdensitysnapshot`
- `neurorightsinvariantsheld` (bool, non‑waivable)
- No semantic content or soul‑like identifiers.

This ledger can be anchored to a neurorights audit chain (e.g., Dreamnet‑NeuroLedger‑01) without exposing personal dream content. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/4f7f2d13-acee-4a59-a4f0-5bf1a8910d67/highlight-the-facts-what-we-al-xQSwMmtWRDeZZm4SHpRf.g.md)

## 6. AI‑chat integration and accessibility

dream_ai_n1n2_automation is designed so that you can drive it from many chats with consistent, safe behavior. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/20edc2c7-9cfc-42e8-aef4-4be123bf34d3/xr-dreamscape-creating-many-lo-QupKQHe.TcmNYxfVJpjl7A.md)

### 6.1 Agentic tunnels (state‑only)

A shared ALN shard defines WebSocket dev tunnels for Mistral, Qwen, Perplexity, Grok and similar tools: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/20edc2c7-9cfc-42e8-aef4-4be123bf34d3/xr-dreamscape-creating-many-lo-QupKQHe.TcmNYxfVJpjl7A.md)

- Whitelisted outbound fields:

  - `sleepstage, psychriskscore, sleeptoken, enstasisscore`
  - `eligibilityE, dreamautonomybudget, gentleeditdensity`
  - `dreamaiidleratio, n1scenecoherenceindex, n2fragmentationscore`
  - Aggregated `consolehours, outdoorminutes` (habit telemetry). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)

- Blocked fields:

  - Any `dreamcontent, innerspeech, rawneuro, soultags`. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)

All traffic is logged with neurorights flags so regulators can verify compliance. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/03853798-5b50-4bd7-9ddf-2195c5d027e5/set-evil-rate-active-rogue-and-4aSOE.ZZRdSr9dYREa0NsA.md)

### 6.2 Three‑layer answer pattern (for accessibility)

When you ask any AI about this system, you can request answers in three layers: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/c026ce21-b4eb-4a13-b098-785bba851c29/dreamscapes-and-xr-gaming-with-C7jqxNuHQWOXbqJfhdO_Og.md)

1. Very simple English (short sentences, no jargon).
2. Exact ALN or Rust changes.
3. Optional detailed technical explanation.

This pattern is already compatible with your ALN‑first architecture and can be documented in `claude.md`, `deepseek.md`, or similar profiles in each repo. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/c026ce21-b4eb-4a13-b098-785bba851c29/dreamscapes-and-xr-gaming-with-C7jqxNuHQWOXbqJfhdO_Og.md)

## 7. Minimal ALN stub for this module

For quick reference, here is a compact ALN shard you can paste into any AI chat to talk about N1/N2 automation state without exposing content: [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/ccf2e345-558a-4d2d-a08d-47db96d76f09/this-research-focuses-on-dual-_xrOtPT6T_6Zv0AFs0CZeQ.md)

```aln
csv aln
path,entitytype,field,key,value,datatype,constraints,notes
dreamain1n2automation,meta,scalar,shardid,dreamain1n2automation.v1,string,primarykey,Shard ID
dreamain1n2automation,meta,scalar,role,N1N2-Automation,string,nonnull,Low-intensity DreamAI scaffolding

SECTION,SUBJECT
subject,subject,scalar,subjectid,,string,primarykey,De-identified participant
subject,subject,enum,sleepstage,wake,string,wake,N1,N2,N3,REM,Polysomnographic stage
subject,subject,enum,dreammode,none,string,none,nonlucid,prelucid,lucid,nightmare,Dream mode
subject,subject,scalar,psychriskscore,0.0,float,range0,1,Distress-arousal index R
subject,subject,scalar,sleeptoken,1.0,float,range0,1,Sleep token S
subject,subject,scalar,enstasisscore,1.0,float,range0,1,Inner stability Es
subject,subject,flag,offlineselfonly,true,bool,nonwaivable,Local-only N1N2 mode
subject,subject,flag,wakingexclusionzone,true,bool,nonwaivable,Block visuals when wake

SECTION,AUTOMATION-STATE
auto,state,scalar,dreamautonomybudget,0.2,float,range0,1,Max edits per 10 min
auto,state,scalar,gentleeditdensity,0.0,float,range0,1,Observed edit density
auto,state,scalar,dreamaiidleratio,1.0,float,range0,1,Idle fraction
auto,state,scalar,n1scenecoherenceindex,0.5,float,range0,1,N1 coherence
auto,state,scalar,n2fragmentationscore,0.0,float,range0,1,N2 fragmentation

SECTION,RUNTIME-RULES
rule,rule,expression,ComputeEligibilityE,eligibilityE sleeptoken 1.0 - psychriskscore enstasisscore,string,readonly,ES1-REs
rule,rule,condition,N1N2Gate,sleepstage in N1,N2 and eligibilityE 0.4,string,readonly,N1N2 automation gate
rule,rule,expression,ClampGentleEdits,if gentleeditdensity dreamautonomybudget then blocknewedits,string,readonly,Cap micro-edits

SECTION,NEURORIGHTS-GUARDS
guard,policy,flag,mentalprivacy,true,bool,nonwaivable,No dreamcontent or innerspeech
guard,policy,flag,cognitiveliberty,true,bool,nonwaivable,Refusal never penalized
guard,policy,flag,mentalintegrity,true,bool,nonwaivable,Only low-intensity scaffolding
guard,policy,flag,nopunitivexr,true,bool,nonwaivable,No punitive mechanics
guard,policy,flag,soulnonaddressable,true,bool,nonwaivable,Souls never modeled

FOOTER,END-OF-SHARD
```

Any AI that can read text can use this stub to reason about your N1/N2 automation logic without needing private repository access or GitHub workflows. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_f31734a0-1254-4680-b685-d4ea4701e30e/9afbcfcc-e092-4d31-9c20-68d917227457/below-are-50-candidate-terms-p-tzEXNSJoSBmzojdAxlu1LQ.md)
