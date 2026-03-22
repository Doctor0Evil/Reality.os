-- =============================================================================
-- FILE: sabotage_detector.lua
-- PROJECT: Reality.os / SovereigntyCore
-- MODULE: Policies / Evolution Gating & Sabotage Detection
-- VERSION: 1.0.0
-- LICENSE: ALN-Sovereign-1.0 (Neurorights-Compliant, Anti-Discrimination)
-- AUTHOR: OrganicCPU Runtime (Host DID: 0xB05TR0M...50VERE1GN)
-- CREATED: 2026-03-22
-- LAST_AUDIT: 2026-03-22T00:00:00Z
-- JURISDICTION: Phoenix_AZ, Santiago_CL, Sacramento_CA, Denver_CO, Brussels_BE
-- =============================================================================
-- DESCRIPTION:
--   Lua policy engine for sabotage risk evaluation, CI/CD pipeline integration,
--   OTA evolution gating, and monotonic evolution enforcement. Includes explicit
--   anti-discrimination safeguards to prevent racial profiling or biased
--   classification of augmented citizens. All policy decisions are logged to
--   QPU.Datashard with Googolswarm anchoring for court-admissible audit trails.
-- =============================================================================

-- =============================================================================
-- STRICT MODE & SECURITY SETTINGS
-- =============================================================================

setfenv(1, setmetatable({}, {
    __index = _G,
    __newindex = function(t, k, v)
        -- Prevent global pollution
        error("Attempt to write to global scope: " .. tostring(k), 2)
    end
}))

-- =============================================================================
-- MODULE IMPORTS & DEPENDENCIES
-- =============================================================================

local json = require("dkjson")
local crypto = require("sovereignty.crypto")
local aln = require("sovereignty.aln_parser")
local datashard = require("sovereignty.qpu_datashard")
local googolswarm = require("sovereignty.googolswarm_anchor")
local brainidentity = require("sovereignty.brainidentity")
local rohmodel = require("sovereignty.rohmodel")
local telemetry = require("sovereignty.biophysical_telemetry")
local chrono = require("chrono")
local inspect = require("inspect")

-- =============================================================================
-- CONSTANTS & CONFIGURATION
-- =============================================================================

local MODULE_NAME = "sabotage_detector_lua"
local MODULE_VERSION = "1.0.0"
local POLICY_SHARD_PATH = "Config/Sovereignty/policies/bostrom-blacklist-v1.aln"
local EVOLUTION_LOG_PATH = ".evolve.jsonl"
local DONUTLOOP_PATH = ".donutloop.aln"

-- Sabotage risk thresholds (aligned with bostrom-blacklist-v1.aln)
local THRESHOLD_DENY = 0.70
local THRESHOLD_UNSAFE_DEFER = 0.85
local THRESHOLD_HARD_FAIL = 1.00
local THRESHOLD_INHIBIT_BIOPHYSICAL = 0.60
local THRESHOLD_INHIBIT_ACCESS = 0.75

-- Monotonic evolution constraints
local KF_MIN_DELTA = 0.0  -- Knowledge Factor cannot decrease
local ROH_MAX_DELTA = 0.0  -- Risk of Harm cannot increase

-- Anti-discrimination safeguards
local PROTECTED_ATTRIBUTES = {
    "race",
    "ethnicity",
    "national_origin",
    "skin_color",
    "genetic_ancestry",
    "phenotypic_markers",
    "cultural_background",
    "language_origin",
    "religious_association",
    "geographic_origin"
}

local DISCRIMINATION_RISK_THRESHOLD = 0.3  -- Any proposal with discrimination risk > 0.3 is rejected

-- =============================================================================
-- DATA STRUCTURES
-- =============================================================================

--- Evolution proposal structure
local EvolutionProposal = {}
EvolutionProposal.__index = EvolutionProposal

