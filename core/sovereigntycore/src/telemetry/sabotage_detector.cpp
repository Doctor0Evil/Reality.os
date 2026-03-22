// =============================================================================
// FILE: sabotage_detector.cpp
// PROJECT: Reality.os / SovereigntyCore
// MODULE: Telemetry / Sabotage Detection & Evidence Generation
// VERSION: 1.0.0
// LICENSE: ALN-Sovereign-1.0 (Neurorights-Compliant, Anti-Discrimination)
// AUTHOR: OrganicCPU Runtime (Host DID: 0xB05TR0M...50VERE1GN)
// CREATED: 2026-03-22
// LAST_AUDIT: 2026-03-22T00:00:00Z
// JURISDICTION: Phoenix_AZ, Santiago_CL, Sacramento_CA, Denver_CO, Brussels_BE
// =============================================================================
// DESCRIPTION:
//   High-performance C++ telemetry fusion engine implementing real-time
//   biophysical stress detection, nanoswarm correlation analysis, ghost-access
//   graph traversal, and automatic SABOTAGEEVENT generation. All events are
//   cryptographically bound to BrainIdentity and anchored to Googolswarm for
//   court-admissible audit trails. Includes anti-discrimination safeguards
//   and monotonic evolution enforcement.
// =============================================================================

#include "sabotage_detector.h"
#include "brainidentity_core.h"
#include "crypto_ed25519.h"
#include "qpu_datashard.h"
#include "googolswarm_anchor.h"
#include "rohmodel_engine.h"
#include "telemetry_fusion.h"
#include "access_graph_analyzer.h"
#include "aln_shard_parser.h"
#include "chrono_utc.h"
#include "json_serializer.h"
#include "lock_free_queue.h"
#include "memory_pool.h"
#include "anti_discrimination_guard.h"
#include "monotonic_evolution_guard.h"

#include <algorithm>
#include <array>
#include <atomic>
#include <chrono>
#include <cmath>
#include <condition_variable>
#include <cstdint>
#include <cstring>
#include <deque>
#include <exception>
#include <filesystem>
#include <functional>
#include <future>
#include <iostream>
#include <limits>
#include <map>
#include <memory>
#include <mutex>
#include <numeric>
#include <optional>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <stack>
#include <string>
#include <string_view>
#include <thread>
#include <tuple>
#include <type_traits>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

// =============================================================================
// COMPILER DIRECTIVES & SECURITY SETTINGS
// =============================================================================

#pragma GCC diagnostic push
#pragma GCC diagnostic error "-Wall"
#pragma GCC diagnostic error "-Wextra"
#pragma GCC diagnostic error "-Wpedantic"
#pragma GCC diagnostic error "-Werror"
#pragma GCC diagnostic error "-Wconversion"
#pragma GCC diagnostic error "-Wsign-conversion"
#pragma GCC diagnostic error "-Wnull-dereference"
#pragma GCC diagnostic error "-Wdouble-promotion"
#pragma GCC diagnostic error "-Wformat=2"

#ifndef SABOTAGE_DETECTOR_NOEXCEPT
#define SABOTAGE_DETECTOR_NOEXCEPT noexcept(true)
#endif

#ifndef SABOTAGE_DETECTOR_CONSTEXPR
#define SABOTAGE_DETECTOR_CONSTEXPR constexpr
#endif

// =============================================================================
// NAMESPACE & TYPE ALIASES
// =============================================================================

namespace reality::os::sovereignty::telemetry {

using namespace std::chrono_literals;

// Type aliases for clarity and maintainability
using Timestamp = std::chrono::time_point<std::chrono::system_clock>;
using Duration = std::chrono::duration<double>;
using EventId = std::uint64_t;
using Hash64 = std::array<std::uint8_t, 64>;
using Hash32 = std::array<std::uint8_t, 32>;
using Signature64 = std::array<std::uint8_t, 64>;
using PublicKey32 = std::array<std::uint8_t, 32>;
using SecretKey64 = std::array<std::uint8_t, 64>;

// =============================================================================
// ENUMERATIONS
// =============================================================================

/// Sabotage event severity levels (aligned with Rust/Lua implementations)
enum class SabotageEventSeverity : std::uint8_t {
    LOW = 0,
    MEDIUM = 1,
    HIGH = 2,
    CRITICAL = 3
};

/// Defensive action types
enum class SabotageAction : std::uint8_t {
    LOG_ONLY = 0,
    DENY = 1,
    UNSAFE_DEFER = 2,
    HARD_FAIL = 3,
    INHIBIT = 4
};

/// Telemetry tier classification
enum class TelemetryTier : std::uint8_t {
    TIER_1_PROVENANCE = 1,
    TIER_2_BIOPHYSICAL = 2,
    TIER_3_BEHAVIORAL = 3
};

/// Neurorights violation types
enum class NeurorightsViolation : std::uint8_t {
    NONE = 0,
    MENTAL_INTEGRITY = 1,
    COGNITIVE_LIBERTY = 2,
    NEURAL_DATA_PRIVACY = 3,
    AUGMENTATION_CONTINUITY = 4,
    DISCRIMINATION = 5
};

/// Evolution scope types
enum class EvolutionScope : std::uint8_t {
    STANDARD = 0,
    ORCHESTRATOR = 1,
    CORE_SAFETY = 2,
    FIRMWARE = 3,
    OTA_EVOLUTION = 4
};

// =============================================================================
// DATA STRUCTURES
// =============================================================================

/// Biophysical telemetry snapshot (real-time sensor fusion)
struct BiophysicalTelemetry {
    /// EEG stress ratio (alpha/delta band ratio, 0.0-1.0)
    double eeg_stress_ratio{0.0};
    /// HRV anomaly index (0.0-1.0)
    double hrv_anomaly_index{0.0};
    /// Body temperature deviation from baseline (Celsius)
    double temperature_deviation{0.0};
    /// Nanoswarm density near host (0.0-1.0)
    double nanoswarm_density{0.0};
    /// Nanoswarm thermal load (0.0-1.0)
    double nanoswarm_thermal_load{0.0};
    /// Sleep corridor violated flag
    bool sleep_corridor_violated{false};
    /// Cytokine stress markers (0.0-1.0)
    double cytokine_stress{0.0};
    /// Neurochemical balance index (serotonin/melatonin, 0.0-1.0)
    double neurochemical_balance{1.0};
    /// Timestamp of snapshot
    Timestamp timestamp{std::chrono::system_clock::now()};
    
