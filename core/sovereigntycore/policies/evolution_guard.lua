-- =============================================================================
-- FILE: evolution_guard.lua
-- PROJECT: Reality.os / SovereigntyCore
-- MODULE: Policies / Evolution Guard & Monotonicity Enforcement
-- VERSION: 1.0.0
-- LICENSE: ALN-Sovereign-1.0 (Neurorights-Compliant, Anti-Discrimination)
-- AUTHOR: OrganicCPU Runtime (Host DID: 0xB05TR0M...50VERE1GN)
-- CREATED: 2026-03-22
-- LAST_AUDIT: 2026-03-22T00:00:00Z
-- JURISDICTION: Phoenix_AZ, Santiago_CL, Sacramento_CA, Denver_CO, Brussels_BE
-- =============================================================================
-- DESCRIPTION:
--   Lua evolution guard module enforcing monotonic evolution constraints,
--   KF/RoH corridor validation, and non-reversibility verification. All
--   evolution events are logged to QPU.Datashard with Googolswarm anchoring
--   for court-admissible audit trails. Includes anti-discrimination safeguards
--   and protected attribute scanning to prevent racial profiling of augmented
--   citizens.
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
local chrono = require("chrono")
local inspect = require("inspect")

-- =============================================================================
-- CONSTANTS & CONFIGURATION
-- =============================================================================

local MODULE_NAME = "evolution_guard_lua"
local MODULE_VERSION = "1.0.0"
local EVOLUTION_LOG_PATH = ".evolve.jsonl"
local DONUTLOOP_PATH = ".donutloop.aln"
local QPU_DATASHARD_PATH = ".qpu_datashard/"

-- Monotonic evolution constraints (CRITICAL - cannot be relaxed)
local KF_MIN_DELTA = 0.0  -- Knowledge Factor cannot decrease
local ROH_MAX_DELTA = 0.0  -- Risk of Harm cannot increase

-- Evolution corridor thresholds
local KF_CORRIDOR_MIN = 0.0
local KF_CORRIDOR_MAX = 1.0
local ROH_CORRIDOR_MIN = 0.0
local ROH_CORRIDOR_MAX = 1.0

-- Evolution scope multipliers (stricter scopes have tighter corridors)
local SCOPE_MULTIPLIERS = {
    standard = 1.0,
    orchestrator = 0.5,
    core_safety = 0.25,
    firmware = 0.3,
    ota_evolution = 0.4
}

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

local DISCRIMINATION_RISK_THRESHOLD = 0.3

-- Evolution history retention (for non-reversibility verification)
local EVOLUTION_HISTORY_MAX_SIZE = 10000
local EVOLUTION_HISTORY_RETENTION_DAYS = 365

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
    self.justification_sko = data.justification_sko or nil
    self.parent_evolution_id = data.parent_evolution_id or nil
    return self
end

function EvolutionProposal:compute_hash()
    local data = {
        id = self.id,
        identifier = self.identifier,
        signer_did = self.signer_did,
        timestamp = self.timestamp,
        commit_hash = self.commit_hash,
        kf_before = self.kf_before,
        kf_after = self.kf_after,
        roh_before = self.roh_before,
        roh_after = self.roh_after
    }
    return crypto.sha256_hex(json.encode(data))
end

function EvolutionProposal:get_kf_delta()
    return self.kf_after - self.kf_before
end

function EvolutionProposal:get_roh_delta()
    return self.roh_after - self.roh_before
end

function EvolutionProposal:is_within_corridors()
    local scope_mult = SCOPE_MULTIPLIERS[self.scope] or 1.0
    
    local kf_min = KF_CORRIDOR_MIN
    local kf_max = KF_CORRIDOR_MAX * scope_mult
    local roh_min = ROH_CORRIDOR_MIN
    local roh_max = ROH_CORRIDOR_MAX * scope_mult
    
    return (self.kf_before >= kf_min and self.kf_before <= kf_max) and
           (self.kf_after >= kf_min and self.kf_after <= kf_max) and
           (self.roh_before >= roh_min and self.roh_before <= roh_max) and
           (self.roh_after >= roh_min and self.roh_after <= roh_max)
end

