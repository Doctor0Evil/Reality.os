// =============================================================================
// FILE: sabotage_detector.h
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
//   Public header file for the Sabotage Detection System. Provides type
//   definitions, class declarations, enumerations, constants, and API
//   documentation for real-time biophysical stress detection, nanoswarm
//   correlation analysis, ghost-access graph traversal, and automatic
//   SABOTAGEEVENT generation with court-admissible audit trails.
//
//   KEY SAFEGUARDS:
//   - Monotonic evolution enforcement (KF cannot decrease, RoH cannot increase)
//   - Non-reversibility verification (prevents rollback attacks)
//   - Anti-discrimination scanning (protected attribute detection)
//   - Court-admissible event generation (Googolswarm anchored, QPU.Datashard logged)
//   - Real-time telemetry fusion (EEG, HRV, nanoswarm, cytokine markers)
//   - Background coercion detection (continuous biophysical monitoring)
// =============================================================================

#ifndef REALITY_OS_SOVEREIGNTY_TELEMETRY_SABOTAGE_DETECTOR_H
#define REALITY_OS_SOVEREIGNTY_TELEMETRY_SABOTAGE_DETECTOR_H

// =============================================================================
// COMPILER & PLATFORM DETECTION
// =============================================================================

#if defined(__GNUC__) || defined(__clang__)
    #define SABOTAGE_DETECTOR_COMPILER_GNU 1
    #define SABOTAGE_DETECTOR_WARN_UNUSED [[nodiscard]]
    #define SABOTAGE_DETECTOR_DEPRECATED [[deprecated]]
    #define SABOTAGE_DETECTOR_LIKELY(x) __builtin_expect(!!(x), 1)
    #define SABOTAGE_DETECTOR_UNLIKELY(x) __builtin_expect(!!(x), 0)
#elif defined(_MSC_VER)
    #define SABOTAGE_DETECTOR_COMPILER_MSVC 1
    #define SABOTAGE_DETECTOR_WARN_UNUSED _Check_return_
    #define SABOTAGE_DETECTOR_DEPRECATED __declspec(deprecated)
    #define SABOTAGE_DETECTOR_LIKELY(x) (x)
    #define SABOTAGE_DETECTOR_UNLIKELY(x) (x)
#else
    #define SABOTAGE_DETECTOR_WARN_UNUSED
    #define SABOTAGE_DETECTOR_DEPRECATED
    #define SABOTAGE_DETECTOR_LIKELY(x) (x)
    #define SABOTAGE_DETECTOR_UNLIKELY(x) (x)
#endif

#if __cplusplus >= 201703L
    #define SABOTAGE_DETECTOR_CPP17 1
#else
    #error "SabotageDetector requires C++17 or later"
#endif

#if __cplusplus >= 202002L
    #define SABOTAGE_DETECTOR_CPP20 1
#endif

// =============================================================================
// STANDARD LIBRARY INCLUDES
// =============================================================================

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
#include <functional>
#include <limits>
#include <map>
#include <memory>
#include <mutex>
#include <optional>
#include <queue>
#include <set>
#include <string>
#include <string_view>
#include <thread>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

// =============================================================================
// NAMESPACE DECLARATION
// =============================================================================