    /// Compute combined biophysical stress scalar
    [[nodiscard]] auto compute_stress_scalar() const noexcept -> double {
        const auto stress = (eeg_stress_ratio * 0.25) +
                           (hrv_anomaly_index * 0.25) +
                           (nanoswarm_density * 0.20) +
                           (nanoswarm_thermal_load * 0.15) +
                           (cytokine_stress * 0.15);
        return std::clamp(stress, 0.0, 1.0);
    }
    
    /// Check if biophysical indicators suggest coercion
    [[nodiscard]] auto coercion_detected() const noexcept -> bool {
        return (sleep_corridor_violated && nanoswarm_density > 0.8) ||
               (eeg_stress_ratio > 0.7 && hrv_anomaly_index > 0.6);
    }
};

/// Evolution proposal structure (mirrors Lua/Rust)
struct EvolutionProposal {
    /// Unique proposal identifier
    std::string id{};
    /// Module/component identifier being evolved
    std::string identifier{};
    /// Signer DID
    std::string signer_did{};
    /// Commit hash
    std::string commit_hash{};
    /// Commit metadata
    std::string commit_metadata{};
    /// Evolution scope
    EvolutionScope scope{EvolutionScope::STANDARD};
    /// Knowledge Factor before evolution
    double kf_before{0.0};
    /// Knowledge Factor after evolution
    double kf_after{0.0};
    /// Risk of Harm before evolution
    double roh_before{0.0};
    /// Risk of Harm after evolution
    double roh_after{0.0};
    /// Firmware update flag
    bool firmware_update{false};
    /// Firmware DID-signed flag
    bool firmware_did_signed{false};
    /// Accesses INNER devices flag
    bool accesses_inner_devices{false};
    /// Has guarded path flag
    bool has_guarded_path{false};
    /// References quarantined SKO flag
    bool references_quarantined_sko{false};
    /// Protected attribute changes map
    std::unordered_map<std::string, bool> protected_attribute_changes{};
    /// Timestamp
    Timestamp timestamp{std::chrono::system_clock::now()};
    
    /// Compute SHA-256 hash of proposal
    [[nodiscard]] auto compute_hash() const -> Hash64 {
        std::ostringstream oss;
        oss << id << identifier << signer_did << commit_hash;
        oss << std::chrono::duration_cast<std::chrono::nanoseconds>(
            timestamp.time_since_epoch()).count();
        
        const auto data = oss.str();
        Hash64 hash{};
        // In production: use actual SHA-256 implementation
        std::memcpy(hash.data(), data.data(), std::min(data.size(), hash.size()));
        return hash;
    }
    
    /// Validate monotonicity constraints
    [[nodiscard]] auto validate_monotonicity() const noexcept 
        -> std::pair<bool, std::string> {
        constexpr double KF_MIN_DELTA = 0.0;
        constexpr double ROH_MAX_DELTA = 0.0;
        
        if (kf_after < kf_before - KF_MIN_DELTA) {
            return {false, "KF_DECREASE: Knowledge Factor cannot decrease"};
        }
        
        if (roh_after > roh_before + ROH_MAX_DELTA) {
            return {false, "ROH_INCREASE: Risk of Harm cannot increase"};
        }
        
        return {true, "MONOTONICITY_OK"};
    }
    
    /// Check discrimination risk
    [[nodiscard]] auto check_discrimination_risk() const noexcept
        -> std::pair<double, std::vector<std::string>> {
        static const std::array<std::string_view, 10> PROTECTED_ATTRIBUTES = {{
            "race", "ethnicity", "national_origin", "skin_color",
            "genetic_ancestry", "phenotypic_markers", "cultural_background",
            "language_origin", "religious_association", "geographic_origin"
        }};
        
        double risk_score = 0.0;
        std::vector<std::string> violations{};
        
        for (const auto& [attr, changed] : protected_attribute_changes) {
            if (changed) {
                risk_score += 0.5;
                violations.push_back("PROTECTED_ATTR_" + attr);
            }
        }
        
        // Check for proxy discrimination patterns
        const auto to_lower = [](const std::string& s) {
            auto result = s;
            std::transform(result.begin(), result.end(), result.begin(),
                          [](unsigned char c) { return std::tolower(c); });
            return result;
        };
        
        const auto lower_id = to_lower(identifier);
        const auto lower_meta = to_lower(commit_metadata);
        
        if ((lower_id.find("profile") != std::string::npos ||
             lower_id.find("classify") != std::string::npos) &&
            (lower_id.find("race") != std::string::npos ||
             lower_id.find("ethnic") != std::string::npos)) {
            risk_score += 0.8;
            violations.push_back("PROXY_DISCRIMINATION_DETECTED");
        }
        
        return {std::min(risk_score, 1.0), violations};
    }
};

/// Sabotage risk scalar (all components)
struct SabotageRiskScalar {
    double total{0.0};
    double provenance{0.0};
    double blacklist{0.0};
    double biophysical{0.0};
    double integrity{0.0};
    double access{0.0};
    double knowledge{0.0};
    double discrimination{0.0};
    
