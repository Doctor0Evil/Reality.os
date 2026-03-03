import React, { createContext, useContext, useState, ReactNode } from 'react';
import { AllowedRpcMethod } from '../types/aln/rpc_session_envelope';
import { EcoImpactScore } from '../types/aln/eco_metrics_host_budget';

interface SovereigntyCoreContextType {
  queryEndpointRegistry: (filters: any) => Promise<any>;
  requestRpcActivation: (request: any) => Promise<any>;
  anchorToROW: (params: any) => Promise<any>;
  queryActiveSession: (citizenDid: string) => Promise<any>;
  evaluateCorridor: (request: any) => Promise<any>;
  queryEcoMetrics: (did: string) => Promise<any>;
  queryHostBudget: (did: string) => Promise<any>;
  queryNeuroChannels: (did: string) => Promise<any>;
  queryHealthCorridors: (did: string) => Promise<any>;
  broadcastEmergencyStop: (params: any) => Promise<any>;
  startNeuroChannelCalibration: (channelId: string) => Promise<void>;
  toggleChannelSharing: (channelId: string, enabled: boolean) => Promise<void>;
  requestCognitiveLoadChange: (params: any) => Promise<void>;
  queryValidatorSet: () => Promise<any>;
  queryMissionContributions: (did: string) => Promise<any>;
}

const SovereigntyCoreContext = createContext<SovereigntyCoreContextType | undefined>(undefined);

