import React, { useState } from 'react';
import { useDID } from '../hooks/useDID';
import { useSovereigntyCore } from '../hooks/useSovereigntyCore';
import { CyberspectreIntrospectionEngine } from '../cyberspectre/CyberspectreIntrospectionEngine';
import { AllowedRpcMethod } from '../types/aln/rpc_session_envelope';

interface EmergencyNeuroStopProps {
  onActivation?: () => void;
  onError?: (error: Error) => void;
}

export const EmergencyNeuroStop: React.FC<EmergencyNeuroStopProps> = ({
  onActivation,
  onError,
}) => {
  const { did } = useDID();
  const sovereigntyCore = useSovereigntyCore();
  const [activating, setActivating] = useState(false);
  const [showConfirm, setShowConfirm] = useState(false);
  const [introspectionEngine] = useState(
    () => new CyberspectreIntrospectionEngine(`emergency-stop-${Date.now()}`)
  );

  const handleEmergencyStop = async () => {
    if (!did) return;
    setActivating(true);
    setShowConfirm(false);

    try {
      // Record this critical action in Cyberspectre
      const provenanceNode = introspectionEngine.recordNode({
        nonce: crypto.randomUUID(),
        did: did,
        action: {
          id: 'portal.btn.emergency_neuro_stop',
          title: 'Emergency NeuroStop Activated',
          layer: 'BIOPHYSICAL',
          alnCapability: 'aln.tx.emergency_neuro_stop',
          forwardOnly: true,
        },
        timestampIso: new Date().toISOString(),
        origin: {
          lang: 'TypeScript',
          file: 'EmergencyNeuroStop.tsx',
          lineStart: 35,
          colStart: 0,
          lineEnd: 75,
          colEnd: 0,
          authorDid: did,
          symbolId: 'handleEmergencyStop',
        },
        payloadSummary: 'Emergency stop triggered for all bound devices',
      });

      // Broadcast EmergencyNeuroStop envelope to all bound devices
      // This bypasses normal rate limits and energy budgets
      const result = await sovereigntyCore.broadcastEmergencyStop({
        citizenDid: did,
        reason: 'User initiated emergency stop',
        timestamp: Date.now(),
      });

      // Anchor to ROW ledger immediately
      await sovereigntyCore.anchorToROW({
        sessionId: provenanceNode.nonce,
        envelope: result.envelope,
      });

      // Log to local CozoDB 'transaction' table for offline audit
      await window.cozodb?.execute(`
        ?[hash, index, neuron, type, block_height, success, timestamp, value, memo] <- [
          [${JSON.stringify(result.transaction_hash)}, 0, ${JSON.stringify(did)}, 'emergency_stop', ${result.block_height}, true, ${Date.now()}, {}, 'Emergency NeuroStop Activated']
        ]
        :insert transaction
      `);

      onActivation?.();
    } catch (err) {
      const error = err instanceof Error ? err : new Error('Emergency stop failed');
      onError?.(error);
      console.error('Emergency Stop Error:', error);
    } finally {
      setActivating(false);
    }
  };

  return (
    <>
      <button
        onClick={() => setShowConfirm(true)}
        disabled={activating}
        className="btn-emergency-stop"
        aria-label="Emergency NeuroStop"
      >
        {activating ? 'STOPPING...' : '⚠ EMERGENCY STOP'}
      </button>

      {showConfirm && (
        <div className="modal-overlay">
          <div className="modal-confirm-emergency">
            <h3>⚠ Confirm Emergency Stop</h3>
            <p>
              This will immediately halt all stimulation and actuation 
              across all devices bound to your DID.
            </p>
            <p className="warning-text">
              This action is irreversible and will be anchored to the 
              ROW ledger as a critical safety event.
            </p>
            <div className="modal-actions">
              <button
                onClick={() => setShowConfirm(false)}
                className="btn-cancel"
              >
                Cancel
              </button>
              <button
                onClick={handleEmergencyStop}
                className="btn-confirm-emergency"
              >
                CONFIRM STOP
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
};
