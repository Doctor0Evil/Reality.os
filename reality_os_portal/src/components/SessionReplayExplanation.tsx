import React, { useState, useEffect } from 'react';
import { ProvenanceNode } from '../cyberspectre/CyberspectreIntrospectionEngine';

interface SessionReplayExplanationProps {
  sessionId?: string;
  limit?: number;
}

export const SessionReplayExplanation: React.FC<SessionReplayExplanationProps> = ({
  sessionId,
  limit = 50,
}) => {
  const [nodes, setNodes] = useState<ProvenanceNode[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadSession = async () => {
      try {
        setLoading(true);
        // Retrieve introspection records from local storage or CozoDB
        const stored = localStorage.getItem(`cyberspectre-session-${sessionId || 'default'}`);
        if (stored) {
          const parsed = JSON.parse(stored);
          setNodes(parsed.nodes?.slice(0, limit) || []);
        }
      } catch (err) {
        console.error('Failed to load session replay:', err);
      } finally {
        setLoading(false);
      }
    };

    loadSession();
  }, [sessionId, limit]);

  if (loading) {
    return <div className="session-replay loading">Loading session replay...</div>;
  }

  if (nodes.length === 0) {
    return (
      <div className="session-replay empty">
        <p>No provenance nodes recorded for this session.</p>
      </div>
    );
  }

  return (
    <div className="session-replay">
      <h3>Session Replay & Explanation</h3>
      <p className="subtitle">
        Cyberspectre introspection trace: all actions, origins, and ALN anchors
      </p>

      <div className="node-timeline">
        {nodes.map((node, index) => (
          <div key={node.nonce} className="timeline-node">
            <div className="node-header">
              <span className="node-index">#{nodes.length - index}</span>
              <span className="node-timestamp">
                {new Date(node.timestampIso).toLocaleTimeString()}
              </span>
              <span className="node-action">{node.action.title}</span>
            </div>
            
            <div className="node-details">
              <div className="detail-row">
                <strong>DID:</strong> <span className="mono">{node.did}</span>
              </div>
              <div className="detail-row">
                <strong>Layer:</strong> {node.action.layer}
              </div>
              <div className="detail-row">
                <strong>ALN Capability:</strong> <span className="mono">{node.action.alnCapability}</span>
              </div>
              <div className="detail-row">
                <strong>Origin:</strong> {node.origin.file}:{node.origin.lineStart}
              </div>
              <div className="detail-row">
                <strong>Payload:</strong> {node.payloadSummary}
              </div>
            </div>

            <div className="node-proof">
              <span className="proof-label">Forward-Only:</span>
              <span className="proof-value">{node.action.forwardOnly ? '✓ True' : '✗ False'}</span>
            </div>
          </div>
        ))}
      </div>

      <div className="replay-info">
        <p className="info-text">
          <strong>Audit Trail:</strong> These nodes are anchored to the ALN ROW ledger 
          and can be verified via Merkle proofs.
        </p>
      </div>
    </div>
  );
};
