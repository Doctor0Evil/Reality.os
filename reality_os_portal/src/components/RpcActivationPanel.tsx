import React, { useState } from 'react';
import { useDID } from '../hooks/useDID';
import { useSovereigntyCore } from '../hooks/useSovereigntyCore';
import { EndpointRegistryEntry } from '../types/aln/endpoint_registry';
import { AllowedRpcMethod, PrivacyLevel } from '../types/aln/rpc_session_envelope';
import { CyberspectreIntrospectionEngine } from '../cyberspectre/CyberspectreIntrospectionEngine';

interface RpcActivationPanelProps {
  endpoint: EndpointRegistryEntry;
  requestedMethods: AllowedRpcMethod[];
  energyBudgetMj: number;
  privacyLevel: PrivacyLevel;
  sessionDurationMinutes: number;
  onActivationComplete: (sessionId: string) => void;
  onActivationDenied: (reason: string) => void;
}

export const RpcActivationPanel: React.FC<RpcActivationPanelProps> = ({
  endpoint,
  requestedMethods,
  energyBudgetMj,
  privacyLevel,
  sessionDurationMinutes,
  onActivationComplete,
  onActivationDenied,
}) => {
  const { did } = useDID();
  const sovereigntyCore = useSovereigntyCore();
  const [activating, setActivating] = useState(false);
  const [introspectionEngine] = useState(
    () => new CyberspectreIntrospectionEngine(`rpc-activation-${Date.now()}`)
  );

  const handleActivate = async () => {
    setActivating(true);
    try {
      // Record this action in Cyberspectre for audit trail
      const provenanceNode = introspectionEngine.recordNode({
        nonce: crypto.randomUUID(),
        did: did!,
        action: {
          id: 'portal.btn.rpc-activation',
          title: 'RPC Activation Request',
          layer: 'BIOPHYSICAL',
          alnCapability: 'aln.tx.rpc_session_envelope',
          forwardOnly: true,
        },
        timestampIso: new Date().toISOString(),
        origin: {
          lang: 'TypeScript',
          file: 'RpcActivationPanel.tsx',
          lineStart: 45,
          colStart: 0,
          lineEnd: 80,
          colEnd: 0,
          authorDid: did!,
          symbolId: 'handleActivate',
        },
        payloadSummary: `RPC activation for endpoint ${endpoint.endpoint_id}`,
      });

      // Request SovereigntyCore to create RpcSessionEnvelope
      const result = await sovereigntyCore.requestRpcActivation({
        citizenDid: did!,
        endpointId: endpoint.endpoint_id,
        allowedMethods: requestedMethods,
        energyBudgetMj,
        privacyLevel,
        sessionDurationMinutes,
      });

      if (result.approved) {
        // Anchor to ROW ledger
        await sovereigntyCore.anchorToRow({
          sessionId: result.sessionId,
          envelope: result.envelope,
        });
        onActivationComplete(result.sessionId);
      } else {
        // Log denial reason
        onActivationDenied(result.denialReason || 'Unknown reason');
      }
    } catch (err) {
      onActivationDenied(err instanceof Error ? err.message : 'Activation failed');
    } finally {
      setActivating(false);
    }
  };

  const handleDeny = async () => {
    setActivating(true);
    try {
      // Record denial in Cyberspectre
      introspectionEngine.recordNode({
        nonce: crypto.randomUUID(),
        did: did!,
        action: {
          id: 'portal.btn.rpc-activation-denied',
          title: 'RPC Activation Denied by User',
          layer: 'BIOPHYSICAL',
          alnCapability: 'aln.tx.rpc_session_denial',
          forwardOnly: true,
        },
        timestampIso: new Date().toISOString(),
        origin: {
          lang: 'TypeScript',
          file: 'RpcActivationPanel.tsx',
          lineStart: 85,
          colStart: 0,
          lineEnd: 100,
          colEnd: 0,
          authorDid: did!,
          symbolId: 'handleDeny',
        },
        payloadSummary: `User denied RPC activation for endpoint ${endpoint.endpoint_id}`,
      });

      onActivationDenied('User denied activation');
    } finally {
      setActivating(false);
    }
  };

  return (
    <div className="rpc-activation-panel">
      <h3>RPC Activation Request</h3>
      <div className="activation-details">
        <p>
          <strong>Endpoint:</strong> {endpoint.jurisdiction} Organichain RPC
        </p>
        <p>
          <strong>Validator DID:</strong> {endpoint.validator_did}
        </p>
        <p>
          <strong>Requested Methods:</strong>{' '}
          {requestedMethods.join(', ')}
        </p>
        <p>
          <strong>Energy Budget:</strong> {energyBudgetMj} mJ
        </p>
        <p>
          <strong>Privacy Level:</strong> {privacyLevel}
        </p>
        <p>
          <strong>Session Duration:</strong> {sessionDurationMinutes} minutes
        </p>
        <div className="eco-roh-check">
          <p>
            <strong>Eco Impact Score:</strong> {endpoint.eco_impact_score.value} 
            {endpoint.eco_impact_score.value >= 0.86 ? ' ✓' : ' ✗'}
          </p>
          <p>
            <strong>Risk of Harm:</strong> {endpoint.risk_of_harm} 
            {endpoint.risk_of_harm <= 0.3 ? ' ✓' : ' ✗'}
          </p>
        </div>
      </div>
      <div className="activation-actions">
        <button
          onClick={handleActivate}
          disabled={activating}
          className="btn-activate"
        >
          {activating ? 'Activating...' : 'Activate RPC Session'}
        </button>
        <button
          onClick={handleDeny}
          disabled={activating}
          className="btn-deny"
        >
          Deny
        </button>
      </div>
      <p className="forward-only-notice">
        ⚠ This is a forward-only, DID-signed decision. Once activated, 
        this session cannot be rolled back or modified. Changes require 
        a new RpcSessionEnvelope.
      </p>
    </div>
  );
};