function EvolutionProposal:validate_monotonicity()
    local kf_delta = self:get_kf_delta()
    local roh_delta = self:get_roh_delta()
    
    -- Knowledge Factor must not decrease
    if kf_delta < KF_MIN_DELTA then
        return false, "KF_DECREASE", string.format(
            "Knowledge Factor cannot decrease (before: %.4f, after: %.4f, delta: %.4f)",
            self.kf_before, self.kf_after, kf_delta
        )
    end
    
    -- Risk of Harm must not increase
    if roh_delta > ROH_MAX_DELTA then
        return false, "ROH_INCREASE", string.format(
            "Risk of Harm cannot increase (before: %.4f, after: %.4f, delta: %.4f)",
            self.roh_before, self.roh_after, roh_delta
        )
    end
    
    return true, "MONOTONICITY_OK", string.format(
        "KF delta: %.4f, RoH delta: %.4f", kf_delta, roh_delta
    )
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

--- Evolution history entry (for non-reversibility verification)
local EvolutionHistoryEntry = {}
EvolutionHistoryEntry.__index = EvolutionHistoryEntry

function EvolutionHistoryEntry.new(proposal, validation_result)
    local self = setmetatable({}, EvolutionHistoryEntry)
    self.id = proposal.id
    self.identifier = proposal.identifier
    self.timestamp = proposal.timestamp
    self.signer_did = proposal.signer_did
    self.scope = proposal.scope
    self.kf_before = proposal.kf_before
    self.kf_after = proposal.kf_after
    self.roh_before = proposal.roh_before
    self.roh_after = proposal.roh_after
    self.kf_delta = proposal:get_kf_delta()
    self.roh_delta = proposal:get_roh_delta()
    self.validation_result = validation_result
    self.hash = proposal:compute_hash()
    self.parent_id = proposal.parent_evolution_id
    return self
end

function EvolutionHistoryEntry:to_dict()
    return {
        id = self.id,
        identifier = self.identifier,
        timestamp = self.timestamp,
        signer_did = self.signer_did,
        scope = self.scope,
        kf_before = self.kf_before,
        kf_after = self.kf_after,
        roh_before = self.roh_before,
        roh_after = self.roh_after,
        kf_delta = self.kf_delta,
        roh_delta = self.roh_delta,
        validation_result = self.validation_result,
        hash = self.hash,
        parent_id = self.parent_id
    }
end

--- Evolution validation result
local EvolutionValidationResult = {}
EvolutionValidationResult.__index = EvolutionValidationResult

function EvolutionValidationResult.new()
    local self = setmetatable({}, EvolutionValidationResult)
    self.valid = false
    self.monotonicity_ok = false
    self.corridors_ok = false
    self.non_reversible_ok = false
    self.discrimination_ok = false
    self.justification_ok = false
    self.error_code = nil
    self.error_message = nil
    self.warnings = {}
    self.evidence_hash = nil
    return self
end

function EvolutionValidationResult:set_valid()
    self.valid = true
    self.monotonicity_ok = true
    self.corridors_ok = true
    self.non_reversible_ok = true
    self.discrimination_ok = true
    self.justification_ok = true
    return self
end

function EvolutionValidationResult:set_invalid(error_code, error_message)
    self.valid = false
    self.error_code = error_code
    self.error_message = error_message
    return self
end

function EvolutionValidationResult:add_warning(warning)
    table.insert(self.warnings, warning)
    return self
end

function EvolutionValidationResult:to_dict()
    return {
        valid = self.valid,
        monotonicity_ok = self.monotonicity_ok,
        corridors_ok = self.corridors_ok,
        non_reversible_ok = self.non_reversible_ok,
        discrimination_ok = self.discrimination_ok,
        justification_ok = self.justification_ok,
        error_code = self.error_code,
        error_message = self.error_message,
        warnings = self.warnings,
        evidence_hash = self.evidence_hash
    }
end

-- =============================================================================
-- EVOLUTION GUARD CLASS
-- =============================================================================

local EvolutionGuard = {}
EvolutionGuard.__index = EvolutionGuard

