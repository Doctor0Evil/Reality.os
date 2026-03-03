export type DidUri = string;

export interface OriginSpan {
  lang: "Rust" | "Lua" | "TypeScript" | "WASM" | "ALN";
  file: string;
  lineStart: number;
  colStart: number;
  lineEnd: number;
  colEnd: number;
  authorDid: DidUri;
  symbolId: string;
}

export type PortalLayer =
  | "IDENTITY"
  | "BIOPHYSICAL"
  | "LEARNING"
  | "HEALTH"
  | "MISSION"
  | "GOVERNANCE"
  | "UX"
  | "CORE_ALN";

export interface PortalActionMeta {
  id: string;               // e.g. "portal.btn.72.multisig-care-team-attestor"
  title: string;            // human label
  layer: PortalLayer;
  alnCapability: string;    // e.g. "aln.tx.multisig_attest"
  forwardOnly: true;
}

export interface ProvenanceNode {
  nonce: string;            // UI-level uniqueness; final hex-stamp is on-chain
  did: DidUri;
  action: PortalActionMeta;
  timestampIso: string;
  origin: OriginSpan;
  payloadSummary: string;   // small, non-PII summary for audit UIs
}

export interface IntrospectionRecord {
  sessionId: string;
  nodes: ProvenanceNode[];
}

/**
 * Cyberspectre-based introspection layer:
 * collects portal actions as structured provenance nodes,
 * ready to be hex-stamped and anchored on ALN via ROW-style logs.
 */
export class CyberspectreIntrospectionEngine {
  private readonly sessionId: string;
  private readonly nodes: ProvenanceNode[] = [];

  constructor(sessionId: string) {
    this.sessionId = sessionId;
  }

  recordNode(node: ProvenanceNode): void {
    // Forward-only guard: never mutate existing nodes.
    this.nodes.push(node);
  }

  exportRecord(): IntrospectionRecord {
    // Shallow copy to avoid external mutation.
    return {
      sessionId: this.sessionId,
      nodes: this.nodes.slice(),
    };
  }
}
