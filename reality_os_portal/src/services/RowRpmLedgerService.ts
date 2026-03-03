import { DidUri } from '../types/aln/did';
import { RowEntry, RpmEntry, WorkType } from '../types/aln/row_rpm_ledger';
import { CyberspectreIntrospectionEngine } from '../cyberspectre/CyberspectreIntrospectionEngine';

export interface RowRpmLedgerApi {
  submitRowEntry: (entry: {
    citizenDid: DidUri;
    workType: WorkType;
    workDescription: string;
    ecoImpactDelta: number;
    joulesConsumed: number;
  }) => Promise<RowEntry>;

  queryRowEntries: (citizenDid: DidUri) => Promise<RowEntry[]>;

  queryRpmEntries: (citizenDid: DidUri) => Promise<RpmEntry[]>;

  calculateGovernanceWeight: (citizenDid: DidUri) => Promise<number>;

  calculateCacIndex: (citizenDid: DidUri) => Promise<number>;

  anchorToChain: (entry: RowEntry | RpmEntry) => Promise<{
    blockHeight: number;
    transactionHash: string;
    hexStampProof: string;
  }>;
}

export class RowRpmLedgerService implements RowRpmLedgerApi {
  private introspectionEngine: CyberspectreIntrospectionEngine;

  constructor(sessionId: string) {
    this.introspectionEngine = new CyberspectreIntrospectionEngine(sessionId);
  }

  async submitROWEntry(entry: {
    citizenDid: DidUri;
    workType: WorkType;
    workDescription: string;
    ecoImpactDelta: number;
    joulesConsumed: number;
  }): Promise<RowEntry> {
    // Record this action in Cyberspectre for audit trail
    this.introspectionEngine.recordNode({
      nonce: crypto.randomUUID(),
      did: entry.citizenDid,
      action: {
        id: 'portal.row.submit',
        title: 'Submit ROW Entry',
        layer: 'GOVERNANCE',
        alnCapability: 'aln.tx.row_entry',
        forwardOnly: true,
      },
      timestampIso: new Date().toISOString(),
      origin: {
        lang: 'TypeScript',
        file: 'RowRpmLedgerService.ts',
        lineStart: 45,
        colStart: 0,
        lineEnd: 75,
        colEnd: 0,
        authorDid: entry.citizenDid,
        symbolId: 'submitROWEntry',
      },
      payloadSummary: `ROW entry: ${entry.workType} - ${entry.workDescription.substring(0, 50)}`,
    });

    // Call SovereigntyCore to create ROW entry
    const rowEntry = await window.sovereigntyCore?.createRowEntry(entry);
    if (!rowEntry) throw new Error('SovereigntyCore not available');

    // Anchor to chain
    const anchorResult = await this.anchorToChain(rowEntry);

    return {
      ...rowEntry,
      block_height: anchorResult.blockHeight,
      transaction_hash: anchorResult.transactionHash,
      hex_stamp_proof: anchorResult.hexStampProof,
      is_immutable: true,
    };
  }

  async queryROWEntries(citizenDid: DidUri): Promise<RowEntry[]> {
    const entries = await window.sovereigntyCore?.queryROWEntries(citizenDid);
    return entries || [];
  }

  async queryRPMEntries(citizenDid: DidUri): Promise<RpmEntry[]> {
    const entries = await window.sovereigntyCore?.queryRPMEntries(citizenDid);
    return entries || [];
  }

  async calculateGovernanceWeight(citizenDid: DidUri): Promise<number> {
    const weight = await window.sovereigntyCore?.calculateGovernanceWeight(citizenDid);
    return weight || 0;
  }

  async calculateCACIndex(citizenDid: DidUri): Promise<number> {
    const cacIndex = await window.sovereigntyCore?.calculateCACIndex(citizenDid);
    return cacIndex || 0;
  }

  async anchorToChain(entry: RowEntry | RpmEntry): Promise<{
    blockHeight: number;
    transactionHash: string;
    hexStampProof: string;
  }> {
    const result = await window.sovereigntyCore?.anchorToChain(entry);
    if (!result) throw new Error('Failed to anchor to chain');
    return result;
  }
}

// Global type declaration for SovereigntyCore bridge
declare global {
  interface Window {
    sovereigntyCore?: {
      createROWEntry: (entry: any) => Promise<any>;
      queryROWEntries: (did: string) => Promise<any>;
      queryRPMEntries: (did: string) => Promise<any>;
      calculateGovernanceWeight: (did: string) => Promise<any>;
      calculateCACIndex: (did: string) => Promise<any>;
      anchorToChain: (entry: any) => Promise<any>;
    };
  }
}