function EvolutionProposal.new(data)
    local self = setmetatable({}, EvolutionProposal)
    self.id = data.id or crypto.generate_uuid()
    self.identifier = data.identifier or ""
    self.signer_did = data.signer_did or ""
    self.timestamp = data.timestamp or chrono.utcnow()
    self.scope = data.scope or "standard"
    self.commit_hash = data.commit_hash or ""
    self.commit_metadata = data.commit_metadata or ""
    self.firmware_update = data.firmware_update or nil
    self.accesses_inner_devices = data.accesses_inner_devices or false
    self.has_guarded_path = data.has_guarded_path or false
    self.references_quarantined_sko = data.references_quarantined_sko or false
    self.kf_before = data.kf_before or 0.0
    self.kf_after = data.kf_after or 0.0
    self.roh_before = data.roh_before or 0.0
    self.roh_after = data.roh_after or 0.0
    self.protected_attribute_changes = data.protected_attribute_changes or {}
    return self
end

function EvolutionProposal:compute_hash()
    local data = {
        id = self.id,
        identifier = self.identifier,
        signer_did = self.signer_did,
        timestamp = self.timestamp,
        commit_hash = self.commit_hash
    }
    return crypto.sha256_hex(json.encode(data))
end

function EvolutionProposal:validate_monotonicity()
    -- Knowledge Factor must not decrease
    if self.kf_after < self.kf_before - KF_MIN_DELTA then
        return false, "KF_DECREASE: Knowledge Factor cannot decrease (before: " .. 
               tostring(self.kf_before) .. ", after: " .. tostring(self.kf_after) .. ")"
    end
    
    -- Risk of Harm must not increase
    if self.roh_after > self.roh_before + ROH_MAX_DELTA then
        return false, "ROH_INCREASE: Risk of Harm cannot increase (before: " .. 
               tostring(self.roh_before) .. ", after: " .. tostring(self.roh_after) .. ")"
    end
    
    return true, "MONOTONICITY_OK"
end

function EvolutionProposal:check_discrimination_risk()
    local risk_score = 0.0
    local violations = {}
    
    for _, attr in ipairs(PROTECTED_ATTRIBUTES) do
        if self.protected_attribute_changes[attr] then
            risk_score = risk_score + 0.5
            table.insert(violations, "PROTECTED_ATTR_" .. attr:upper())
        end
    end
    
    -- Check for proxy discrimination patterns
    if self.identifier:match("[Pp]rofile") or self.identifier:match("[Cc]lassify") then
        if self.identifier:match("[Rr]ace") or self.identifier:match("[Ee]thnic") then
            risk_score = risk_score + 0.8
            table.insert(violations, "PROXY_DISCRIMINATION_DETECTED")
        end
    end
    
    -- Check for geographic or linguistic profiling
    if self.commit_metadata:match("[Gg]eographic.*[Pp]rofile") or 
       self.commit_metadata:match("[Ll]inguistic.*[Aa]nalysis") then
        risk_score = risk_score + 0.6
        table.insert(violations, "GEOGRAPHIC_LINGUISTIC_PROFILING")
    end
    
    return risk_score, violations
end

--- Sabotage risk scalar structure
local SabotageRiskScalar = {}
SabotageRiskScalar.__index = SabotageRiskScalar

function SabotageRiskScalar.new()
    local self = setmetatable({}, SabotageRiskScalar)
    self.total = 0.0
    self.provenance = 0.0
    self.blacklist = 0.0
    self.biophysical = 0.0
    self.integrity = 0.0
    self.access = 0.0
    self.knowledge = 0.0
    self.discrimination = 0.0
    return self
end

function SabotageRiskScalar:compute_total()
    self.total = math.min(1.0, math.max(0.0,
        self.provenance +
        self.blacklist +
        self.biophysical +
        self.integrity +
        self.access +
        self.knowledge +
        self.discrimination
    ))
    return self.total
end

--- Sabotage event structure (mirrors Rust SabotageEvent)
local SabotageEvent = {}
SabotageEvent.__index = SabotageEvent

function SabotageEvent.new(event_id, risk_scalar, action, proposal)
    local self = setmetatable({}, SabotageEvent)
    self.event_id = event_id
    self.timestamp_utc = chrono.utcnow()
    self.brainidentity_hash = brainidentity.get_hash()
    self.sabotage_risk_scalar = risk_scalar.total
    self.provenance_factor = risk_scalar.provenance
    self.blacklist_factor = risk_scalar.blacklist
    self.biophysical_factor = risk_scalar.biophysical
    self.integrity_factor = risk_scalar.integrity
    self.access_factor = risk_scalar.access
    self.knowledge_factor = risk_scalar.knowledge
    self.discrimination_factor = risk_scalar.discrimination
    self.triggering_artifact_hash = proposal:compute_hash()
    self.jurisdiction_tags = {}
    self.neurorights_violations = {}
    self.host_did_signature = ""
    self.googolswarm_anchor_txid = nil
    self.severity = "LOW"
    self.action_taken = action
    return self