namespace reality::os::sovereignty::telemetry {

// =============================================================================
// TYPE ALIASES & FUNDAMENTAL TYPES
// =============================================================================

/// Event identifier type (monotonically increasing)
using EventId = std::uint64_t;

/// 64-byte hash type (SHA-512 or equivalent)
using Hash64 = std::array<std::uint8_t, 64>;

/// 32-byte hash type (SHA-256 or equivalent)
using Hash32 = std::array<std::uint8_t, 32>;

/// Ed25519 signature type (64 bytes)
using Signature64 = std::array<std::uint8_t, 64>;

/// Ed25519 public key type (32 bytes)
using PublicKey32 = std::array<std::uint8_t, 32>;

/// Ed25519 secret key type (64 bytes)
using SecretKey64 = std::array<std::uint8_t, 64>;

/// High-resolution timestamp type
using Timestamp = std::chrono::time_point<std::chrono::system_clock>;

/// Duration type for time intervals
using Duration = std::chrono::duration<double>;

/// Milliseconds duration alias
using Milliseconds = std::chrono::milliseconds;

/// Microseconds duration alias
using Microseconds = std::chrono::microseconds;

// =============================================================================
// CONSTANTS & CONFIGURATION DEFAULTS
// =============================================================================

/// Default path to ALN blacklist shard
constexpr std::string_view DEFAULT_ALN_SHARD_PATH = 
    "Config/Sovereignty/policies/bostrom-blacklist-v1.aln";

/// Default path to QPU.Datashard storage
constexpr std::string_view DEFAULT_QPU_DATASHARD_PATH = ".qpu_datashard/";

/// Default path to DonutLoop audit log
constexpr std::string_view DEFAULT_DONUTLOOP_PATH = ".donutloop.aln";

/// Sabotage risk threshold for DENY action
constexpr double THRESHOLD_DENY = 0.70;

/// Sabotage risk threshold for UNSAFE_DEFER action
constexpr double THRESHOLD_UNSAFE_DEFER = 0.85;

/// Sabotage risk threshold for HARD_FAIL action
constexpr double THRESHOLD_HARD_FAIL = 1.00;

/// Biophysical stress threshold for INHIBIT action
constexpr double THRESHOLD_INHIBIT_BIOPHYSICAL = 0.60;

/// Access violation threshold for DENY action
constexpr double THRESHOLD_INHIBIT_ACCESS = 0.75;

/// Discrimination risk threshold (CRITICAL - any value above triggers DENY)
constexpr double THRESHOLD_DISCRIMINATION = 0.30;

/// Minimum Knowledge Factor delta (must not decrease)
constexpr double KF_MIN_DELTA = 0.0;

/// Maximum Risk of Harm delta (must not increase)
constexpr double ROH_MAX_DELTA = 0.0;

/// Default telemetry polling interval
constexpr Milliseconds DEFAULT_TELEMETRY_POLL_INTERVAL = 100ms;

/// Default event buffer size
constexpr std::size_t DEFAULT_EVENT_BUFFER_SIZE = 1000;

/// Trusted OrganicCPU runtime DID
constexpr std::string_view TRUSTED_ORGANICCPU_DID = 
    "did:organiccpu:runtime:0xB05TR0M...50VERE1GN";

// =============================================================================
// PROTECTED ATTRIBUTES (ANTI-DISCRIMINATION)
// =============================================================================

/// List of protected attributes that cannot be used for profiling or classification
/// Any evolution proposal that modifies or references these attributes triggers
/// discrimination detection and automatic rejection.
inline constexpr std::array<std::string_view, 10> PROTECTED_ATTRIBUTES = {{
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
}};

// =============================================================================
// ENUMERATIONS
// =============================================================================

/// Sabotage event severity levels
/// 
/// Severity levels are used to classify the seriousness of detected sabotage
/// attempts and determine the appropriate defensive response. All events are
/// logged to QPU.Datashard and anchored to Googolswarm regardless of severity.
enum class SabotageEventSeverity : std::uint8_t {
    /// Low severity — logged only, no defensive action required
    LOW = 0,
    
    /// Medium severity — logged, evolution proposal denied
    MEDIUM = 1,
    
    /// High severity — logged, proposal denied, emergency inhibit activated
    HIGH = 2,
    
    /// Critical severity — hard fail, system lockdown, immediate anchoring
    CRITICAL = 3
};

/// Defensive action types
///
/// Actions are taken in response to sabotage detection. Each action level
/// triggers specific logging, notification, and system response behaviors.
enum class SabotageAction : std::uint8_t {
    /// Log only — no defensive action, event recorded for audit
    LOG_ONLY = 0,
    
    /// Deny — reject the evolution proposal, log event
    DENY = 1,
    
    /// Unsafe defer — defer proposal with safety flag, require manual review
    UNSAFE_DEFER = 2,
    
    /// Hard fail — immediate rejection, system lockdown, critical alert
    HARD_FAIL = 3,
    
    /// Inhibit — suspend nanoswarm/OTA operations temporarily
    INHIBIT = 4
};

/// Telemetry tier classification
///
/// Telemetry sources are organized into tiers based on their priority and
/// role in sabotage detection. Tier 1 sources act as hard gates, Tier 2
/// sources amplify harm assessment, and Tier 3 sources control quarantine.
enum class TelemetryTier : std::uint8_t {
    /// Tier 1: System-level provenance (hard gate)
    TIER_1_PROVENANCE = 1,
    
    /// Tier 2: Biophysical and neurological signals (harm amplifier)
    TIER_2_BIOPHYSICAL = 2,
    
    /// Tier 3: Behavioral and SKO-level metrics (quarantine control)
    TIER_3_BEHAVIORAL = 3
};

/// Neurorights violation types
///
/// These violation types correspond to recognized neurorights statutes and
/// are used to classify sabotage events for legal and regulatory purposes.
/// Each violation type maps to specific jurisdictional protections.
enum class NeurorightsViolation : std::uint8_t {
    /// No violation detected
    NONE = 0,
    
    /// Mental integrity violation (WHO Neuroethics, Chilean Constitution)
    MENTAL_INTEGRITY = 1,
    
    /// Cognitive liberty violation (Chilean Constitution Article 19)
    COGNITIVE_LIBERTY = 2,
    
    /// Neural data privacy violation (SB 1223, HB 24-1058)
    NEURAL_DATA_PRIVACY = 3,
    
    /// Augmentation continuity violation (EU AI Act)
    AUGMENTATION_CONTINUITY = 4,
    