function EvolutionGuard.new(config)
    local self = setmetatable({}, EvolutionGuard)
    
    self.config = config or {}
    self.host_did = config.host_did or crypto.generate_did()
    self.host_keypair = config.host_keypair or crypto.generate_keypair()
    self.evolution_history = {}
    self.evolution_history_by_id = {}
    self.event_id_counter = self:load_event_id_counter()
    self.qpu_datashard = datashard.new("EVOLUTIONEVENT")
    self.googolswarm_client = googolswarm.new()
    self.aln_shard_path = config.aln_shard_path or "Config/Sovereignty/policies/evolution-corridors-v1.aln"
    self.corridors = self:load_corridors()
    
    return self
end

function EvolutionGuard:load_event_id_counter()
    local last_event = self.qpu_datashard:get_last_event()
    if last_event and last_event.event_id then
        return last_event.event_id
    end
    return 0
end

function EvolutionGuard:load_corridors()
    local success, shard_data = pcall(function()
        return aln.load_shard(self.aln_shard_path)
    end)
    
    if not success then
        -- Use default corridors if ALN shard not found
        return {
            kf_min = KF_CORRIDOR_MIN,
            kf_max = KF_CORRIDOR_MAX,
            roh_min = ROH_CORRIDOR_MIN,
            roh_max = ROH_CORRIDOR_MAX,
            scope_multipliers = SCOPE_MULTIPLIERS
        }
    end
    
    local corridors = {
        kf_min = KF_CORRIDOR_MIN,
        kf_max = KF_CORRIDOR_MAX,
        roh_min = ROH_CORRIDOR_MIN,
        roh_max = ROH_CORRIDOR_MAX,
        scope_multipliers = SCOPE_MULTIPLIERS
    }
    
    for _, entry in ipairs(shard_data.entries or {}) do
        if entry.metric == "evolution_corridor" then
            if entry.param == "kf_min" then
                corridors.kf_min = tonumber(entry.value) or KF_CORRIDOR_MIN
            elseif entry.param == "kf_max" then
                corridors.kf_max = tonumber(entry.value) or KF_CORRIDOR_MAX
            elseif entry.param == "roh_min" then
                corridors.roh_min = tonumber(entry.value) or ROH_CORRIDOR_MIN
            elseif entry.param == "roh_max" then
                corridors.roh_max = tonumber(entry.value) or ROH_CORRIDOR_MAX
            end
        elseif entry.metric == "scope_multiplier" then
            corridors.scope_multipliers[entry.scope] = tonumber(entry.multiplier) or 1.0
        end
    end
    
    return corridors
end

function EvolutionGuard:validate_proposal(proposal)
    local result = EvolutionValidationResult.new()
    
    -- Step 1: Validate monotonicity (CRITICAL)
    local mono_ok, mono_code, mono_msg = proposal:validate_monotonicity()
    result.monotonicity_ok = mono_ok
    
    if not mono_ok then
        return result:set_invalid(mono_code, mono_msg)
    end
    
    -- Step 2: Validate corridors
    local corridors_ok = proposal:is_within_corridors()
    result.corridors_ok = corridors_ok
    
    if not corridors_ok then
        return result:set_invalid("CORRIDOR_VIOLATION", 
            "Evolution exceeds KF/RoH corridor bounds for scope: " .. proposal.scope)
    end
    
    -- Step 3: Verify non-reversibility
    local non_rev_ok, non_rev_msg = self:verify_non_reversibility(proposal)
    result.non_reversible_ok = non_rev_ok
    
    if not non_rev_ok then
        return result:set_invalid("REVERSAL_DETECTED", non_rev_msg)
    end
    
    -- Step 4: Check discrimination risk (CRITICAL)
    local disc_risk, disc_violations = proposal:check_discrimination_risk()
    result.discrimination_ok = (disc_risk <= DISCRIMINATION_RISK_THRESHOLD)
    
    if not result.discrimination_ok then
        local violation_str = table.concat(disc_violations, ", ")
        return result:set_invalid("DISCRIMINATION_DETECTED", 
            "Discrimination risk " .. tostring(disc_risk) .. " exceeds threshold. Violations: " .. violation_str)
    end
    
    -- Step 5: Validate justification SKO (if required)
    if proposal.scope == "core_safety" or proposal.scope == "orchestrator" then
        if not proposal.justification_sko then
            result.justification_ok = false
            result:add_warning("Missing justification SKO for high-scope evolution")
        else
            result.justification_ok = true
        end
    else
        result.justification_ok = true
    end
    
    -- All checks passed
    result:set_valid()
    result.evidence_hash = proposal:compute_hash()
    
    return result