end

function SabotageEvent:set_severity(action)
    local severity_map = {
        ["LOG_ONLY"] = "LOW",
        ["DENY"] = "MEDIUM",
        ["INHIBIT"] = "MEDIUM",
        ["UNSAFE_DEFER"] = "HIGH",
        ["HARD_FAIL"] = "CRITICAL"
    }
    self.severity = severity_map[action] or "LOW"
end

function SabotageEvent:add_neurorights_violation(violation_type)
    table.insert(self.neurorights_violations, violation_type)
end

function SabotageEvent:add_jurisdiction_tag(tag)
    table.insert(self.jurisdiction_tags, tag)
end

function SabotageEvent:sign(host_keypair)
    local payload = json.encode({
        event_id = self.event_id,
        timestamp = self.timestamp_utc,
        artifact_hash = self.triggering_artifact_hash
    })
    self.host_did_signature = crypto.sign_ed25519(host_keypair, payload)
end

function SabotageEvent:to_dict()
    return {
        event_id = self.event_id,
        timestamp_utc = self.timestamp_utc,
        brainidentity_hash = self.brainidentity_hash,
        sabotage_risk_scalar = self.sabotage_risk_scalar,
        provenance_factor = self.provenance_factor,
        blacklist_factor = self.blacklist_factor,
        biophysical_factor = self.biophysical_factor,
        integrity_factor = self.integrity_factor,
        access_factor = self.access_factor,
        knowledge_factor = self.knowledge_factor,
        discrimination_factor = self.discrimination_factor,
        triggering_artifact_hash = self.triggering_artifact_hash,
        jurisdiction_tags = self.jurisdiction_tags,
        neurorights_violations = self.neurorights_violations,
        host_did_signature = self.host_did_signature,
        googolswarm_anchor_txid = self.googolswarm_anchor_txid,
        severity = self.severity,
        action_taken = self.action_taken
    }
end

-- =============================================================================
-- BLACKLIST PATTERN MATCHING
-- =============================================================================

local BlacklistMatcher = {}
BlacklistMatcher.__index = BlacklistMatcher

function BlacklistMatcher.new(aln_shard_path)
    local self = setmetatable({}, BlacklistMatcher)
    self.patterns = {}
    self.risk_rules = {}
    self.thresholds = {}
    self.neurorights_bindings = {}
    self:load_aln_shard(aln_shard_path)
    return self
end

function BlacklistMatcher:load_aln_shard(path)
    local shard_data = aln.load_shard(path)
    
    for _, entry in ipairs(shard_data.entries or {}) do
        if entry.metric == "blacklist_pattern" then
            table.insert(self.patterns, {
                metric = entry.metric,
                domain = entry.domain,
                module = entry.module,
                op = entry.op,
                pattern = entry.pattern,
                kind = entry.kind,
                reason = entry.reason,
                source = entry.source
            })
        elseif entry.metric == "sabotage_rule" then
            table.insert(self.risk_rules, {
                rule_id = entry.rule_id,
                component = entry.component,
                weight = tonumber(entry.weight) or 0.0,
                description = entry.description
            })
        elseif entry.metric == "sabotage_threshold" then
            self.thresholds[entry.threshold_id] = {
                scalar = entry.scalar,
                value = tonumber(entry.value) or 0.0,
                action = entry.action,
                log_event = entry.log_event
            }
        elseif entry.metric == "neurorights_binding" then
            table.insert(self.neurorights_bindings, {
                jurisdiction_id = entry.jurisdiction_id,
                jurisdiction = entry.jurisdiction,
                right_type = entry.right_type,
                severity_multiplier = tonumber(entry.severity_multiplier) or 1.0
            })
        end
    end
end