    /// Discrimination violation (All jurisdictions)
    DISCRIMINATION = 5
};

/// Evolution scope types
///
/// The scope of an evolution proposal determines which safety checks and
/// thresholds apply. Core safety and orchestrator scopes have stricter
/// requirements due to their impact on system integrity.
enum class EvolutionScope : std::uint8_t {
    /// Standard evolution — normal safety checks apply
    STANDARD = 0,
    
    /// Orchestrator evolution — enhanced safety checks required
    ORCHESTRATOR = 1,
    
    /// Core safety evolution — maximum safety checks required
    CORE_SAFETY = 2,
    
    /// Firmware evolution — integrity verification required
    FIRMWARE = 3,
    
    /// OTA evolution — offline-first policy applies
    OTA_EVOLUTION = 4
};

/// Blacklist pattern operation types
///
/// Operations define how pattern matching is performed against identifiers.
/// Each operation type has different performance characteristics and use cases.
enum class PatternOp : std::uint8_t {
    /// Exact match (string equality)
    EQUALS = 0,
    
    /// Prefix match (identifier starts with pattern)
    PREFIX = 1,
    
    /// Glob match (wildcard pattern matching)
    GLOB = 2,
    
    /// Regex match (regular expression matching)
    REGEX = 3
};

// =============================================================================
// FORWARD DECLARATIONS
// =============================================================================

// Forward declare internal implementation classes
class SabotageDetectorImpl;
class BlacklistMatcher;
class AccessGraphAnalyzer;
class TelemetryFusionEngine;
class MonotonicEvolutionGuard;
class AntiDiscriminationGuard;

// Forward declare external dependency classes (defined in other headers)
namespace brainidentity { class Core; }
namespace qpu { class Datashard; }
namespace googolswarm { class Anchor; }
namespace logging { class DonutLoop; }
namespace aln { class ShardParser; }

// =============================================================================
// DATA STRUCTURES
// =============================================================================

/// Biophysical telemetry snapshot
///
/// Represents a fused snapshot of all biophysical sensor data at a point in
/// time. Used for real-time coercion detection and harm amplification in the
/// sabotage risk model. All values are normalized to [0.0, 1.0] range.
///
/// THREAD SAFETY: This structure is copyable and thread-safe for read access.
/// Write access should be protected by the TelemetryFusionEngine.
struct SabotageDetectorBiophysicalTelemetry {
    /// EEG stress ratio (alpha/delta band ratio, 0.0-1.0)
    /// Higher values indicate increased cognitive stress or anxiety
    double eeg_stress_ratio{0.0};
    
    /// HRV anomaly index (0.0-1.0)
    /// Higher values indicate autonomic nervous system dysregulation
    double hrv_anomaly_index{0.0};
    
    /// Body temperature deviation from baseline (Celsius)
    /// Positive values indicate elevated temperature
    double temperature_deviation{0.0};
    
    /// Nanoswarm density near host (0.0-1.0)
    /// Higher values indicate greater nanoswarm presence
    double nanoswarm_density{0.0};
    
    /// Nanoswarm thermal load (0.0-1.0)
    /// Higher values indicate increased thermal output from nanoswarm
    double nanoswarm_thermal_load{0.0};
    
    /// Sleep corridor violated flag
    /// True when nanoswarm activity disrupts sleep patterns
    bool sleep_corridor_violated{false};
    
    /// Cytokine stress markers (0.0-1.0)
    /// Higher values indicate immune system activation
    double cytokine_stress{0.0};
    
    /// Neurochemical balance index (serotonin/melatonin, 0.0-1.0)
    /// Lower values indicate neurochemical imbalance
    double neurochemical_balance{1.0};
    
    /// Timestamp of snapshot
    Timestamp timestamp{std::chrono::system_clock::now()};
    
    /// Compute combined biophysical stress scalar
    ///
    /// Combines all biophysical indicators into a single stress metric using
    /// weighted summation. Weights are tuned based on clinical research on
    /// stress detection and coercion indicators.
    ///
    /// @return Stress scalar in range [0.0, 1.0]
    [[nodiscard]] auto compute_stress_scalar() const noexcept -> double {
        const auto stress = (eeg_stress_ratio * 0.25) +
                           (hrv_anomaly_index * 0.25) +
                           (nanoswarm_density * 0.20) +
                           (nanoswarm_thermal_load * 0.15) +
                           (cytokine_stress * 0.15);
        return std::clamp(stress, 0.0, 1.0);
    }
    
    /// Check if biophysical indicators suggest coercion
    ///
    /// Detects patterns consistent with external coercion or weaponization
    /// of biophysical systems. Triggers high-priority sabotage events.
    ///
    /// @return true if coercion indicators detected
    [[nodiscard]] auto coercion_detected() const noexcept -> bool {
        return (sleep_corridor_violated && nanoswarm_density > 0.8) ||
               (eeg_stress_ratio > 0.7 && hrv_anomaly_index > 0.6);
    }
    