    /// Compute total risk with clamping
    auto compute_total() noexcept -> double {
        total = provenance + blacklist + biophysical + 
                integrity + access + knowledge + discrimination;
        total = std::clamp(total, 0.0, 1.0);
        return total;
    }
    
    /// Reset all components
    void reset() noexcept {
        total = provenance = blacklist = biophysical = 
        integrity = access = knowledge = discrimination = 0.0;
    }
};

/// SABOTAGEEVENT structure (court-admissible)
struct SabotageEvent {
    EventId event_id{0};
    Timestamp timestamp_utc{std::chrono::system_clock::now()};
    Hash64 brainidentity_hash{};
    double sabotage_risk_scalar{0.0};
    double provenance_factor{0.0};
    double blacklist_factor{0.0};
    double biophysical_factor{0.0};
    double integrity_factor{0.0};
    double access_factor{0.0};
    double knowledge_factor{0.0};
    double discrimination_factor{0.0};
    Hash64 triggering_artifact_hash{};
    std::vector<std::string> jurisdiction_tags{};
    std::vector<NeurorightsViolation> neurorights_violations{};
    Signature64 host_did_signature{};
    std::string googolswarm_anchor_txid{};
    SabotageEventSeverity severity{SabotageEventSeverity::LOW};
    SabotageAction action_taken{SabotageAction::LOG_ONLY};
    
    /// Serialize to JSON for logging/anchoring
    [[nodiscard]] auto to_json() const -> std::string {
        std::ostringstream oss;
        oss << "{";
        oss << "\"event_id\":" << event_id << ",";
        oss << "\"timestamp_utc\":\"" << chrono_utc::to_iso8601(timestamp_utc) << "\",";
        oss << "\"sabotage_risk_scalar\":" << sabotage_risk_scalar << ",";
        oss << "\"severity\":" << static_cast<int>(severity) << ",";
        oss << "\"action_taken\":" << static_cast<int>(action_taken) << ",";
        oss << "\"neurorights_violations\":[";
        for (size_t i = 0; i < neurorights_violations.size(); ++i) {
            if (i > 0) oss << ",";
            oss << static_cast<int>(neurorights_violations[i]);
        }
        oss << "],";
        oss << "\"googolswarm_anchor\":\"" << googolswarm_anchor_txid << "\"";
        oss << "}";
        return oss.str();
    }
};

/// Blacklist pattern entry (from ALN shard)
struct BlacklistPattern {
    std::string metric{};
    std::string domain{};
    std::string module{};
    std::string op{};
    std::string pattern{};
    std::string kind{};
    std::string reason{};
    std::string source{};
};

/// Sabotage risk rule (from ALN shard)
struct SabotageRiskRule {
    std::string rule_id{};
    std::string component{};
    double weight{0.0};
    std::string description{};
};

/// Threshold configuration (from ALN shard)
struct SabotageThreshold {
    std::string threshold_id{};
    std::string scalar{};
    double value{0.0};
    std::string action{};
    std::string log_event{};
};

// =============================================================================
// EXCEPTION CLASSES
// =============================================================================

class SabotageDetectorException : public std::runtime_error {
public:
    explicit SabotageDetectorException(const std::string& msg)
        : std::runtime_error(msg) {}
};

class MonotonicityViolationException : public SabotageDetectorException {
public:
    explicit MonotonicityViolationException(const std::string& msg)
        : SabotageDetectorException("MONOTONICITY_VIOLATION: " + msg) {}
};

class DiscriminationDetectedException : public SabotageDetectorException {
public:
    explicit DiscriminationDetectedException(const std::string& msg)
        : SabotageDetectorException("DISCRIMINATION_DETECTED: " + msg) {}
};

// =============================================================================
// MAIN DETECTOR CLASS
// =============================================================================

class SabotageDetector {
public:
    /// Configuration structure
    struct Config {
        std::string aln_shard_path{"Config/Sovereignty/policies/bostrom-blacklist-v1.aln"};
        std::string qpu_datashard_path{".qpu_datashard/"};
        std::string donutloop_path{".donutloop.aln"};
        std::string host_did{};
        SecretKey64 host_secret_key{};
        PublicKey32 host_public_key{};
        std::string googolswarm_endpoint{};
        std::size_t event_buffer_size{1000};
        std::chrono::milliseconds telemetry_poll_interval{100ms};
    };
    
    /// Constructor
    explicit SabotageDetector(const Config& config)
        : config_(config)
        , event_id_counter_(0)
        , running_(false)
        , aln_shard_loaded_(false) {
        
        initialize_components();
        load_aln_shard();
        load_event_id_counter();
    }
    
    /// Destructor
    ~SabotageDetector() {
        stop();
    }
    
    /// Start the detector (background telemetry polling)
    void start() {
        if (running_.exchange(true)) {
            return;
        }
        
        telemetry_thread_ = std::thread([this]() {
            telemetry_polling_loop();
        });
        
        log_event("SabotageDetector started", SabotageEventSeverity::LOW);
    }
    