export const SovereigntyCoreProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [isInitialized, setIsInitialized] = useState(false);

  // Initialize SovereigntyCore connection
  useEffect(() => {
    const init = async () => {
      try {
        // Check if SovereigntyCore is available via window object
        if (window.sovereigntyCore) {
          setIsInitialized(true);
        } else {
          // Mock implementation for development
          console.warn('SovereigntyCore not available, using mock implementation');
        }
      } catch (err) {
        console.error('Failed to initialize SovereigntyCore:', err);
      }
    };

    init();
  }, []);

  const queryEndpointRegistry = async (filters: any) => {
    // Query CozoDB for cached endpoint registry
    const result = await window.cozodb?.execute(`
      ?[value] := *config{key: 'endpoint_registry', group_key: 'network', value: value}
      :order value.timestamp desc
      :limit 1
    `);
    return result?.results?.[0]?.[0] ? JSON.parse(result.results[0][0]) : { entries: [] };
  };

  const requestRpcActivation = async (request: any) => {
    // Evaluate against ALN corridors and RoH envelopes
    const evaluation = await evaluateCorridor({
      method: 'RPC_ACTIVATION',
      biophysicalData: request,
    });

    if (!evaluation.allowed) {
      return { approved: false, denialReason: evaluation.reason };
    }

    // Create RpcSessionEnvelope
    const envelope = {
      session_id: crypto.randomUUID(),
      citizen_did: request.citizenDid,
      allowed_methods: request.allowedMethods,
      energy_budget_mj: request.energyBudgetMj,
      privacy_level: request.privacyLevel,
      start_time: new Date().toISOString(),
      expiry_time: new Date(Date.now() + request.sessionDurationMinutes * 60000).toISOString(),
    };

    return { approved: true, sessionId: envelope.session_id, envelope };
  };

  const anchorToROW = async (params: any) => {
    // Anchor to ALN ROW ledger via SovereigntyCore
    const result = await window.sovereigntyCore?.anchorToROW(params);
    
    // Also store in CozoDB transaction table for local audit
    await window.cozodb?.execute(`
      ?[hash, index, neuron, type, block_height, success, timestamp, value, memo] <- [
        [${JSON.stringify(params.sessionId)}, 0, ${JSON.stringify(params.envelope.citizen_did || 'local')}, 'row_anchor', 0, true, ${Date.now()}, ${JSON.stringify(params.envelope)}, 'ROW anchor']
      ]
      :insert transaction
    `);

    return result || { transactionHash: crypto.randomUUID(), blockHeight: 0 };
  };

  const queryActiveSession = async (citizenDid: string) => {
    const result = await window.cozodb?.execute(`
      ?[value] := *config{key: 'active_rpc_session', group_key: 'sessions', value: value}
      value.citizen_did == ${JSON.stringify(citizenDid)}
      :order value.timestamp desc
      :limit 1
    `);
    return result?.results?.[0]?.[0] ? JSON.parse(result.results[0][0]) : null;
  };

  const evaluateCorridor = async (request: any) => {
    // Mock corridor evaluation (integrate with actual ALN logic)
    const allowed = true;
    return { allowed, reason: allowed ? undefined : 'Corridor evaluation failed' };
  };

  const queryEcoMetrics = async (did: string) => {
    const result = await window.cozodb?.execute(`
      ?[value] := *config{key: 'eco_metrics', group_key: 'citizen', value: value}
      value.citizen_did == ${JSON.stringify(did)}
      :order value.timestamp desc
      :limit 1
    `);
    return result?.results?.[0]?.[0] ? JSON.parse(result.results[0][0]) : null;
  };

  const queryHostBudget = async (did: string) => {
    const result = await window.cozodb?.execute(`
      ?[value] := *config{key: 'host_budget', group_key: 'citizen', value: value}
      value.citizen_did == ${JSON.stringify(did)}
      :order value.timestamp desc
      :limit 1
    `);
    return result?.results?.[0]?.[0] ? JSON.parse(result.results[0][0]) : null;
  };

  const queryNeuroChannels = async (did: string) => {
    const result = await window.cozodb?.execute(`
      ?[value] := *config{key: 'neuro_channels', group_key: 'citizen', value: value}
      value.citizen_did == ${JSON.stringify(did)}
      :order value.timestamp desc
      :limit 1
    `);
    return result?.results?.[0]?.[0] ? JSON.parse(result.results[0][0]).channels : [];
  };

  const queryHealthCorridors = async (did: string) => {
    // Mock health corridor data
    return [
      { name: 'Heart Rate', current: 72, min_safe: 50, max_safe: 100, unit: 'bpm', status: 'safe' },
      { name: 'EEG Stress', current: 0.3, min_safe: 0, max_safe: 0.7, unit: 'score', status: 'safe' },
      { name: 'HRV', current: 45, min_safe: 30, max_safe: 100, unit: 'ms', status: 'safe' },
    ];
  };

  const broadcastEmergencyStop = async (params: any) => {
    // Broadcast to all bound devices
    const envelope = {
      type: 'emergency_neuro_stop',
      citizen_did: params.citizenDid,
      reason: params.reason,
      timestamp: params.timestamp,
    };

    // Anchor to ROW ledger
    const anchorResult = await anchorToROW({
      sessionId: crypto.randomUUID(),
      envelope,
    });

    return { envelope, ...anchorResult };
  };

  const startNeuroChannelCalibration = async (channelId: string) => {
    // Update channel status in CozoDB
    await window.cozodb?.execute(`
      ?[key, group, value] <- [
        ['neuro_channels', 'citizen', ${JSON.stringify({ channel_id: channelId, status: 'calibrating', timestamp: Date.now() })}]
      ]
      :insert config
    `);
  };

  const toggleChannelSharing = async (channelId: string, enabled: boolean) => {
    // Update sharing status in CozoDB link table
    await window.cozodb?.execute(`
      ?[from, to, neuron, timestamp, transaction_hash] <- [
        ['${channelId}', '${enabled ? 'public' : 'private'}', 'local', ${Date.now()}, ${JSON.stringify(crypto.randomUUID())}]
      ]
      :insert link
    `);
  };

  const requestCognitiveLoadChange = async (params: any) => {
    // Update cognitive load config
    await window.cozodb?.execute(`
      ?[key, group, value] <- [
        ['cognitive_load', 'ux_preferences', ${JSON.stringify({ ...params, timestamp: Date.now() })}]
      ]
      :insert config
    `);
  };

  const queryValidatorSet = async () => {
    const result = await window.cozodb?.execute(`
      ?[value] := *config{key: 'validator_set', group_key: 'network', value: value}
      :order value.timestamp desc
      :limit 1
    `);
    return result?.results?.[0]?.[0] ? JSON.parse(result.results[0][0]) : { validators: [], total_validators: 0, active_validators: 0 };
  };

  const queryMissionContributions = async (did: string) => {
    const result = await window.cozodb?.execute(`
      ?[value] := *config{key: 'mission_contributions', group_key: 'citizen', value: value}
      value.citizen_did == ${JSON.stringify(did)}
      :order value.timestamp desc
    `);
    return result?.results?.map((r: any) => JSON.parse(r[0])) || [];
  };

  return (
    <SovereigntyCoreContext.Provider
      value={{
        queryEndpointRegistry,
        requestRpcActivation,
        anchorToROW,
        queryActiveSession,
        evaluateCorridor,
        queryEcoMetrics,
        queryHostBudget,
        queryNeuroChannels,
        queryHealthCorridors,
        broadcastEmergencyStop,
        startNeuroChannelCalibration,
        toggleChannelSharing,
        requestCognitiveLoadChange,
        queryValidatorSet,
        queryMissionContributions,
      }}
    >
      {children}
    </SovereigntyCoreContext.Provider>
  );
};

export const useSovereigntyCore = () => {
  const context = useContext(SovereigntyCoreContext);
  if (context === undefined) {
    throw new Error('useSovereigntyCore must be used within a SovereigntyCoreProvider');
  }
  return context;
};