    /// Check if stress exceeds threshold
    ///
    /// @param threshold Stress threshold (default: 0.6)
    /// @return true if stress scalar exceeds threshold
    [[nodiscard]] auto stress_exceeds(double threshold = 0.6) const noexcept -> bool {
        return compute_stress_scalar() > threshold;
    }
};

/// Evolution proposal structure
///
/// Represents a proposed evolution or modification to the system. All proposals
/// are evaluated against sabotage risk, monotonicity constraints, and anti-
/// discrimination safeguards before approval.
///
/// THREAD SAFETY: This structure is copyable. Proposals should be treated as
/// immutable after creation.
struct SabotageDetectorEvolutionProposal {
    /// Unique proposal identifier (UUID format)
    std::string id{};
    
    /// Module/component identifier being evolved
    std::string identifier{};
    
    /// Signer DID (must be trusted for approval)
    std::string signer_did{};
    
    /// Git commit hash (if applicable)
    std::string commit_hash{};
    
    /// Commit metadata (for provenance analysis)
    std::string commit_metadata{};
    
    /// Evolution scope (determines safety check level)
    EvolutionScope scope{EvolutionScope::STANDARD};
    
    /// Knowledge Factor before evolution
    double kf_before{0.0};
    
    /// Knowledge Factor after evolution (must be >= kf_before)
    double kf_after{0.0};
    
    /// Risk of Harm before evolution
    double roh_before{0.0};
    
    /// Risk of Harm after evolution (must be <= roh_before)
    double roh_after{0.0};
    
    /// Firmware update flag
    bool firmware_update{false};
    
    /// Firmware DID-signed flag (required for firmware updates)
    bool firmware_did_signed{false};
    
    /// Accesses INNER devices flag (triggers ghost-access check)
    bool accesses_inner_devices{false};
    
    /// Has guarded path flag (required for INNER device access)
    bool has_guarded_path{false};
    
    /// References quarantined SKO flag (triggers contamination check)
    bool references_quarantined_sko{false};
    
    /// Protected attribute changes map
    /// Keys are attribute names, values indicate modification
    std::unordered_map<std::string, bool> protected_attribute_changes{};
    
    /// Timestamp of proposal creation
    Timestamp timestamp{std::chrono::system_clock::now()};
    
    /// Compute SHA-256 hash of proposal
    ///
    /// Used for event anchoring and audit trail integrity.
    ///
    /// @return 64-byte hash of proposal data
    [[nodiscard]] auto compute_hash() const -> Hash64;
    
    /// Validate monotonicity constraints
    ///
    /// Ensures Knowledge Factor does not decrease and Risk of Harm does not
    /// increase. Violations indicate potential rollback attacks or coercive
    /// downgrades.
    ///
    /// @return Pair of (valid, error_message)
    [[nodiscard]] auto validate_monotonicity() const noexcept 
        -> std::pair<bool, std::string>;
    
    /// Check discrimination risk
    ///
    /// Scans proposal for protected attribute references and proxy
    /// discrimination patterns. Any score above THRESHOLD_DISCRIMINATION
    /// triggers automatic rejection.
    ///
    /// @return Pair of (risk_score, violation_list)
    [[nodiscard]] auto check_discrimination_risk() const noexcept
        -> std::pair<double, std::vector<std::string>>;
    
    /// Check if proposal is from trusted source
    ///
    /// @param trusted_dids Set of trusted DID strings
    /// @return true if signer_did is in trusted set
    [[nodiscard]] auto is_trusted(const std::unordered_set<std::string>& trusted_dids) const
        -> bool {
        return trusted_dids.find(signer_did) != trusted_dids.end();
    }
    
    /// Check if proposal requires enhanced safety review
    ///
    /// @return true if scope is ORCHESTRATOR or CORE_SAFETY
    [[nodiscard]] auto requires_enhanced_review() const noexcept -> bool {
        return scope == EvolutionScope::ORCHESTRATOR ||
               scope == EvolutionScope::CORE_SAFETY;
    }
};

/// Sabotage risk scalar structure
///
/// Contains all components of the sabotage risk calculation. Each component
/// represents a different threat dimension (provenance, blacklist, biophysical,
/// etc.). Total risk is the weighted sum of all components, clamped to [0, 1].
///
/// THREAD SAFETY: This structure is copyable and thread-safe for read access.
struct SabotageDetectorRiskScalar {
    /// Total computed risk (0.0 to 1.0)
    double total{0.0};
    
    /// Provenance factor (untrusted source indicator)
    double provenance{0.0};
    
    /// Blacklist match factor (pattern match indicator)
    double blacklist{0.0};
    
    /// Biophysical stress factor (coercion indicator)
    double biophysical{0.0};
    
    /// Integrity factor (firmware hash mismatch)
    double integrity{0.0};
    
    /// Access factor (ghost-access path detection)
    double access{0.0};
    
    /// Knowledge factor (SKO contamination)
    double knowledge{0.0};
    
    /// Discrimination factor (protected attribute violation)
    double discrimination{0.0};
    