end

function EvolutionGuard:verify_non_reversibility(proposal)
    -- Check against all previous evolution entries
    for _, entry in ipairs(self.evolution_history) do
        -- KF cannot decrease from any previous state
        if proposal.kf_after < entry.kf_after - KF_MIN_DELTA then
            return false, string.format(
                "REVERSAL_DETECTED: KF would decrease from evolution %s (current: %.4f, previous: %.4f)",
                entry.id, proposal.kf_after, entry.kf_after
            )
        end
        
        -- RoH cannot increase from any previous state
        if proposal.roh_after > entry.roh_after + ROH_MAX_DELTA then
            return false, string.format(
                "REVERSAL_DETECTED: RoH would increase from evolution %s (current: %.4f, previous: %.4f)",
                entry.id, proposal.roh_after, entry.roh_after
            )
        end
    end
    
    -- Check parent evolution chain if applicable
    if proposal.parent_evolution_id then
        local parent = self.evolution_history_by_id[proposal.parent_evolution_id]
        if parent then
            if proposal.kf_before ~= parent.kf_after then
                return false, string.format(
                    "CHAIN_BREAK: KF before (%.4f) does not match parent KF after (%.4f)",
                    proposal.kf_before, parent.kf_after
                )
            end
            if proposal.roh_before ~= parent.roh_after then
                return false, string.format(
                    "CHAIN_BREAK: RoH before (%.4f) does not match parent RoH after (%.4f)",
                    proposal.roh_before, parent.roh_after
                )
            end
        end
    end
    
    return true, "NON_REVERSIBLE_OK"
end

function EvolutionGuard:record_evolution(proposal, validation_result)
    local entry = EvolutionHistoryEntry.new(proposal, validation_result)
    
    -- Add to history
    table.insert(self.evolution_history, entry)
    self.evolution_history_by_id[entry.id] = entry
    
    -- Trim history if too large
    while #self.evolution_history > EVOLUTION_HISTORY_MAX_SIZE do
        local removed = table.remove(self.evolution_history, 1)
        if removed then
            self.evolution_history_by_id[removed.id] = nil
        end
    end
    
    -- Log to QPU.Datashard
    self:log_evolution_event(entry, validation_result)
    
    return entry
end

function EvolutionGuard:log_evolution_event(entry, validation_result)
    self.event_id_counter = self.event_id_counter + 1
    
    local event = {
        event_id = self.event_id_counter,
        event_type = "EVOLUTIONEVENT",
        timestamp_utc = chrono.utcnow(),
        brainidentity_hash = brainidentity.get_hash(),
        evolution_id = entry.id,
        evolution_identifier = entry.identifier,
        signer_did = entry.signer_did,
        scope = entry.scope,
        kf_before = entry.kf_before,
        kf_after = entry.kf_after,
        kf_delta = entry.kf_delta,
        roh_before = entry.roh_before,
        roh_after = entry.roh_after,
        roh_delta = entry.roh_delta,
        validation_result = validation_result:to_dict(),
        evidence_hash = entry.hash,
        host_did_signature = "",
        googolswarm_anchor_txid = nil
    }
    
    -- Sign event
    local payload = json.encode({
        event_id = event.event_id,
        evolution_id = event.evolution_id,
        evidence_hash = event.evidence_hash
    })
    event.host_did_signature = crypto.sign_ed25519(self.host_keypair, payload)
    
    -- Write to QPU.Datashard
    self.qpu_datashard:append("EVOLUTIONEVENT", event)
    
    -- Anchor to Googolswarm
    local anchor_txid = self.googolswarm_client:anchor_event(event)
    event.googolswarm_anchor_txid = anchor_txid
    
    -- Update event record with anchor
    self.qpu_datashard:update_anchor(event.event_id, anchor_txid)
    
    -- Log to DonutLoop
    self:log_to_donutloop(event, validation_result)
    
    return event
