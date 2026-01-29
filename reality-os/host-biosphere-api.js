export class HostBiosphereApi {
  constructor(jsonRpcEndpoint, hostId) {
    this.endpoint = jsonRpcEndpoint;
    this.hostId = hostId;
  }

  async getSummary() {
    const payload = {
      jsonrpc: "2.0",
      id: `summary-${this.hostId}`,
      method: "ledger_getHostSummary",
      params: [this.hostId],
    };
    const res = await fetch(this.endpoint, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });
    const body = await res.json();
    return body.result; // eco_band, lifeforce_band, adapter_health, history
  }

  async proposeEvolutionFrame(frame) {
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
    const body = await res.json();
    return body.result;
  }
}