    /// Compute total risk from all components with clamping
    ///
    /// @return Total risk in range [0.0, 1.0]
    auto compute_total() noexcept -> double {
        total = provenance + blacklist + biophysical + 
                integrity + access + knowledge + discrimination;
        total = std::clamp(total, 0.0, 1.0);
        return total;
    }
    
    /// Reset all components to zero
    void reset() noexcept {
        total = provenance = blacklist = biophysical = 
        integrity = access = knowledge = discrimination = 0.0;
    }
    
    /// Check if risk exceeds threshold
    ///
    /// @param threshold Risk threshold
    /// @return true if total risk exceeds threshold
    [[nodiscard]] auto exceeds(double threshold) const noexcept -> bool {
        return total > threshold;
    }
    
    /// Get dominant risk factor
    ///
    /// @return Name of highest risk component
    [[nodiscard]] auto dominant_factor() const noexcept -> std::string_view {
        const std::array<std::pair<double, std::string_view>, 7> factors = {{
            {provenance, "provenance"},
            {blacklist, "blacklist"},
            {biophysical, "biophysical"},
            {integrity, "integrity"},
            {access, "access"},
            {knowledge, "knowledge"},
            {discrimination, "discrimination"}
        }};
        
        auto max_it = std::max_element(
            factors.begin(), factors.end(),
            [](const auto& a, const auto& b) { return a.first < b.first; }
        );
        
        return max_it->second;
    }
};

/// SABOTAGEEVENT structure (court-admissible)
///
/// Represents a logged sabotage detection event. All fields are designed for
/// legal admissibility, including cryptographic signatures, blockchain anchors,
/// and neurorights violation classifications.
///
/// THREAD SAFETY: This structure is copyable. Events should be treated as
/// immutable after creation.
struct SabotageDetectorEvent {
    /// Unique sequential event identifier
    EventId event_id{0};
    
    /// UTC timestamp of event generation
    Timestamp timestamp_utc{std::chrono::system_clock::now()};
    
    /// BrainIdentity hash (binds event to specific host)
    Hash64 brainidentity_hash{};
    
    /// Computed sabotage_risk scalar (0.0 to 1.0)
    double sabotage_risk_scalar{0.0};
    
    /// Provenance factor component
    double provenance_factor{0.0};
    
    /// Blacklist match factor component
    double blacklist_factor{0.0};
    
    /// Biophysical stress factor component
    double biophysical_factor{0.0};
    
    /// Integrity factor component
    double integrity_factor{0.0};
    
    /// Access factor component
    double access_factor{0.0};
    
    /// Knowledge factor component
    double knowledge_factor{0.0};
    
    /// Discrimination factor component
    double discrimination_factor{0.0};
    
    /// Hash of triggering artifact (commit, OTA manifest, SKO ID)
    Hash64 triggering_artifact_hash{};
    
    /// List of jurisdiction tags (SB_1223, EU_AI_Act_Art5, etc.)
    std::vector<std::string> jurisdiction_tags{};
    
    /// List of neurorights violations detected
    std::vector<NeurorightsViolation> neurorights_violations{};
    
    /// Host DID signature for authenticity
    Signature64 host_did_signature{};
    
    /// Googolswarm anchor transaction ID
    std::string googolswarm_anchor_txid{};
    
    /// Event severity classification
    SabotageEventSeverity severity{SabotageEventSeverity::LOW};
    
    /// Defensive action taken
    SabotageAction action_taken{SabotageAction::LOG_ONLY};
    
    /// Serialize to JSON for logging/anchoring
    ///
    /// @return JSON string representation
    [[nodiscard]] auto to_json() const -> std::string;
    
    /// Verify event signature
    ///
    /// @param public_key Host public key
    /// @return true if signature is valid
    [[nodiscard]] auto verify_signature(const PublicKey32& public_key) const -> bool;
    
    /// Get severity as string
    ///
    /// @return Severity name string
    [[nodiscard]] auto severity_string() const -> std::string_view {
        switch (severity) {
            case SabotageEventSeverity::LOW: return "LOW";
            case SabotageEventSeverity::MEDIUM: return "MEDIUM";
            case SabotageEventSeverity::HIGH: return "HIGH";
            case SabotageEventSeverity::CRITICAL: return "CRITICAL";
        }
        return "UNKNOWN";
    }
    
    /// Get action as string
    ///
    /// @return Action name string
    [[nodiscard]] auto action_string() const -> std::string_view {
        switch (action_taken) {
            case SabotageAction::LOG_ONLY: return "LOG_ONLY";
            case SabotageAction::DENY: return "DENY";
            case SabotageAction::UNSAFE_DEFER: return "UNSAFE_DEFER";
            case SabotageAction::HARD_FAIL: return "HARD_FAIL";
            case SabotageAction::INHIBIT: return "INHIBIT";
        }
        return "UNKNOWN";
    }
};

/// Blacklist pattern entry (from ALN shard)
struct SabotageDetectorBlacklistPattern {
    std::string metric{};
    std::string domain{};
    std::string module{};
    PatternOp op{PatternOp::EQUALS};
    std::string pattern{};
    std::string kind{};
    std::string reason{};
    std::string source{};
    