end

function EvolutionGuard:log_to_donutloop(event, validation_result)
    local log_entry = {
        event_type = "EVOLUTION_GUARD_EVENT",
        event_id = event.event_id,
        evolution_id = event.evolution_id,
        timestamp = event.timestamp_utc,
        valid = event.validation_result.valid,
        error_code = event.validation_result.error_code,
        error_message = event.validation_result.error_message,
        kf_delta = event.kf_delta,
        roh_delta = event.roh_delta,
        googolswarm_anchor = event.googolswarm_anchor_txid
    }
    
    datashard.append_log(DONUTLOOP_PATH, log_entry)
end

function EvolutionGuard:approve_evolution(proposal)
    local validation_result = self:validate_proposal(proposal)
    
    if not validation_result.valid then
        -- Log rejection event
        local entry = EvolutionHistoryEntry.new(proposal, validation_result)
        self:record_evolution(proposal, validation_result)
        
        return false, validation_result
    end
    
    -- Record approved evolution
    local entry = self:record_evolution(proposal, validation_result)
    
    return true, validation_result, entry
end

function EvolutionGuard:get_evolution_chain(evolution_id)
    local chain = {}
    local current_id = evolution_id
    
    while current_id do
        local entry = self.evolution_history_by_id[current_id]
        if not entry then
            break
        end
        
        table.insert(chain, 1, entry:to_dict())
        current_id = entry.parent_id
    end
    
    return chain
end