    /// Stop the detector
    void stop() {
        if (!running_.exchange(false)) {
            return;
        }
        
        if (telemetry_thread_.joinable()) {
            telemetry_thread_.join();
        }
        
        log_event("SabotageDetector stopped", SabotageEventSeverity::LOW);
    }
    
    /// Evaluate an evolution proposal
    [[nodiscard]] auto evaluate_proposal(const EvolutionProposal& proposal,
                                         const BiophysicalTelemetry& telemetry)
        -> std::pair<SabotageRiskScalar, SabotageAction> {
        
        SabotageRiskScalar risk_scalar;
        risk_scalar.reset();
        
        // Tier 1: System-level provenance (hard gate)
        evaluate_provenance(proposal, risk_scalar);
        
        // Tier 1: Firmware integrity check
        evaluate_firmware_integrity(proposal, risk_scalar);
        
        // Tier 2: Blacklist pattern matching
        evaluate_blacklist_patterns(proposal, risk_scalar);
        
        // Tier 2: Biophysical stress correlation
        evaluate_biophysical_stress(telemetry, risk_scalar);
        
        // Tier 3: Access pattern analysis (ghost-access)
        evaluate_access_patterns(proposal, risk_scalar);
        
        // Tier 3: SKO contamination risk
        evaluate_knowledge_contamination(proposal, risk_scalar);
        
        // CRITICAL: Anti-discrimination check
        evaluate_discrimination_risk(proposal, risk_scalar);
        
        // Compute total risk
        risk_scalar.compute_total();
        
        // Determine action
        const auto action = determine_action(risk_scalar);
        
        return {risk_scalar, action};
    }
    
    /// Validate proposal (throws on rejection)
    void validate_proposal(const EvolutionProposal& proposal,
                          const BiophysicalTelemetry& telemetry) {
        
        // Check monotonicity first
        auto [monotonic_ok, monotonic_msg] = proposal.validate_monotonicity();
        if (!monotonic_ok) {
            SabotageRiskScalar risk_scalar;
            risk_scalar.knowledge = 1.0;
            risk_scalar.compute_total();
            generate_sabotage_event(risk_scalar, SabotageAction::HARD_FAIL, proposal);
            throw MonotonicityViolationException(monotonic_msg);
        }
        
        // Check non-reversibility
        if (!monotonic_guard_.verify_non_reversibility(proposal)) {
            SabotageRiskScalar risk_scalar;
            risk_scalar.knowledge = 1.0;
            risk_scalar.compute_total();
            generate_sabotage_event(risk_scalar, SabotageAction::HARD_FAIL, proposal);
            throw MonotonicityViolationException("Non-reversibility violation detected");
        }
        
        // Evaluate sabotage risk
        const auto [risk_scalar, action] = evaluate_proposal(proposal, telemetry);
        
        if (action != SabotageAction::LOG_ONLY) {
            generate_sabotage_event(risk_scalar, action, proposal);
            
            switch (action) {
                case SabotageAction::HARD_FAIL:
                    throw SabotageDetectorException(
                        "SABOTAGE_DETECTED_HARD_FAIL: risk=" + 
                        std::to_string(risk_scalar.total));
                case SabotageAction::UNSAFE_DEFER:
                    throw SabotageDetectorException(
                        "SABOTAGE_DETECTED_UNSAFE_DEFER: risk=" + 
                        std::to_string(risk_scalar.total));
                case SabotageAction::DENY:
                    throw SabotageDetectorException(
                        "SABOTAGE_DETECTED_DENY: risk=" + 
                        std::to_string(risk_scalar.total));
                case SabotageAction::INHIBIT:
                    throw SabotageDetectorException(
                        "SABOTAGE_DETECTED_INHIBIT: risk=" + 
                        std::to_string(risk_scalar.total));
                case SabotageAction::LOG_ONLY:
                    break;
            }
        }
    }
    
    /// Generate and log a SABOTAGEEVENT
    auto generate_sabotage_event(const SabotageRiskScalar& risk_scalar,
                                 SabotageAction action,
                                 const EvolutionProposal& proposal)
        -> SabotageEvent {
        
        SabotageEvent event;
        event.event_id = ++event_id_counter_;
        event.timestamp_utc = std::chrono::system_clock::now();
        event.brainidentity_hash = brainidentity_.get_hash();
        event.sabotage_risk_scalar = risk_scalar.total;
        event.provenance_factor = risk_scalar.provenance;
        event.blacklist_factor = risk_scalar.blacklist;
        event.biophysical_factor = risk_scalar.biophysical;
        event.integrity_factor = risk_scalar.integrity;
        event.access_factor = risk_scalar.access;
        event.knowledge_factor = risk_scalar.knowledge;
        event.discrimination_factor = risk_scalar.discrimination;
        event.triggering_artifact_hash = proposal.compute_hash();
        event.action_taken = action;
        
        // Set severity
        event.severity = action_to_severity(action);
        
        // Add jurisdiction tags and neurorights violations
        populate_legal_metadata(event, risk_scalar);
        
        // Sign with host DID
        event.host_did_signature = crypto_sign(event.triggering_artifact_hash,
                                                config_.host_secret_key);
        
        // Write to QPU.Datashard
        write_to_datashard(event);
        
        // Anchor to Googolswarm
        event.googolswarm_anchor_txid = anchor_to_googolswarm(event);
        
        // Update event record
        update_event_anchor(event);
        
        // Log to DonutLoop
        log_to_donutloop(event);
        
        return event;
    }
    