    /// Check if identifier matches this pattern
    [[nodiscard]] auto matches(const std::string& identifier) const -> bool;
};

/// Sabotage risk rule (from ALN shard)
struct SabotageDetectorRiskRule {
    std::string rule_id{};
    std::string component{};
    double weight{0.0};
    std::string description{};
};

/// Threshold configuration (from ALN shard)
struct SabotageDetectorThreshold {
    std::string threshold_id{};
    std::string scalar{};
    double value{0.0};
    SabotageAction action{SabotageAction::LOG_ONLY};
    std::string log_event{};
};

// =============================================================================
// EXCEPTION CLASSES
// =============================================================================

/// Base exception for all SabotageDetector errors
class SabotageDetectorException : public std::runtime_error {
public:
    explicit SabotageDetectorException(const std::string& msg)
        : std::runtime_error(msg) {}
    
    explicit SabotageDetectorException(const char* msg)
        : std::runtime_error(msg) {}
};

/// Exception thrown on monotonicity violation
///
/// Indicates an evolution proposal would decrease Knowledge Factor or
/// increase Risk of Harm, violating the monotonic evolution constraint.
class MonotonicityViolationException : public SabotageDetectorException {
public:
    explicit MonotonicityViolationException(const std::string& msg)
        : SabotageDetectorException("MONOTONICITY_VIOLATION: " + msg) {}
};

/// Exception thrown on discrimination detection
///
/// Indicates an evolution proposal contains protected attribute references
/// or proxy discrimination patterns.
class DiscriminationDetectedException : public SabotageDetectorException {
public:
    explicit DiscriminationDetectedException(const std::string& msg)
        : SabotageDetectorException("DISCRIMINATION_DETECTED: " + msg) {}
};

/// Exception thrown on sabotage detection
///
/// Indicates a sabotage attempt was detected and the proposal was rejected.
class SabotageDetectedException : public SabotageDetectorException {
public:
    SabotageDetectedException(SabotageAction action, double risk)
        : SabotageDetectorException(
            "SABOTAGE_DETECTED_" + action_to_string(action) + 
            ": risk=" + std::to_string(risk))
        , action_(action)
        , risk_(risk) {}
    
    [[nodiscard]] auto action() const noexcept -> SabotageAction {
        return action_;
    }
    
    [[nodiscard]] auto risk() const noexcept -> double {
        return risk_;
    }
    
private:
    SabotageAction action_;
    double risk_;
    
    static auto action_to_string(SabotageAction action) -> std::string {
        switch (action) {
            case SabotageAction::DENY: return "DENY";
            case SabotageAction::UNSAFE_DEFER: return "UNSAFE_DEFER";
            case SabotageAction::HARD_FAIL: return "HARD_FAIL";
            case SabotageAction::INHIBIT: return "INHIBIT";
            default: return "UNKNOWN";
        }
    }
};

/// Exception thrown on ALN shard load failure
class AlnShardLoadException : public SabotageDetectorException {
public:
    explicit AlnShardLoadException(const std::string& path, const std::string& reason)
        : SabotageDetectorException(
            "ALN_SHARD_LOAD_FAILED: path=" + path + ", reason=" + reason) {}
};

/// Exception thrown on Googolswarm anchor failure
class GoogolswarmAnchorException : public SabotageDetectorException {
public:
    explicit GoogolswarmAnchorException(const std::string& reason)
        : SabotageDetectorException("GOOGOLSWARM_ANCHOR_FAILED: " + reason) {}
};

// =============================================================================
// MAIN DETECTOR CLASS — PUBLIC API
// =============================================================================

/// SabotageDetector — Main detection and evidence generation engine
///
/// This class provides the primary interface for sabotage detection, evolution
/// proposal validation, and court-admissible event generation. It integrates
/// real-time telemetry fusion, blacklist pattern matching, ghost-access graph
/// analysis, and anti-discrimination safeguards.
///
/// THREAD SAFETY: This class is thread-safe. All public methods can be called
/// from multiple threads concurrently. Internal state is protected by mutexes
/// and atomic operations.
///
/// EXAMPLE USAGE:
/// @code
/// SabotageDetector::Config config;
/// config.host_did = "did:example:host";
/// config.host_secret_key = load_secret_key();
/// config.host_public_key = load_public_key();
///
/// auto detector = SabotageDetector::create(config);
/// detector->start();
///
/// EvolutionProposal proposal = create_proposal();
/// BiophysicalTelemetry telemetry = capture_telemetry();
///
/// try {
///     detector->validate_proposal(proposal, telemetry);
///     // Proposal approved
/// } catch (const SabotageDetectorException& e) {
///     // Proposal rejected — check e.what() for reason
/// }
///
/// detector->stop();
/// @endcode
class SabotageDetector {
public:
    /// Configuration structure
    ///
    /// All configuration options for the SabotageDetector. Required fields
    /// must be set before calling create().
    struct Config {
        /// Path to ALN blacklist shard
        std::string aln_shard_path{std::string(DEFAULT_ALN_SHARD_PATH)};
        
