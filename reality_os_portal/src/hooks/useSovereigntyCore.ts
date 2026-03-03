import { useCallback } from 'react';
import { EndpointRegistryEntry } from '../types/aln/endpoint_registry';
import { 
  RpcSessionEnvelope, 
  AllowedRpcMethod, 
  PrivacyLevel 
} from '../types/aln/rpc_session_envelope';

export interface SovereigntyCoreApi {
  queryEndpointRegistry: (filters: {
    jurisdiction: string;
    mode: string;
    healthRpcRequired: boolean;
  }) => Promise<EndpointRegistryEntry[]>;
  
  requestRpcActivation: (request: {
    citizenDid: string;
    endpointId: string;
    allowedMethods: AllowedRpcMethod[];
    energyBudgetMj: number;
    privacyLevel: PrivacyLevel;
    sessionDurationMinutes: number;
  }) => Promise<{
    approved: boolean;
    sessionId?: string;
    envelope?: RpcSessionEnvelope;
    denialReason?: string;
  }>;
  
  anchorToRow: (params: {
    sessionId: string;
    envelope: RpcSessionEnvelope;
  }) => Promise<string>; // Returns ROW anchor height
  
  queryActiveSession: (citizenDid: string) => Promise<RpcSessionEnvelope | null>;
  
  evaluateCorridor: (request: {
    method: AllowedRpcMethod;
    biophysicalData?: unknown;
  }) => Promise<{ allowed: boolean; reason?: string }>;
}

export const useSovereigntyCore = (): SovereigntyCoreApi => {
  const queryEndpointRegistry = useCallback(async (filters) => {
    // Call local SovereigntyCore kernel via IPC or WASM bridge
    const response = await window.sovereigntyCore?.queryEndpointRegistry(filters);
    if (!response) throw new Error('SovereigntyCore not available');
    return response.entries;
  }, []);

  const requestRpcActivation = useCallback(async (request) => {
    // Host evaluates request against ALN corridors and RoH/eco envelopes
    const response = await window.sovereigntyCore?.requestRpcActivation(request);
    if (!response) throw new Error('SovereigntyCore not available');
    return response;
  }, []);

  const anchorToROW = useCallback(async (params) => {
    // Commit RpcSessionEnvelope to ROW ledger
    const response = await window.sovereigntyCore?.anchorToROW(params);
    if (!response) throw new Error('SovereigntyCore not available');
    return response.height;
  }, []);

  const queryActiveSession = useCallback(async (citizenDid) => {
    const response = await window.sovereigntyCore?.queryActiveSession(citizenDid);
    return response ?? null;
  }, []);

  const evaluateCorridor = useCallback(async (request) => {
    const response = await window.sovereigntyCore?.evaluateCorridor(request);
    return response ?? { allowed: false, reason: 'Corridor evaluation failed' };
  }, []);

  return {
    queryEndpointRegistry,
    requestRpcActivation,
    anchorToROW,
    queryActiveSession,
    evaluateCorridor,
  };
};

// Global type declaration for SovereigntyCore bridge
declare global {
  interface Window {
    sovereigntyCore?: {
      queryEndpointRegistry: (filters: any) => Promise<any>;
      requestRpcActivation: (request: any) => Promise<any>;
      anchorToROW: (params: any) => Promise<any>;
      queryActiveSession: (did: string) => Promise<any>;
      evaluateCorridor: (request: any) => Promise<any>;
    };
  }
}