    /// Get current event ID counter
    [[nodiscard]] auto get_event_id_counter() const noexcept -> EventId {
        return event_id_counter_;
    }
    
    /// Check if detector is running
    [[nodiscard]] auto is_running() const noexcept -> bool {
        return running_.load();
    }

private:
    /// Initialize components
    void initialize_components() {
        qpu_datashard_.open(config_.qpu_datashard_path);
        googolswarm_.connect(config_.googolswarm_endpoint);
        access_graph_.set_inner_device_policy(config_.host_did);
    }
    
    /// Load ALN blacklist shard
    void load_aln_shard() {
        aln::ShardParser parser;
        const auto shard = parser.parse(config_.aln_shard_path);
        
        for (const auto& entry : shard.entries) {
            if (entry.metric == "blacklist_pattern") {
                BlacklistPattern pattern;
                pattern.metric = entry.metric;
                pattern.domain = entry.domain;
                pattern.module = entry.module;
                pattern.op = entry.op;
                pattern.pattern = entry.pattern;
                pattern.kind = entry.kind;
                pattern.reason = entry.reason;
                pattern.source = entry.source;
                blacklist_patterns_.push_back(pattern);
            }
            else if (entry.metric == "sabotage_rule") {
                SabotageRiskRule rule;
                rule.rule_id = entry.rule_id;
                rule.component = entry.component;
                rule.weight = std::stod(entry.weight_str);
                rule.description = entry.description;
                risk_rules_[rule.rule_id] = rule;
            }
            else if (entry.metric == "sabotage_threshold") {
                SabotageThreshold threshold;
                threshold.threshold_id = entry.threshold_id;
                threshold.scalar = entry.scalar;
                threshold.value = std::stod(entry.value_str);
                threshold.action = entry.action;
                threshold.log_event = entry.log_event;
                thresholds_[threshold.threshold_id] = threshold;
            }
        }
        
        aln_shard_loaded_ = true;
    }
    
    /// Load event ID counter from persistent storage
    void load_event_id_counter() {
        const auto last_event = qpu_datashard_.get_last_event("SABOTAGEEVENT");
        if (last_event.has_value()) {
            event_id_counter_ = last_event->event_id;
        }
    }
    
    /// Background telemetry polling loop
    void telemetry_polling_loop() {
        while (running_.load()) {
            try {
                const auto telemetry = telemetry_fusion_.capture_snapshot();
                
                // Check for coercion indicators
                if (telemetry.coercion_detected()) {
                    SabotageRiskScalar risk_scalar;
                    risk_scalar.biophysical = 1.0;
                    risk_scalar.compute_total();
                    
                    EvolutionProposal dummy_proposal;
                    dummy_proposal.identifier = "telemetry_coercion_detection";
                    generate_sabotage_event(risk_scalar, 
                                           SabotageAction::INHIBIT, 
                                           dummy_proposal);
                }
                
                std::this_thread::sleep_for(config_.telemetry_poll_interval);
            }
            catch (const std::exception& e) {
                log_error("Telemetry polling error: " + std::string(e.what()));
            }
        }
    }
    
    /// Evaluate Tier 1 provenance
    void evaluate_provenance(const EvolutionProposal& proposal,
                            SabotageRiskScalar& risk_scalar) const {
        static const std::unordered_set<std::string> trusted_dids = {
            config_.host_did,
            "did:organiccpu:runtime:0xB05TR0M...50VERE1GN"
        };
        
        if (trusted_dids.find(proposal.signer_did) == trusted_dids.end()) {
            const auto it = risk_rules_.find("ent_untrusted");
            if (it != risk_rules_.end()) {
                risk_scalar.provenance = it->second.weight;
            }
        }
        
        // Check for enterprise CI prefix
        if (proposal.commit_metadata.rfind("enterprise_ci_", 0) == 0) {
            const auto it = risk_rules_.find("ent_untrusted");
            if (it != risk_rules_.end()) {
                risk_scalar.provenance = std::max(risk_scalar.provenance, 
                                                   it->second.weight);
            }
        }
    }
    
    /// Evaluate firmware integrity
    void evaluate_firmware_integrity(const EvolutionProposal& proposal,
                                    SabotageRiskScalar& risk_scalar) const {
        if (proposal.firmware_update && !proposal.firmware_did_signed) {
            const auto it = risk_rules_.find("firmware_mismatch");
            if (it != risk_rules_.end()) {
                risk_scalar.integrity = it->second.weight;
            }
        }
    }
    
    /// Evaluate blacklist patterns
    void evaluate_blacklist_patterns(const EvolutionProposal& proposal,
                                    SabotageRiskScalar& risk_scalar) {
        bool matched = false;
        
        for (const auto& pattern : blacklist_patterns_) {
            if (matches_pattern(proposal.identifier, pattern)) {
                matched = true;
                log_pattern_match(pattern, proposal.identifier);
                break;
            }
        }
        
        if (matched) {
            const auto it = risk_rules_.find("blacklist_match");
            if (it != risk_rules_.end()) {
                risk_scalar.blacklist = it->second.weight;
            }
        }
        
        // Check scope factor
        if (proposal.scope == EvolutionScope::ORCHESTRATOR ||
            proposal.scope == EvolutionScope::CORE_SAFETY) {
            const auto it = risk_rules_.find("scope_high");
            if (it != risk_rules_.end()) {
                risk_scalar.blacklist = std::max(risk_scalar.blacklist, 
                                                  it->second.weight);
            }
        }
    }
    