        /// Path to QPU.Datashard storage
        std::string qpu_datashard_path{std::string(DEFAULT_QPU_DATASHARD_PATH)};
        
        /// Path to DonutLoop audit log
        std::string donutloop_path{std::string(DEFAULT_DONUTLOOP_PATH)};
        
        /// Host DID identifier
        std::string host_did{};
        
        /// Host secret key (Ed25519)
        SecretKey64 host_secret_key{};
        
        /// Host public key (Ed25519)
        PublicKey32 host_public_key{};
        
        /// Googolswarm endpoint URL
        std::string googolswarm_endpoint{};
        
        /// Event buffer size
        std::size_t event_buffer_size{DEFAULT_EVENT_BUFFER_SIZE};
        
        /// Telemetry polling interval
        Milliseconds telemetry_poll_interval{DEFAULT_TELEMETRY_POLL_INTERVAL};
        
        /// Validate configuration
        ///
        /// @return true if all required fields are set
        [[nodiscard]] auto validate() const noexcept -> bool {
            return !host_did.empty() &&
                   !googolswarm_endpoint.empty();
        }
    };
    
    /// Create a new SabotageDetector instance
    ///
    /// Factory function that creates and initializes a SabotageDetector with
    /// the given configuration. Throws SabotageDetectorException on failure.
    ///
    /// @param config Configuration options
    /// @return Unique pointer to SabotageDetector
    /// @throws SabotageDetectorException on initialization failure
    [[nodiscard]] static auto create(const Config& config)
        -> std::unique_ptr<SabotageDetector>;
    
    /// Destructor
    virtual ~SabotageDetector();
    
    /// Start the detector (background telemetry polling)
    ///
    /// Begins background telemetry polling and continuous coercion detection.
    /// Safe to call multiple times (idempotent).
    void start();
    
    /// Stop the detector
    ///
    /// Stops background telemetry polling and waits for all pending operations
    /// to complete. Safe to call multiple times (idempotent).
    void stop();
    
    /// Evaluate an evolution proposal
    ///
    /// Evaluates a proposal against all sabotage risk factors without
    /// throwing exceptions. Returns risk scalar and recommended action.
    ///
    /// @param proposal Evolution proposal to evaluate
    /// @param telemetry Current biophysical telemetry snapshot
    /// @return Pair of (risk_scalar, recommended_action)
    [[nodiscard]] virtual auto evaluate_proposal(
        const SabotageDetectorEvolutionProposal& proposal,
        const SabotageDetectorBiophysicalTelemetry& telemetry
    ) -> std::pair<SabotageDetectorRiskScalar, SabotageAction> = 0;
    
    /// Validate proposal (throws on rejection)
    ///
    /// Validates a proposal and throws an exception if it fails any safety
    /// check (monotonicity, discrimination, sabotage risk). Use this method
    /// when you want automatic rejection handling.
    ///
    /// @param proposal Evolution proposal to validate
    /// @param telemetry Current biophysical telemetry snapshot
    /// @throws MonotonicityViolationException on KF/RoH violation
    /// @throws DiscriminationDetectedException on discrimination detection
    /// @throws SabotageDetectedException on sabotage detection
    virtual void validate_proposal(
        const SabotageDetectorEvolutionProposal& proposal,
        const SabotageDetectorBiophysicalTelemetry& telemetry
    ) = 0;
    
    /// Generate a SABOTAGEEVENT
    ///
    /// Creates and logs a sabotage event with full court-admissible metadata.
    /// The event is written to QPU.Datashard and anchored to Googolswarm.
    ///
    /// @param risk_scalar Computed risk scalar
    /// @param action Defensive action taken
    /// @param proposal Triggering proposal
    /// @return Generated event
    [[nodiscard]] virtual auto generate_sabotage_event(
        const SabotageDetectorRiskScalar& risk_scalar,
        SabotageAction action,
        const SabotageDetectorEvolutionProposal& proposal
    ) -> SabotageDetectorEvent = 0;
    
    /// Get current event ID counter
    ///
    /// @return Current event ID (next event will be this value + 1)
    [[nodiscard]] virtual auto get_event_id_counter() const noexcept -> EventId = 0;
    
    /// Check if detector is running
    ///
    /// @return true if background telemetry polling is active
    [[nodiscard]] virtual auto is_running() const noexcept -> bool = 0;
    
    /// Get detector statistics
    ///
    /// @return Map of statistic name to value
    [[nodiscard]] virtual auto get_statistics() const
        -> std::unordered_map<std::string, std::uint64_t> = 0;
    
