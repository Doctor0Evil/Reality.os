export class OtaNeuromorphClient {
  constructor(jsonRpcEndpoint, hostId) {
    this.endpoint = jsonRpcEndpoint;
    this.hostId = hostId;
  }

  buildEvolutionFrame(candidateUpdate) {
    return {
      host: this.hostId,
      frame_id: candidateUpdate.frameId,
      plane: candidateUpdate.plane,
      scope: candidateUpdate.scope,
      cost: {
        flop_budget: candidateUpdate.flops,
        nJ_budget: candidateUpdate.nJ,
        eco_intent: candidateUpdate.ecoIntent,
      },
      expected_effect: {
        latency_band: candidateUpdate.latencyBand,
        error_band: candidateUpdate.errorBand,
        eco_impact_band: candidateUpdate.ecoImpactBand,
      },
      guards_snapshot: candidateUpdate.guardsSnapshot,
    };
  }

  async submitFrame(frame) {
    const payload = {
      jsonrpc: "2.0",
      id: frame.frame_id,
      method: "ledger_submitEvolutionFrame",
      params: [frame],
    };

    const res = await fetch(this.endpoint, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });

    if (!res.ok) {
      throw new Error(`RPC error: ${res.status}`);
    }

    const body = await res.json();
    return body.result;
  }

  async proposeNeuromorphUpdate(candidateUpdate) {
    const frame = this.buildEvolutionFrame(candidateUpdate);
    const decision = await this.submitFrame(frame);

    if (decision.verdict === "Safe") {
      this.applyUiFeedback("safe", decision);
    } else if (decision.verdict === "Defer") {
      this.applyUiFeedback("defer", decision);
    } else {
      this.applyUiFeedback("deny", decision);
    }

    return decision;
  }

  applyUiFeedback(state, decision) {
    // Hook for haptics, visuals, logs in Reality.os
    console.log("[ota-neuromorph]", state, decision);
  }
}