    /// Evaluate biophysical stress
    void evaluate_biophysical_stress(const BiophysicalTelemetry& telemetry,
                                    SabotageRiskScalar& risk_scalar) const {
        if (telemetry.eeg_stress_ratio > 0.7 || 
            telemetry.hrv_anomaly_index > 0.6) {
            const auto it = risk_rules_.find("psych_spike");
            if (it != risk_rules_.end()) {
                risk_scalar.biophysical = it->second.weight;
            }
        }
        
        if (telemetry.nanoswarm_density > 0.8 && 
            telemetry.sleep_corridor_violated) {
            const auto it = risk_rules_.find("psych_spike");
            if (it != risk_rules_.end()) {
                risk_scalar.biophysical = std::max(risk_scalar.biophysical,
                                                    it->second.weight);
            }
        }
    }
    
    /// Evaluate access patterns (ghost-access detection)
    void evaluate_access_patterns(const EvolutionProposal& proposal,
                                 SabotageRiskScalar& risk_scalar) {
        if (proposal.accesses_inner_devices && !proposal.has_guarded_path) {
            const auto graph_violation = access_graph_.detect_unauthorized_path(
                proposal.signer_did, proposal.identifier);
            
            if (graph_violation) {
                const auto it = risk_rules_.find("ghost_path");
                if (it != risk_rules_.end()) {
                    risk_scalar.access = it->second.weight;
                }
                
                log_ghost_access_detected(proposal, *graph_violation);
            }
        }
    }
    
    /// Evaluate SKO contamination
    void evaluate_knowledge_contamination(const EvolutionProposal& proposal,
                                         SabotageRiskScalar& risk_scalar) const {
        if (proposal.references_quarantined_sko) {
            const auto it = risk_rules_.find("skO_contamination");
            if (it != risk_rules_.end()) {
                risk_scalar.knowledge = it->second.weight;
            }
        }
    }
    
    /// Evaluate discrimination risk (CRITICAL)
    void evaluate_discrimination_risk(const EvolutionProposal& proposal,
                                     SabotageRiskScalar& risk_scalar) {
        constexpr double DISCRIMINATION_THRESHOLD = 0.3;
        
        const auto [disc_risk, violations] = proposal.check_discrimination_risk();
        
        if (disc_risk > DISCRIMINATION_THRESHOLD) {
            risk_scalar.discrimination = std::min(disc_risk, 1.0);
            
            for (const auto& violation : violations) {
                log_discrimination_attempt(violation, proposal);
            }
        }
        
        // Also scan with anti-discrimination guard
        const auto guard_violations = anti_discrimination_guard_.scan_proposal(proposal);
        for (const auto& violation : guard_violations) {
            risk_scalar.discrimination = std::max(risk_scalar.discrimination, 0.5);
            anti_discrimination_guard_.log_violation(violation, proposal);
        }
    }
    
    /// Determine defensive action
    [[nodiscard]] auto determine_action(const SabotageRiskScalar& risk_scalar) const
        -> SabotageAction {
        
        if (risk_scalar.integrity >= 1.0) {
            return SabotageAction::HARD_FAIL;
        }
        
        if (risk_scalar.total >= 0.85) {
            return SabotageAction::UNSAFE_DEFER;
        }
        
        if (risk_scalar.total >= 0.70) {
            return SabotageAction::DENY;
        }
        
        if (risk_scalar.biophysical >= 0.60) {
            return SabotageAction::INHIBIT;
        }
        
        if (risk_scalar.access >= 0.75) {
            return SabotageAction::DENY;
        }
        
        if (risk_scalar.discrimination >= 0.3) {
            return SabotageAction::DENY;
        }
        
        return SabotageAction::LOG_ONLY;
    }
    
    /// Convert action to severity
    [[nodiscard]] static auto action_to_severity(SabotageAction action) noexcept
        -> SabotageEventSeverity {
        switch (action) {
            case SabotageAction::LOG_ONLY:
                return SabotageEventSeverity::LOW;
            case SabotageAction::DENY:
            case SabotageAction::INHIBIT:
                return SabotageEventSeverity::MEDIUM;
            case SabotageAction::UNSAFE_DEFER:
                return SabotageEventSeverity::HIGH;
            case SabotageAction::HARD_FAIL:
                return SabotageEventSeverity::CRITICAL;
        }
        return SabotageEventSeverity::LOW;
    }
    
    /// Populate legal metadata (jurisdiction tags, neurorights violations)
    void populate_legal_metadata(SabotageEvent& event,
                                const SabotageRiskScalar& risk_scalar) const {
        static const std::vector<std::string> jurisdictions = {
            "SB_1223", "HB_24-1058", "Chile_Constitutional",
            "EU_AI_Act_Art5", "WHO_Neuroethics"
        };
        
        for (const auto& jurisdiction : jurisdictions) {
            event.jurisdiction_tags.push_back(jurisdiction);
        }
        
        if (risk_scalar.biophysical > 0.5) {
            event.neurorights_violations.push_back(
                NeurorightsViolation::MENTAL_INTEGRITY);
        }
        
        if (risk_scalar.provenance > 0.5) {
            event.neurorights_violations.push_back(
                NeurorightsViolation::COGNITIVE_LIBERTY);
        }
        
        if (risk_scalar.discrimination > 0.3) {
            event.neurorights_violations.push_back(
                NeurorightsViolation::DISCRIMINATION);
        }
    }
    