function EvolutionGuard:get_evolution_history(start_id, end_id)
    start_id = start_id or 1
    end_id = end_id or #self.evolution_history
    
    local history = {}
    for i = start_id, math.min(end_id, #self.evolution_history) do
        table.insert(history, self.evolution_history[i]:to_dict())
    end
    
    return history
end

function EvolutionGuard:verify_evolution_integrity(evolution_id)
    local entry = self.evolution_history_by_id[evolution_id]
    if not entry then
        return false, "EVOLUTION_NOT_FOUND"
    end
    
    -- Recompute hash and verify
    local proposal = EvolutionProposal.new({
        id = entry.id,
        identifier = entry.identifier,
        signer_did = entry.signer_did,
        timestamp = entry.timestamp,
        scope = entry.scope,
        kf_before = entry.kf_before,
        kf_after = entry.kf_after,
        roh_before = entry.roh_before,
        roh_after = entry.roh_after
    })
    
    local computed_hash = proposal:compute_hash()
    if computed_hash ~= entry.hash then
        return false, "HASH_MISMATCH"
    end
    
    return true, "INTEGRITY_OK"
end

function EvolutionGuard:export_audit_trail(start_date, end_date)
    local audit_trail = {
        export_timestamp = chrono.utcnow(),
        host_did = self.host_did,
        brainidentity_hash = brainidentity.get_hash(),
        evolution_events = {},
        total_events = 0,
        valid_events = 0,
        rejected_events = 0
    }
    
    for _, entry in ipairs(self.evolution_history) do
        local event_date = entry.timestamp:sub(1, 10)
        
        if (not start_date or event_date >= start_date) and
           (not end_date or event_date <= end_date) then
            table.insert(audit_trail.evolution_events, entry:to_dict())
            audit_trail.total_events = audit_trail.total_events + 1
            
            if entry.validation_result == "MONOTONICITY_OK" or
               entry.validation_result.valid then
                audit_trail.valid_events = audit_trail.valid_events + 1
            else
                audit_trail.rejected_events = audit_trail.rejected_events + 1
            end
        end
    end
    
    -- Sign audit trail
    local payload = json.encode({
        export_timestamp = audit_trail.export_timestamp,
        total_events = audit_trail.total_events,
        valid_events = audit_trail.valid_events,
        rejected_events = audit_trail.rejected_events
    })
    audit_trail.host_did_signature = crypto.sign_ed25519(self.host_keypair, payload)
    
    -- Anchor to Googolswarm
    audit_trail.googolswarm_anchor_txid = self.googolswarm_client:anchor_event(audit_trail)
    
    return audit_trail
end

-- =============================================================================
-- ANTI-DISCRIMINATION GUARD
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
    for attr, changed in pairs(proposal.protected_attribute_changes or {}) do
        if changed then
            table.insert(violations, {
                type = "PROTECTED_ATTRIBUTE_MODIFICATION",
                attribute = attr
            })
        end
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

function AntiDiscriminationGuard:export_violation_report()
    local report = {
        export_timestamp = chrono.utcnow(),
        total_violations = #self.violation_log,
        violations = self.violation_log
    }
    
    -- Sign report
    local payload = json.encode({
        export_timestamp = report.export_timestamp,
        total_violations = report.total_violations
    })
    report.host_did_signature = crypto.sign_ed25519(
        crypto.generate_keypair(), 
        payload
    )
    
    return report
end

-- =============================================================================
-- MONOTONIC EVOLUTION GUARD
-- =============================================================================

local MonotonicEvolutionGuard = {}
MonotonicEvolutionGuard.__index = MonotonicEvolutionGuard

function MonotonicEvolutionGuard.new()
    local self = setmetatable({}, MonotonicEvolutionGuard)
    self.evolution_history = {}
    self.kf_floor = 0.0
    self.roh_ceiling = 1.0
    return self
end

function MonotonicEvolutionGuard:update_bounds(proposal)
    -- Update KF floor (can only increase or stay same)
    if proposal.kf_after > self.kf_floor then
        self.kf_floor = proposal.kf_after
    end
    
    -- Update RoH ceiling (can only decrease or stay same)
    if proposal.roh_after < self.roh_ceiling then
        self.roh_ceiling = proposal.roh_after
    end
end

function MonotonicEvolutionGuard:verify_proposal(proposal)
    -- Check against KF floor
    if proposal.kf_after < self.kf_floor - KF_MIN_DELTA then
        return false, string.format(
            "KF_FLOOR_VIOLATION: KF after (%.4f) below floor (%.4f)",
            proposal.kf_after, self.kf_floor
        )
    end
    
    -- Check against RoH ceiling
    if proposal.roh_after > self.roh_ceiling + ROH_MAX_DELTA then
        return false, string.format(
            "ROH_CEILING_VIOLATION: RoH after (%.4f) above ceiling (%.4f)",
            proposal.roh_after, self.roh_ceiling
        )
    end
    
    return true, "MONOTONIC_BOUNDS_OK"
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
    
    self:update_bounds(proposal)
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

function MonotonicEvolutionGuard:get_current_bounds()
    return {
        kf_floor = self.kf_floor,
        roh_ceiling = self.roh_ceiling
    }
end

function MonotonicEvolutionGuard:export_bounds_audit()
    local audit = {
        export_timestamp = chrono.utcnow(),
        kf_floor = self.kf_floor,
        roh_ceiling = self.roh_ceiling,
        evolution_count = #self.evolution_history,
        evolution_history = self.evolution_history
    }
    
    return audit
end

-- =============================================================================
-- EXPORTED API FUNCTIONS
-- =============================================================================

local _M = {}

function _M.create_evolution_guard(config)
    return EvolutionGuard.new(config)
end

function _M.create_anti_discrimination_guard()
    return AntiDiscriminationGuard.new()
end

function _M.create_monotonic_guard()
    return MonotonicEvolutionGuard.new()
end

function _M.create_evolution_proposal(data)
    return EvolutionProposal.new(data)
end

function _M.create_validation_result()
    return EvolutionValidationResult.new()
end

function _M.validate_evolution_proposal(guard, proposal)
    return guard:validate_proposal(proposal)
end

function _M.approve_evolution(guard, proposal)
    return guard:approve_evolution(proposal)
end

function _M.check_monotonicity(proposal)
    return proposal:validate_monotonicity()
end

function _M.check_discrimination_risk(proposal)
    return proposal:check_discrimination_risk()
end

function _M.verify_non_reversibility(guard, proposal)
    return guard:verify_non_reversibility(proposal)
end

function _M.export_audit_trail(guard, start_date, end_date)
    return guard:export_audit_trail(start_date, end_date)
end

return _M

-- =============================================================================
-- END OF FILE
-- =============================================================================