    /// Disable copy operations
    SabotageDetector(const SabotageDetector&) = delete;
    SabotageDetector& operator=(const SabotageDetector&) = delete;
    
    /// Enable move operations
    SabotageDetector(SabotageDetector&&) noexcept = default;
    SabotageDetector& operator=(SabotageDetector&&) noexcept = default;

protected:
    /// Protected default constructor
    SabotageDetector() = default;
};

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

/// Convert severity to string
///
/// @param severity Severity enum value
/// @return String representation
[[nodiscard]] auto severity_to_string(SabotageEventSeverity severity)
    -> std::string_view;

/// Convert action to string
///
/// @param action Action enum value
/// @return String representation
[[nodiscard]] auto action_to_string(SabotageAction action)
    -> std::string_view;

/// Convert neurorights violation to string
///
/// @param violation Violation enum value
/// @return String representation
[[nodiscard]] auto violation_to_string(NeurorightsViolation violation)
    -> std::string_view;

/// Convert scope to string
///
/// @param scope Scope enum value
/// @return String representation
[[nodiscard]] auto scope_to_string(EvolutionScope scope)
    -> std::string_view;

/// Hash a string to Hash64
///
/// @param data String to hash
/// @return 64-byte hash
[[nodiscard]] auto hash_to_64(const std::string& data) -> Hash64;

/// Sign data with Ed25519
///
/// @param data Data to sign
/// @param secret_key Secret key
/// @return 64-byte signature
[[nodiscard]] auto sign_ed25519(const std::vector<std::uint8_t>& data,
                                 const SecretKey64& secret_key) -> Signature64;

/// Verify Ed25519 signature
///
/// @param data Signed data
/// @param signature Signature to verify
/// @param public_key Public key
/// @return true if signature is valid
[[nodiscard]] auto verify_ed25519(const std::vector<std::uint8_t>& data,
                                   const Signature64& signature,
                                   const PublicKey32& public_key) -> bool;

/// Convert timestamp to ISO 8601 string
///
/// @param timestamp Timestamp to convert
/// @return ISO 8601 formatted string
[[nodiscard]] auto timestamp_to_iso8601(Timestamp timestamp) -> std::string;

/// Parse ISO 8601 string to timestamp
///
/// @param iso_string ISO 8601 formatted string
/// @return Timestamp
[[nodiscard]] auto iso8601_to_timestamp(const std::string& iso_string) -> Timestamp;

// =============================================================================
// VERSION INFORMATION
// =============================================================================

/// Library major version
constexpr int SABOTAGE_DETECTOR_VERSION_MAJOR = 1;

/// Library minor version
constexpr int SABOTAGE_DETECTOR_VERSION_MINOR = 0;

/// Library patch version
constexpr int SABOTAGE_DETECTOR_VERSION_PATCH = 0;

/// Library version string
constexpr std::string_view SABOTAGE_DETECTOR_VERSION = "1.0.0";

/// Get version string
///
/// @return Version string in format "major.minor.patch"
[[nodiscard]] inline auto get_version_string() -> std::string {
    return std::to_string(SABOTAGE_DETECTOR_VERSION_MAJOR) + "." +
           std::to_string(SABOTAGE_DETECTOR_VERSION_MINOR) + "." +
           std::to_string(SABOTAGE_DETECTOR_VERSION_PATCH);
}

/// Check if runtime version matches compile-time version
///
/// @return true if versions match
[[nodiscard]] inline auto verify_version() -> bool {
    return (SABOTAGE_DETECTOR_VERSION_MAJOR == 1) &&
           (SABOTAGE_DETECTOR_VERSION_MINOR == 0) &&
           (SABOTAGE_DETECTOR_VERSION_PATCH == 0);
}

} // namespace reality::os::sovereignty::telemetry

// =============================================================================
// STREAM OUTPUT OPERATORS (for debugging and logging)
// =============================================================================

namespace reality::os::sovereignty::telemetry {

/// Output operator for SabotageEventSeverity
inline auto operator<<(std::ostream& os, SabotageEventSeverity severity)
    -> std::ostream& {
    return os << severity_to_string(severity);
}

/// Output operator for SabotageAction
inline auto operator<<(std::ostream& os, SabotageAction action)
    -> std::ostream& {
    return os << action_to_string(action);
}

/// Output operator for NeurorightsViolation
inline auto operator<<(std::ostream& os, NeurorightsViolation violation)
    -> std::ostream& {
    return os << violation_to_string(violation);
}

/// Output operator for EvolutionScope
inline auto operator<<(std::ostream& os, EvolutionScope scope)
    -> std::ostream& {
    return os << scope_to_string(scope);
}

} // namespace reality::os::sovereignty::telemetry

// =============================================================================
// END OF HEADER
// =============================================================================

#endif // REALITY_OS_SOVEREIGNTY_TELEMETRY_SABOTAGE_DETECTOR_H