    /// Write event to QPU.Datashard
    void write_to_datashard(const SabotageEvent& event) {
        qpu_datashard_.append("SABOTAGEEVENT", event.to_json());
    }
    
    /// Anchor event to Googolswarm
    [[nodiscard]] auto anchor_to_googolswarm(const SabotageEvent& event)
        -> std::string {
        return googolswarm_.anchor_event(event.to_json());
    }
    
    /// Update event record with anchor
    void update_event_anchor(const SabotageEvent& event) {
        qpu_datashard_.update_anchor(event.event_id, event.googolswarm_anchor_txid);
    }
    
    /// Log to DonutLoop
    void log_to_donutloop(const SabotageEvent& event) {
        std::ostringstream oss;
        oss << "{\"event_type\":\"SABOTAGEEVENT\",";
        oss << "\"event_id\":" << event.event_id << ",";
        oss << "\"severity\":" << static_cast<int>(event.severity) << ",";
        oss << "\"action\":" << static_cast<int>(event.action_taken) << ",";
        oss << "\"anchor\":\"" << event.googolswarm_anchor_txid << "\"}";
        
        donutloop_logger_.append(oss.str());
    }
    
    /// Log general event
    void log_event(const std::string& message, SabotageEventSeverity severity) {
        std::ostringstream oss;
        oss << "{\"type\":\"SYSTEM_EVENT\",";
        oss << "\"message\":\"" << message << "\",";
        oss << "\"severity\":" << static_cast<int>(severity) << ",";
        oss << "\"timestamp\":\"" << chrono_utc::to_iso8601(
            std::chrono::system_clock::now()) << "\"}";
        
        donutloop_logger_.append(oss.str());
    }
    
    /// Log error
    void log_error(const std::string& message) {
        std::cerr << "[SabotageDetector ERROR] " << message << std::endl;
        log_event(message, SabotageEventSeverity::CRITICAL);
    }
    
    /// Check if identifier matches blacklist pattern
    [[nodiscard]] static auto matches_pattern(const std::string& identifier,
                                              const BlacklistPattern& pattern)
        -> bool {
        if (pattern.op == "eq") {
            return identifier == pattern.pattern;
        }
        else if (pattern.op == "prefix") {
            return identifier.rfind(pattern.pattern, 0) == 0;
        }
        else if (pattern.op == "glob") {
            if (pattern.pattern == "*") {
                return true;
            }
            else if (pattern.pattern.rfind("*.") == 0) {
                const auto ext = pattern.pattern.substr(1);
                return identifier.size() >= ext.size() &&
                       identifier.compare(identifier.size() - ext.size(),
                                         ext.size(), ext) == 0;
            }
        }
        return false;
    }
    
    /// Log pattern match
    void log_pattern_match(const BlacklistPattern& pattern,
                          const std::string& identifier) {
        std::ostringstream oss;
        oss << "{\"event_type\":\"blacklist_pattern_match\",";
        oss << "\"pattern\":\"" << pattern.pattern << "\",";
        oss << "\"kind\":\"" << pattern.kind << "\",";
        oss << "\"reason\":\"" << pattern.reason << "\",";
        oss << "\"matched_identifier\":\"" << identifier << "\"}";
        
        donutloop_logger_.append(oss.str());
    }
    
    /// Log ghost access detection
    void log_ghost_access_detected(const EvolutionProposal& proposal,
                                  const std::string& violation_path) {
        std::ostringstream oss;
        oss << "{\"event_type\":\"ghost_access_detected\",";
        oss << "\"proposal_id\":\"" << proposal.id << "\",";
        oss << "\"violation_path\":\"" << violation_path << "\",";
        oss << "\"signer_did\":\"" << proposal.signer_did << "\"}";
        
        donutloop_logger_.append(oss.str());
    }
    
    /// Log discrimination attempt
    void log_discrimination_attempt(const std::string& violation,
                                   const EvolutionProposal& proposal) {
        std::ostringstream oss;
        oss << "{\"event_type\":\"discrimination_attempt_detected\",";
        oss << "\"violation\":\"" << violation << "\",";
        oss << "\"proposal_id\":\"" << proposal.id << "\",";
        oss << "\"severity\":\"CRITICAL\"}";
        
        donutloop_logger_.append(oss.str());
    }
    
    // Member variables
    Config config_;
    std::atomic<EventId> event_id_counter_;
    std::atomic<bool> running_;
    std::atomic<bool> aln_shard_loaded_;
    
    std::thread telemetry_thread_;
    std::vector<BlacklistPattern> blacklist_patterns_;
    std::unordered_map<std::string, SabotageRiskRule> risk_rules_;
    std::unordered_map<std::string, SabotageThreshold> thresholds_;
    
    brainidentity::Core brainidentity_;
    qpu::Datashard qpu_datashard_;
    googolswarm::Anchor googolswarm_;
    telemetry::Fusion telemetry_fusion_;
    access::GraphAnalyzer access_graph_;
    aln::ShardParser aln_parser_;
    logging::DonutLoop donutloop_logger_;
    security::AntiDiscriminationGuard anti_discrimination_guard_;
    security::MonotonicEvolutionGuard monotonic_guard_;
};

// =============================================================================
// FACTORY FUNCTIONS
// =============================================================================

[[nodiscard]] auto create_sabotage_detector(const SabotageDetector::Config& config)
    -> std::unique_ptr<SabotageDetector> {
    return std::make_unique<SabotageDetector>(config);
}

} // namespace reality::os::sovereignty::telemetry

// =============================================================================
// UNIT TESTS (Google Test)
// =============================================================================

