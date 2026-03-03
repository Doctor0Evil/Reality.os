import React, { useState, useEffect } from 'react';
import { useDID } from '../hooks/useDID';
import { useSovereigntyCore } from '../hooks/useSovereigntyCore';
import { EndpointRegistryEntry } from '../types/aln/endpoint_registry';
import { EcoImpactBadge } from './EcoImpactBadge';
import { RoHBadge } from './RoHBadge';

interface EndpointLocatorProps {
  jurisdiction: string;
  mode: 'clinical' | 'field' | 'research' | 'daily';
  healthRpcRequired: boolean;
  onEndpointSelected: (endpoint: EndpointRegistryEntry) => void;
}

export const EndpointLocator: React.FC<EndpointLocatorProps> = ({
  jurisdiction,
  mode,
  healthRpcRequired,
  onEndpointSelected,
}) => {
  const { did } = useDID();
  const sovereigntyCore = useSovereigntyCore();
  const [endpoints, setEndpoints] = useState<EndpointRegistryEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadEndpoints = async () => {
      try {
        setLoading(true);
        // Query EndpointRegistry shard via SovereigntyCore
        const registry = await sovereigntyCore.queryEndpointRegistry({
          jurisdiction,
          mode,
          healthRpcRequired,
        });
        setEndpoints(registry);
        setError(null);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load endpoints');
      } finally {
        setLoading(false);
      }
    };
    loadEndpoints();
  }, [jurisdiction, mode, healthRpcRequired, sovereigntyCore]);

  if (loading) {
    return (
      <div className="endpoint-locator loading">
        <div className="spinner" />
        <p>Resolving eco-audited endpoints for {jurisdiction}...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="endpoint-locator error">
        <p className="error-message">{error}</p>
        <button onClick={() => window.location.reload()}>Retry</button>
      </div>
    );
  }

  if (endpoints.length === 0) {
    return (
      <div className="endpoint-locator no-endpoints">
        <p>No endpoints meet the required eco and RoH floors for your profile.</p>
        <p className="hint">
          Required: EcoImpactScore ≥ {healthRpcRequired ? '0.86' : '0.70'}, 
          Risk-of-Harm ≤ {healthRpcRequired ? '0.3' : '0.5'}
        </p>
      </div>
    );
  }

  return (
    <div className="endpoint-locator">
      <h3>Available Organichain Endpoints</h3>
      <p className="subtitle">
        DID-verifiable, eco-audited RPC endpoints for {jurisdiction} ({mode} mode)
      </p>
      <div className="endpoint-list">
        {endpoints.map((endpoint) => (
          <div
            key={endpoint.endpoint_id}
            className="endpoint-card"
            onClick={() => onEndpointSelected(endpoint)}
            role="button"
            tabIndex={0}
          >
            <div className="endpoint-header">
              <h4>{endpoint.jurisdiction} Organichain RPC</h4>
              <span className="validator-did">{endpoint.validator_did}</span>
            </div>
            <div className="endpoint-metrics">
              <EcoImpactBadge score={endpoint.eco_impact_score.value} />
              <RoHBadge value={endpoint.risk_of_harm} />
            </div>
            <div className="endpoint-details">
              <p>
                <strong>RPC:</strong> {endpoint.rpc_url}
              </p>
              {endpoint.grpc_url && (
                <p>
                  <strong>gRPC:</strong> {endpoint.grpc_url}
                </p>
              )}
              {endpoint.websocket_url && (
                <p>
                  <strong>WebSocket:</strong> {endpoint.websocket_url}
                </p>
              )}
            </div>
            <div className="endpoint-modes">
              {endpoint.mode_tags.map((tag) => (
                <span key={tag} className="mode-tag">
                  {tag}
                </span>
              ))}
            </div>
            {endpoint.meets_health_rpc_floors() && (
              <div className="health-certified">
                ✓ Health RPC Certified (Eco ≥ 0.86, RoH ≤ 0.3)
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};