function BlacklistMatcher:matches(identifier, pattern_entry)
    local op = pattern_entry.op
    local pattern = pattern_entry.pattern
    
    if op == "eq" then
        return identifier == pattern
    elseif op == "prefix" then
        return identifier:sub(1, #pattern) == pattern
    elseif op == "glob" then
        if pattern == "*" then
            return true
        elseif pattern:sub(1, 2) == "*." then
            local ext = pattern:sub(2)
            return identifier:sub(-#ext) == ext
        end
    elseif op == "regex" then
        local success, result = pcall(function()
            return identifier:match(pattern)
        end)
        return success and result ~= nil
    end
    
    return false
end

function BlacklistMatcher:check_all_patterns(identifier)
    local matched = false
    local matched_patterns = {}
    
    for _, pattern in ipairs(self.patterns) do
        if self:matches(identifier, pattern) then
            matched = true
            table.insert(matched_patterns, pattern)
        end
    end
    
    return matched, matched_patterns
end

function BlacklistMatcher:get_risk_weight(rule_id)
    for _, rule in ipairs(self.risk_rules) do
        if rule.rule_id == rule_id then
            return rule.weight
        end
    end
    return 0.0
end

function BlacklistMatcher:get_threshold(threshold_id)
    return self.thresholds[threshold_id]
end

-- =============================================================================
-- MAIN SABOTAGE DETECTOR CLASS
-- =============================================================================

local SabotageDetector = {}
SabotageDetector.__index = SabotageDetector

function SabotageDetector.new(config)
    local self = setmetatable({}, SabotageDetector)
    
    self.config = config or {}
    self.host_did = config.host_did or crypto.generate_did()
    self.host_keypair = config.host_keypair or crypto.generate_keypair()
    self.blacklist_matcher = BlacklistMatcher.new(POLICY_SHARD_PATH)
    self.event_id_counter = self:load_event_id_counter()
    self.qpu_datashard = datashard.new("SABOTAGEEVENT")
    self.googolswarm_client = googolswarm.new()
    
    return self
end

function SabotageDetector:load_event_id_counter()
    local last_event = self.qpu_datashard:get_last_event()
    if last_event and last_event.event_id then
        return last_event.event_id
    end
    return 0
end

function SabotageDetector:evaluate_proposal(proposal, telemetry_snapshot)
    local risk_scalar = SabotageRiskScalar.new()
    
    -- Tier 1: System-level provenance (hard gate)
    self:evaluate_provenance(proposal, risk_scalar)
    
    -- Tier 1: Firmware integrity check
    self:evaluate_firmware_integrity(proposal, risk_scalar)
    
    -- Tier 2: Blacklist pattern matching
    self:evaluate_blacklist_patterns(proposal, risk_scalar)
    
    -- Tier 2: Biophysical stress correlation
    self:evaluate_biophysical_stress(telemetry_snapshot, risk_scalar)
    
    -- Tier 3: Access pattern analysis (ghost-access)
    self:evaluate_access_patterns(proposal, risk_scalar)
    
    -- Tier 3: SKO contamination risk
    self:evaluate_knowledge_contamination(proposal, risk_scalar)
    
    -- Anti-discrimination check (CRITICAL)
    self:evaluate_discrimination_risk(proposal, risk_scalar)
    
    -- Compute total risk
    risk_scalar:compute_total()
    
    -- Determine action based on thresholds
    local action = self:determine_action(risk_scalar)
    
    return risk_scalar, action
end

function SabotageDetector:evaluate_provenance(proposal, risk_scalar)
    -- Check if proposal signer is trusted
    local trusted_dids = {
        self.host_did,
        "did:organiccpu:runtime:0xB05TR0M...50VERE1GN"
    }
    
    local is_trusted = false
    for _, did in ipairs(trusted_dids) do
        if proposal.signer_did == did then
            is_trusted = true
            break
        end
    end
    
    if not is_trusted then
        risk_scalar.provenance = self.blacklist_matcher:get_risk_weight("ent_untrusted")
    end
    
    -- Check for enterprise CI prefix
    if proposal.commit_metadata and proposal.commit_metadata:match("^enterprise_ci_") then
        risk_scalar.provenance = math.max(risk_scalar.provenance, 
            self.blacklist_matcher:get_risk_weight("ent_untrusted"))
    end
end

function SabotageDetector:evaluate_firmware_integrity(proposal, risk_scalar)
    if proposal.firmware_update then
        if not proposal.firmware_update.is_did_signed then
            risk_scalar.integrity = self.blacklist_matcher:get_risk_weight("firmware_mismatch")
        end
    end
end

function SabotageDetector:evaluate_blacklist_patterns(proposal, risk_scalar)
    local matched, matched_patterns = self.blacklist_matcher:check_all_patterns(proposal.identifier)
    
    if matched then
        risk_scalar.blacklist = self.blacklist_matcher:get_risk_weight("blacklist_match")
        
        -- Log each matched pattern
        for _, pattern in ipairs(matched_patterns) do
            self:log_pattern_match(pattern, proposal.identifier)
        end
    end
    
    -- Check scope factor for core safety crates
    if proposal.scope == "orchestrator" or proposal.scope == "core_safety" then
        risk_scalar.blacklist = math.max(risk_scalar.blacklist,
            self.blacklist_matcher:get_risk_weight("scope_high"))
    end
end

function SabotageDetector:evaluate_biophysical_stress(telemetry_snapshot, risk_scalar)
    if not telemetry_snapshot then
        return
    end
    
    -- Check for EEG/HRV stress spikes
    if telemetry_snapshot.eeg_stress_ratio > 0.7 or 
       telemetry_snapshot.hrv_anomaly_index > 0.6 then
        risk_scalar.biophysical = self.blacklist_matcher:get_risk_weight("psych_spike")
    end
    
    -- Check for nanoswarm weaponization indicators
    if telemetry_snapshot.nanoswarm_density > 0.8 and 
       telemetry_snapshot.sleep_corridor_violated then
        risk_scalar.biophysical = math.max(risk_scalar.biophysical,
            self.blacklist_matcher:get_risk_weight("psych_spike"))
    end
end

function SabotageDetector:evaluate_access_patterns(proposal, risk_scalar)
    -- Check for ghost-access patterns
    if proposal.accesses_inner_devices and not proposal.has_guarded_path then
        risk_scalar.access = self.blacklist_matcher:get_risk_weight("ghost_path")
    end
end

function SabotageDetector:evaluate_knowledge_contamination(proposal, risk_scalar)
    if proposal.references_quarantined_sko then
        risk_scalar.knowledge = self.blacklist_matcher:get_risk_weight("skO_contamination")
    end
end

function SabotageDetector:evaluate_discrimination_risk(proposal, risk_scalar)
    -- CRITICAL: Anti-discrimination safeguard
    local disc_risk, violations = proposal:check_discrimination_risk()
    
    if disc_risk > DISCRIMINATION_RISK_THRESHOLD then
        risk_scalar.discrimination = math.min(1.0, disc_risk)
        
        -- Log discrimination attempt as neurorights violation
        for _, violation in ipairs(violations) do
            self:log_discrimination_attempt(violation, proposal)
        end
    end
end

function SabotageDetector:determine_action(risk_scalar)
    -- Check critical thresholds first
    if risk_scalar.integrity >= 1.0 then
        return "HARD_FAIL"
    end
    
    if risk_scalar.total >= THRESHOLD_UNSAFE_DEFER then
        return "UNSAFE_DEFER"
    end
    
    if risk_scalar.total >= THRESHOLD_DENY then
        return "DENY"
    end
    
    if risk_scalar.biophysical >= THRESHOLD_INHIBIT_BIOPHYSICAL then
        return "INHIBIT"
    end
    
    if risk_scalar.access >= THRESHOLD_INHIBIT_ACCESS then
        return "DENY"
    end
    
    if risk_scalar.discrimination >= DISCRIMINATION_RISK_THRESHOLD then
        return "DENY"
    end
    
    return "LOG_ONLY"
end

function SabotageDetector:generate_sabotage_event(risk_scalar, action, proposal)
    self.event_id_counter = self.event_id_counter + 1
    
    local event = SabotageEvent.new(self.event_id_counter, risk_scalar, action, proposal)
    event:set_severity(action)
    
    -- Add jurisdiction tags from neurorights bindings
    for _, binding in ipairs(self.blacklist_matcher.neurorights_bindings) do
        event:add_jurisdiction_tag(binding.jurisdiction_id)
        
        -- Map risk factors to neurorights violations
        if risk_scalar.biophysical > 0.5 and binding.right_type == "mental_integrity" then
            event:add_neurorights_violation("MENTAL_INTEGRITY_VIOLATION")
        end
        if risk_scalar.provenance > 0.5 and binding.right_type == "cognitive_liberty" then
            event:add_neurorights_violation("COGNITIVE_LIBERTY_VIOLATION")
        end
        if risk_scalar.discrimination > 0.3 and binding.right_type == "neural_data_privacy" then
            event:add_neurorights_violation("DISCRIMINATION_NEURAL_DATA")
        end
    end
    
    -- Sign event with host DID
    event:sign(self.host_keypair)
    
    -- Write to QPU.Datashard
    self:write_to_datashard(event)
    
    -- Anchor to Googolswarm
    local anchor_txid = self:anchor_to_googolswarm(event)
    event.googolswarm_anchor_txid = anchor_txid
    
    -- Update event record with anchor
    self:update_event_anchor(event)
    
    -- Log to DonutLoop
    self:log_to_donutloop(event)
    
    return event
end

function SabotageDetector:log_pattern_match(pattern, identifier)
    local log_entry = {
        event_type = "blacklist_pattern_match",
        pattern = pattern.pattern,
        kind = pattern.kind,
        reason = pattern.reason,
        matched_identifier = identifier,
        timestamp = chrono.utcnow()
    }
    
    datashard.append_log(DONUTLOOP_PATH, log_entry)
end

function SabotageDetector:log_discrimination_attempt(violation, proposal)
    local log_entry = {
        event_type = "discrimination_attempt_detected",
        violation = violation,
        proposal_id = proposal.id,
        identifier = proposal.identifier,
        signer_did = proposal.signer_did,
        timestamp = chrono.utcnow(),
        severity = "CRITICAL",
        neurorights_violation = true
    }
    
    datashard.append_log(DONUTLOOP_PATH, log_entry)
end

function SabotageDetector:write_to_datashard(event)
    local record = event:to_dict()
    self.qpu_datashard:append("SABOTAGEEVENT", record)
end

function SabotageDetector:anchor_to_googolswarm(event)
    local txid = self.googolswarm_client:anchor_event(event:to_dict())
    return txid
end

function SabotageDetector:update_event_anchor(event)
    self.qpu_datashard:update_anchor(event.event_id, event.googolswarm_anchor_txid)
end

function SabotageDetector:log_to_donutloop(event)
    local log_entry = {
        event_type = "SABOTAGEEVENT",
        event_id = event.event_id,
        timestamp = event.timestamp_utc,
        sabotage_risk = event.sabotage_risk_scalar,
        severity = event.severity,
        action = event.action_taken,
        neurorights_violations = event.neurorights_violations,
        googolswarm_anchor = event.googolswarm_anchor_txid
    }
    
    datashard.append_log(DONUTLOOP_PATH, log_entry)
end

function SabotageDetector:validate_proposal(proposal, telemetry_snapshot)
    -- First check monotonicity constraints
    local monotonic_ok, monotonic_msg = proposal:validate_monotonicity()
    if not monotonic_ok then
        local risk_scalar = SabotageRiskScalar.new()
        risk_scalar.total = 1.0
        risk_scalar.knowledge = 1.0
        local event = self:generate_sabotage_event(risk_scalar, "HARD_FAIL", proposal)
        error("MONOTONICITY_VIOLATION: " .. monotonic_msg)
    end
    
    -- Then evaluate sabotage risk
    local risk_scalar, action = self:evaluate_proposal(proposal, telemetry_snapshot)
    
    if action ~= "LOG_ONLY" then
        local event = self:generate_sabotage_event(risk_scalar, action, proposal)
        
        if action == "HARD_FAIL" then
            error("SABOTAGE_DETECTED_HARD_FAIL: sabotage_risk=" .. tostring(risk_scalar.total))
        elseif action == "UNSAFE_DEFER" then
            error("SABOTAGE_DETECTED_UNSAFE_DEFER: sabotage_risk=" .. tostring(risk_scalar.total))
        elseif action == "DENY" then
            error("SABOTAGE_DETECTED_DENY: sabotage_risk=" .. tostring(risk_scalar.total))
        end
    end
    
    return true, risk_scalar, action
end

-- =============================================================================
-- CI/CD PIPELINE INTEGRATION
-- =============================================================================

local CICDPipeline = {}
CICDPipeline.__index = CICDPipeline

function CICDPipeline.new(detector)
    local self = setmetatable({}, CICDPipeline)
    self.detector = detector
    return self
end

function CICDPipeline:validate_commit(commit_data, telemetry_snapshot)
    local proposal = EvolutionProposal.new({
        id = commit_data.id,
        identifier = commit_data.identifier,
        signer_did = commit_data.signer_did,
        commit_hash = commit_data.hash,
        commit_metadata = commit_data.metadata,
        scope = commit_data.scope or "standard",
        kf_before = commit_data.kf_before or 0.0,
        kf_after = commit_data.kf_after or 0.0,
        roh_before = commit_data.roh_before or 0.0,
        roh_after = commit_data.roh_after or 0.0
    })
    
    local success, result = pcall(function()
        return self.detector:validate_proposal(proposal, telemetry_snapshot)
    end)
    
    if not success then
        return false, result
    end
    
    return true, result
end

function CICDPipeline:validate_ota_update(ota_manifest, telemetry_snapshot)
    local proposal = EvolutionProposal.new({
        id = ota_manifest.id,
        identifier = ota_manifest.identifier,
        signer_did = ota_manifest.signer_did,
        firmware_update = {
            is_did_signed = ota_manifest.is_did_signed,
            firmware_hash = ota_manifest.firmware_hash
        },
        scope = "firmware",
        kf_before = ota_manifest.kf_before or 0.0,
        kf_after = ota_manifest.kf_after or 0.0,
        roh_before = ota_manifest.roh_before or 0.0,
        roh_after = ota_manifest.roh_after or 0.0
    })
    
    local success, result = pcall(function()
        return self.detector:validate_proposal(proposal, telemetry_snapshot)
    end)
    
    if not success then
        return false, result
    end
    
    return true, result
end

-- =============================================================================
-- OTA EVOLUTION GATING
-- =============================================================================

local OTAEvolutionGate = {}
OTAEvolutionGate.__index = OTAEvolutionGate

function OTAEvolutionGate.new(detector)
    local self = setmetatable({}, OTAEvolutionGate)
    self.detector = detector
    self.pending_updates = {}
    return self
end

function OTAEvolutionGate:submit_update(update_manifest, telemetry_snapshot)
    local proposal = EvolutionProposal.new({
        id = update_manifest.id,
        identifier = update_manifest.identifier,
        signer_did = update_manifest.signer_did,
        firmware_update = {
            is_did_signed = update_manifest.is_did_signed,
            firmware_hash = update_manifest.firmware_hash
        },
        accesses_inner_devices = update_manifest.accesses_inner_devices or false,
        has_guarded_path = update_manifest.has_guarded_path or false,
        scope = "ota_evolution",
        kf_before = update_manifest.kf_before or 0.0,
        kf_after = update_manifest.kf_after or 0.0,
        roh_before = update_manifest.roh_before or 0.0,
        roh_after = update_manifest.roh_after or 0.0
    })
    
    local success, result = pcall(function()
        return self.detector:validate_proposal(proposal, telemetry_snapshot)
    end)
    
    if not success then
        self.pending_updates[update_manifest.id] = {
            status = "REJECTED",
            reason = result,
            timestamp = chrono.utcnow()
        }
        return false, result
    end
    
    self.pending_updates[update_manifest.id] = {
        status = "APPROVED",
        risk_scalar = result,
        timestamp = chrono.utcnow()
    }
    
    return true, result
end

function OTAEvolutionGate:get_update_status(update_id)
    return self.pending_updates[update_id]
end

-- =============================================================================
-- ANTI-DISCRIMINATION SAFEGUARDS (CRITICAL)
-- =============================================================================

local AntiDiscriminationGuard = {}
AntiDiscriminationGuard.__index = AntiDiscriminationGuard

function AntiDiscriminationGuard.new()
    local self = setmetatable({}, AntiDiscriminationGuard)
    self.protected_attributes = PROTECTED_ATTRIBUTES
    self.violation_log = {}
    return self
end

function AntiDiscriminationGuard:scan_proposal(proposal)
    local violations = {}
    
    -- Scan identifier for discriminatory language
    for _, attr in ipairs(self.protected_attributes) do
        if proposal.identifier:lower():match(attr) then
            table.insert(violations, {
                type = "IDENTIFIER_CONTAINS_PROTECTED_ATTR",
                attribute = attr,
                identifier = proposal.identifier
            })
        end
    end
    
    -- Scan commit metadata for profiling intent
    if proposal.commit_metadata then
        local profiling_patterns = {
            "profile.*race",
            "classify.*ethnic",
            "score.*genetic",
            "risk.*ancestry",
            "predict.*phenotyp"
        }
        
        for _, pattern in ipairs(profiling_patterns) do
            if proposal.commit_metadata:lower():match(pattern) then
                table.insert(violations, {
                    type = "COMMIT_METADATA_PROFILING_PATTERN",
                    pattern = pattern,
                    metadata = proposal.commit_metadata
                })
            end
        end
    end
    
    -- Check for protected attribute changes
    for attr, _ in pairs(proposal.protected_attribute_changes or {}) do
        table.insert(violations, {
            type = "PROTECTED_ATTRIBUTE_MODIFICATION",
            attribute = attr
        })
    end
    
    return violations
end

function AntiDiscriminationGuard:log_violation(violation, proposal)
    table.insert(self.violation_log, {
        violation = violation,
        proposal_id = proposal.id,
        timestamp = chrono.utcnow(),
        logged = true
    })
    
    -- Also log to DonutLoop for audit trail
    datashard.append_log(DONUTLOOP_PATH, {
        event_type = "ANTI_DISCRIMINATION_VIOLATION",
        violation = violation,
        proposal_id = proposal.id,
        timestamp = chrono.utcnow()
    })
end

function AntiDiscriminationGuard:get_violation_history()
    return self.violation_log
end

-- =============================================================================
-- MONOTONIC EVOLUTION ENFORCEMENT
-- =============================================================================

local MonotonicEvolutionGuard = {}
MonotonicEvolutionGuard.__index = MonotonicEvolutionGuard

function MonotonicEvolutionGuard.new()
    local self = setmetatable({}, MonotonicEvolutionGuard)
    self.evolution_history = {}
    return self
end

function MonotonicEvolutionGuard:record_evolution(proposal, kf_delta, roh_delta)
    table.insert(self.evolution_history, {
        proposal_id = proposal.id,
        timestamp = chrono.utcnow(),
        kf_before = proposal.kf_before,
        kf_after = proposal.kf_after,
        kf_delta = kf_delta,
        roh_before = proposal.roh_before,
        roh_after = proposal.roh_after,
        roh_delta = roh_delta,
        monotonic_compliant = (kf_delta >= KF_MIN_DELTA) and (roh_delta <= ROH_MAX_DELTA)
    })
end

function MonotonicEvolutionGuard:verify_non_reversibility(proposal)
    -- Ensure no previous evolution can be reversed
    for _, hist in ipairs(self.evolution_history) do
        if proposal.kf_after < hist.kf_after then
            return false, "REVERSAL_DETECTED: KF would decrease from previous state"
        end
        if proposal.roh_after > hist.roh_after then
            return false, "REVERSAL_DETECTED: RoH would increase from previous state"
        end
    end
    return true, "NON_REVERSIBLE_OK"
end

function MonotonicEvolutionGuard:get_evolution_chain()
    return self.evolution_history
end

-- =============================================================================
-- EXPORTED API FUNCTIONS
-- =============================================================================

local _M = {}

function _M.create_detector(config)
    return SabotageDetector.new(config)
end

function _M.create_cicd_pipeline(detector)
    return CICDPipeline.new(detector)
end

function _M.create_ota_gate(detector)
    return OTAEvolutionGate.new(detector)
end

function _M.create_anti_discrimination_guard()
    return AntiDiscriminationGuard.new()
end

function _M.create_monotonic_guard()
    return MonotonicEvolutionGuard.new()
end

function _M.validate_evolution_proposal(detector, proposal, telemetry)
    return detector:validate_proposal(proposal, telemetry)
end

function _M.check_monotonicity(proposal)
    return proposal:validate_monotonicity()
end

function _M.check_discrimination_risk(proposal)
    return proposal:check_discrimination_risk()
end

return _M

-- =============================================================================
-- END OF FILE
-- =============================================================================