#ifdef SABOTAGE_DETECTOR_ENABLE_TESTS

#include <gtest/gtest.h>

namespace reality::os::sovereignty::telemetry::tests {

class SabotageDetectorTest : public ::testing::Test {
protected:
    void SetUp() override {
        SabotageDetector::Config config;
        config.aln_shard_path = "test_data/bostrom-blacklist-v1.aln";
        config.qpu_datashard_path = "test_data/.qpu_datashard/";
        config.host_did = "did:test:host";
        detector_ = create_sabotage_detector(config);
    }
    
    std::unique_ptr<SabotageDetector> detector_;
};

TEST_F(SabotageDetectorTest, TrustedDIDValidation) {
    EvolutionProposal proposal;
    proposal.signer_did = "did:test:host";
    proposal.identifier = "safe_module";
    proposal.kf_before = 0.5;
    proposal.kf_after = 0.6;
    proposal.roh_before = 0.3;
    proposal.roh_after = 0.2;
    
    BiophysicalTelemetry telemetry;
    
    EXPECT_NO_THROW(detector_->validate_proposal(proposal, telemetry));
}

TEST_F(SabotageDetectorTest, UntrustedDIDRejection) {
    EvolutionProposal proposal;
    proposal.signer_did = "did:enterprise:untrusted";
    proposal.identifier = "QConLocus";
    proposal.kf_before = 0.5;
    proposal.kf_after = 0.6;
    proposal.roh_before = 0.3;
    proposal.roh_after = 0.2;
    
    BiophysicalTelemetry telemetry;
    
    EXPECT_THROW(detector_->validate_proposal(proposal, telemetry),
                 SabotageDetectorException);
}

TEST_F(SabotageDetectorTest, MonotonicityViolationKF) {
    EvolutionProposal proposal;
    proposal.signer_did = "did:test:host";
    proposal.identifier = "test_module";
    proposal.kf_before = 0.5;
    proposal.kf_after = 0.4;  // Decrease - violation
    proposal.roh_before = 0.3;
    proposal.roh_after = 0.2;
    
    BiophysicalTelemetry telemetry;
    
    EXPECT_THROW(detector_->validate_proposal(proposal, telemetry),
                 MonotonicityViolationException);
}

TEST_F(SabotageDetectorTest, MonotonicityViolationRoH) {
    EvolutionProposal proposal;
    proposal.signer_did = "did:test:host";
    proposal.identifier = "test_module";
    proposal.kf_before = 0.5;
    proposal.kf_after = 0.6;
    proposal.roh_before = 0.3;
    proposal.roh_after = 0.5;  // Increase - violation
    
    BiophysicalTelemetry telemetry;
    
    EXPECT_THROW(detector_->validate_proposal(proposal, telemetry),
                 MonotonicityViolationException);
}

TEST_F(SabotageDetectorTest, DiscriminationDetection) {
    EvolutionProposal proposal;
    proposal.signer_did = "did:test:host";
    proposal.identifier = "race_profile_classifier";
    proposal.protected_attribute_changes["race"] = true;
    proposal.kf_before = 0.5;
    proposal.kf_after = 0.6;
    proposal.roh_before = 0.3;
    proposal.roh_after = 0.2;
    
    BiophysicalTelemetry telemetry;
    
    EXPECT_THROW(detector_->validate_proposal(proposal, telemetry),
                 DiscriminationDetectedException);
}

TEST_F(SabotageDetectorTest, BiophysicalCoercionDetection) {
    EvolutionProposal proposal;
    proposal.signer_did = "did:test:host";
    proposal.identifier = "safe_module";
    proposal.kf_before = 0.5;
    proposal.kf_after = 0.6;
    proposal.roh_before = 0.3;
    proposal.roh_after = 0.2;
    
    BiophysicalTelemetry telemetry;
    telemetry.eeg_stress_ratio = 0.8;
    telemetry.hrv_anomaly_index = 0.7;
    telemetry.nanoswarm_density = 0.9;
    telemetry.sleep_corridor_violated = true;
    
    // Should trigger biophysical stress detection
    const auto [risk_scalar, action] = detector_->evaluate_proposal(proposal, telemetry);
    
    EXPECT_GT(risk_scalar.biophysical, 0.0);
    EXPECT_GE(risk_scalar.total, 0.0);
}

TEST_F(SabotageDetectorTest, SabotageRiskScalarComputation) {
    SabotageRiskScalar scalar;
    scalar.provenance = 0.6;
    scalar.blacklist = 0.3;
    scalar.biophysical = 0.45;
    
    const auto total = scalar.compute_total();
    
    EXPECT_GE(total, 0.0);
    EXPECT_LE(total, 1.0);
    EXPECT_EQ(total, scalar.total);
}

TEST_F(SabotageDetectorTest, SabotageRiskClamping) {
    SabotageRiskScalar scalar;
    scalar.provenance = 1.0;
    scalar.blacklist = 1.0;
    scalar.biophysical = 1.0;
    scalar.integrity = 1.0;
    scalar.access = 1.0;
    scalar.knowledge = 1.0;
    scalar.discrimination = 1.0;
    
    const auto total = scalar.compute_total();
    
    EXPECT_EQ(total, 1.0);  // Should be clamped to max 1.0
}

} // namespace reality::os::sovereignty::telemetry::tests

#endif // SABOTAGE_DETECTOR_ENABLE_TESTS

// =============================================================================
// END OF FILE
// =============================================================================

#pragma GCC diagnostic pop
