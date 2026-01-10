/**
 * @typedef {Object} EpochRow
 * @property {number} epoch_id
 * @property {string} subject_id
 * @property {string} timestamp           // ISO8601
 * @property {"Wake"|"N1"|"N2"|"N3"|"Rem"} sleep_stage
 * @property {number} excavation_priority
 * @property {number} lucidity_likelihood
 * @property {number} psychrisk_score
 * @property {number} eligibility_e
 * @property {boolean} contains_inner_speech
 * @property {boolean} contains_visual_persons
 * @property {boolean} contains_biographical_memory
 */

/**
 * @typedef {Object} LedgerRow
 * @property {number} id
 * @property {string} timestamp           // ISO8601
 * @property {string} subject_id
 * @property {string} event
 * @property {boolean} person_scoring_applied
 * @property {string|null} message
 */

/**
 * @typedef {Object} NeurorightsPolicy
 * @property {boolean} mental_privacy
 * @property {boolean} cognitive_liberty
 * @property {boolean} mental_integrity
 * @property {boolean} non_commercial_neural
 * @property {boolean} soul_non_addressable
 */

/**
 * @typedef {Object} InvariantCheck
 * @property {string} name
 * @property {boolean} ok
 * @property {string|null} message
 */

/**
 * @typedef {Object} InvariantBatchResult
 * @property {string} batch_id
 * @property {boolean} all_ok
 * @property {InvariantCheck[]} checks
 * @property {number} epoch_count
 * @property {number} ledger_count
 * @property {string} evaluated_at        // ISO8601
 */

/**
 * Bridge into the Rust invariant checker.
 * Expected server-side API:
 *   POST /invariants/check  { epochs, ledger, policy } -> { checks: InvariantCheck[] }
 *
 * @param {EpochRow[]} epochs
 * @param {LedgerRow[]} ledger
 * @param {NeurorightsPolicy} policy
 * @returns {Promise<InvariantBatchResult>}
 */
async function runInvariantBatch(epochs, ledger, policy) {
  const batchId = crypto.randomUUID();
  const res = await fetch("http://127.0.0.1:8088/invariants/check", {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify({ epochs, ledger, policy })
  });
  if (!res.ok) {
    throw new Error(`Invariant check HTTP error: ${res.status}`);
  }
  const data = await res.json();
  const checks = Array.isArray(data.checks) ? data.checks : [];
  const allOk = checks.every(c => c && c.ok === true);
  return {
    batch_id: batchId,
    all_ok: allOk,
    checks,
    epoch_count: epochs.length,
    ledger_count: ledger.length,
    evaluated_at: new Date().toISOString()
  };
}

/**
 * Decide whether to suppress or proceed with a given batch.
 *
 * @param {InvariantBatchResult} result
 */
function decideFromInvariants(result) {
  const failing = result.checks.filter(c => !c.ok);
  return {
    batch_id: result.batch_id,
    decision: result.all_ok ? "proceed" : "suppress",
    failing_names: failing.map(c => c.name),
    evaluated_at: result.evaluated_at
  };
}

/**
 * Example wiring for a stream-emit path.
 *
 * @param {EpochRow[]} epochs
 * @param {LedgerRow[]} ledger
 * @param {NeurorightsPolicy} policy
 * @param {(epochs: EpochRow[]) => Promise<void>} emitSafeStream
 * @param {(record: any) => Promise<void>} writeAuditLog
 */
async function processBatchWithInvariants(
  epochs,
  ledger,
  policy,
  emitSafeStream,
  writeAuditLog
) {
  const result = await runInvariantBatch(epochs, ledger, policy);
  const decision = decideFromInvariants(result);

  const auditRecord = {
    kind: "invariant_batch_evaluated",
    batch_id: decision.batch_id,
    decision: decision.decision,
    failing_names: decision.failing_names,
    epoch_count: result.epoch_count,
    ledger_count: result.ledger_count,
    evaluated_at: result.evaluated_at
  };
  await writeAuditLog(auditRecord);

  if (decision.decision === "proceed") {
    await emitSafeStream(epochs);
  } else {
    // Suppression semantics: nothing is streamed; only the audit log records the failure.
  }
}

export {
  runInvariantBatch,
  decideFromInvariants,
  processBatchWithInvariants
};
