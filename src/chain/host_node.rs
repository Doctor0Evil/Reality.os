//! Biophysical Host Node (AugDoctor, Phoenix-grade)
//!
//! - Maintains a sealed, per-host BioTokenState.
//! - Exposes JSON-over-TCP RPC for:
//!     * reading redacted state summaries,
//!     * submitting RuntimeEvents (WaveLoad, SmartAutonomy, EvolutionUpgrade).
//! - Applies ALNDID gating + Lifeforce safety at node boundary.
//! - No transfers, no staking, no financial ops.

#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::task::JoinHandle;

use crate::chain::biophysical_runtime::{
    alndid, biospectreconsensus, lifeforcesafety, quantumhash, BiophysicalRuntime,
    BioTokenState, LorentzTimeSource, LorentzTimestamp, RuntimeConfig, RuntimeEvent,
    RuntimeEventKind, RuntimeResult, ALNHostFrame, SystemLorentzClock,
};

// -------------------------- Storage ---------------------------------------

#[derive(Clone)]
struct HostStorage {
    inner: Arc<RwLock<InnerStore>>,
}

#[derive(Clone)]
struct InnerStore {
    state: BioTokenState,
    last_frame: Option<biospectreconsensus::ConsensusFrame>,
}

impl HostStorage {
    fn new(initial_state: BioTokenState) -> Self {
        Self {
            inner: Arc::new(RwLock::new(InnerStore {
                state: initial_state,
                last_frame: None,
            })),
        }
    }

    fn read_state(&self) -> BioTokenState {
        self.inner.read().unwrap().state.clone()
    }

    fn read_last_frame(&self) -> Option<biospectreconsensus::ConsensusFrame> {
        self.inner.read().unwrap().last_frame.clone()
    }

    fn apply_state_and_frame(
        &self,
        new_state: BioTokenState,
        frame: biospectreconsensus::ConsensusFrame,
    ) {
        let mut guard = self.inner.write().unwrap();
        guard.state = new_state;
        guard.last_frame = Some(frame);
    }
}

// ----------------- In-memory DID directory + consent verifier -------------

#[derive(Clone)]
struct InMemoryDIDDirectory {
    access: Arc<RwLock<HashMap<String, alndid::AccessEnvelope>>>,
}

impl InMemoryDIDDirectory {
    fn new() -> Self {
        Self {
            access: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn insert(&self, env: alndid::AccessEnvelope) {
        self.access
            .write()
            .unwrap()
            .insert(env.did.id.clone(), env);
    }
}

impl alndid::DIDDirectory for InMemoryDIDDirectory {
    fn resolve_access(&self, did: alndid::ALNDID) -> Option<alndid::AccessEnvelope> {
        self.access.read().unwrap().get(&did.id).cloned()
    }

    fn is_ethical_operator(&self, did: alndid::ALNDID) -> bool {
        self.resolve_access(did)
            .map(|a| a.roles.contains(&alndid::RoleClass::EthicalOperator))
            .unwrap_or(false)
    }
}

#[derive(Clone)]
struct SimpleConsentVerifier;

impl alndid::ConsentVerifier for SimpleConsentVerifier {
    fn verify_self_consent(&self, proof: alndid::ConsentProof) -> bool {
        !proof.zk_sig.is_empty() && !proof.evolution_event_id.is_empty()
    }
}

// ----------------- Local host consensus implementation --------------------

#[derive(Clone)]
struct LocalHostConsensus;

impl biospectreconsensus::HostConsensus for LocalHostConsensus {
    fn validate_state_step(
        &self,
        previous: Option<biospectreconsensus::ConsensusFrame>,
        next: &biospectreconsensus::ConsensusFrame,
        _state: &BioTokenState,
    ) -> Result<(), &'static str> {
        if let Some(prev) = previous {
            if next.seq_no != prev.seq_no + 1 {
                return Err("Sequence mismatch in consensus frame.");
            }
            if let Some(prev_hash) = &next.prev_state_hash {
                if prev_hash != &prev.state_hash {
                    return Err("Previous hash mismatch in consensus frame.");
                }
            } else {
                return Err("Missing prev_state_hash while previous frame exists.");
            }
        } else if next.seq_no != 0 {
            return Err("Genesis frame must have seq_no 0.");
        }
        Ok(())
    }
}

// --------------------------- Lifeforce config -----------------------------

fn default_lifeforce_state() -> lifeforcesafety::LifeforceState {
    lifeforcesafety::LifeforceState {
        bands: lifeforcesafety::MetabolicBands {
            blood_min: 0.25,
            blood_soft_floor: 0.35,
            oxygen_min: 0.90,
            oxygen_soft_floor: 0.94,
        },
        wave_curve: lifeforcesafety::DraculaWaveCurve {
            max_wave_factor: 0.6,
            decay_coefficient: 0.01,
        },
        nano_envelope: lifeforcesafety::NanoEnvelope {
            max_concurrent_workload: 1.0,
            eco_penalty_factor: 0.5,
        },
    }
}

// --------------------------- RPC Types ------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct RedactedStateSummary {
    brain: f64,
    wave: f64,
    blood: f64,
    oxygen: f64,
    nano: f64,
    smart: f64,
    host_id: String,
    lorentz_ts_ns: i128,
}

impl From<&BioTokenState> for RedactedStateSummary {
    fn from(s: &BioTokenState) -> Self {
        Self {
            brain: s.brain,
            wave: s.wave,
            blood: s.blood,
            oxygen: s.oxygen,
            nano: s.nano,
            smart: s.smart,
            host_id: s.host_id.id.clone(),
            lorentz_ts_ns: s.lorentz_ts.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum RpcRequest {
    GetState,
    SubmitEvent {
        kind: String,
        evolution_id: Option<String>,
        task_id: Option<String>,
        agent_id: Option<String>,
        requested_wave: Option<f64>,
        requested_smart: Option<f64>,
        initiator_id: String,
        initiator_shard: String,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
enum RpcResponse {
    OkState { state: RedactedStateSummary },
    OkAck { seq_no: u64 },
    Err { message: String },
}

// ------------------------- Node struct + server ---------------------------

pub struct HostNode {
    storage: HostStorage,
    runtime: BiophysicalRuntime<InMemoryDIDDirectory, SimpleConsentVerifier, LocalHostConsensus>,
    clock: SystemLorentzClock,
}

impl HostNode {
    pub fn new(initial_state: BioTokenState) -> Self {
        let storage = HostStorage::new(initial_state);
        let did_directory = InMemoryDIDDirectory::new();
        let consent_verifier = SimpleConsentVerifier;
        let consensus = LocalHostConsensus;
        let lifeforce = default_lifeforce_state();
        let cfg = RuntimeConfig::default();

        let runtime = BiophysicalRuntime::new(cfg, lifeforce, did_directory, consent_verifier, consensus);

        Self {
            storage,
            runtime,
            clock: SystemLorentzClock,
        }
    }

    pub fn did_directory_mut(&mut self) -> &mut InMemoryDIDDirectory {
        // internal helper; used in tests or bootstrapping
        unsafe {
            // logically safe because we only expose through &mut self
            &mut *(&mut self.runtime.did_directory as *mut _)
        }
    }

    async fn handle_client(self: Arc<Self>, stream: TcpStream) -> RuntimeResult<()> {
        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
            let request: Result<RpcRequest, _> = serde_json::from_str(&line.trim());
            line.clear();

            let response = match request {
                Ok(RpcRequest::GetState) => {
                    let state = self.storage.read_state();
                    RpcResponse::OkState {
                        state: RedactedStateSummary::from(&state),
                    }
                }
                Ok(RpcRequest::SubmitEvent {
                    kind,
                    evolution_id,
                    task_id,
                    agent_id,
                    requested_wave,
                    requested_smart,
                    initiator_id,
                    initiator_shard,
                }) => {
                    let initiator = alndid::ALNDID {
                        id: initiator_id,
                        shard: initiator_shard,
                    };

                    let event_kind = match kind.as_str() {
                        "EvolutionUpgrade" => RuntimeEventKind::EvolutionUpgrade {
                            evolution_id: evolution_id.unwrap_or_else(|| "unknown".into()),
                        },
                        "WaveLoad" => RuntimeEventKind::WaveLoad {
                            task_id: task_id.unwrap_or_else(|| "task".into()),
                            requested_wave: requested_wave.unwrap_or(0.0),
                        },
                        "SmartAutonomy" => RuntimeEventKind::SmartAutonomy {
                            agent_id: agent_id.unwrap_or_else(|| "agent".into()),
                            requested_smart: requested_smart.unwrap_or(0.0),
                        },
                        _ => {
                            RpcResponse::Err {
                                message: "Unknown event kind.".into(),
                            }
                        }
                    };

                    if matches!(&event_kind, RpcResponse::Err { .. }) {
                        event_kind
                    }

                    let now = self.clock.now_lorentz();

                    let event = RuntimeEvent {
                        kind: match event_kind {
                            RuntimeEventKind::EvolutionUpgrade { evolution_id } => {
                                RuntimeEventKind::EvolutionUpgrade { evolution_id }
                            }
                            RuntimeEventKind::WaveLoad { task_id, requested_wave } => {
                                RuntimeEventKind::WaveLoad { task_id, requested_wave }
                            }
                            RuntimeEventKind::SmartAutonomy { agent_id, requested_smart } => {
                                RuntimeEventKind::SmartAutonomy { agent_id, requested_smart }
                            }
                        },
                        initiator: initiator.clone(),
                        consent: None, // attach proofs at higher layers when needed
                        lorentz_ts: now,
                    };

                    let current_state = self.storage.read_state();
                    let last_frame = self.storage.read_last_frame();
                    let host_frame = ALNHostFrame {
                        host_id: current_state.host_id.clone(),
                        access: alndid::AccessEnvelope {
                            did: initiator,
                            roles: vec![alndid::RoleClass::Host],
                            min_biophysics_knowledge_score: 1.0,
                        },
                        lorentz_ts: now,
                    };

                    match self
                        .runtime
                        .execute_event(current_state.clone(), last_frame.clone(), host_frame, event)
                    {
                        Ok(frame) => {
                            self.storage.apply_state_and_frame(current_state, frame.clone());
                            RpcResponse::OkAck { seq_no: frame.seq_no }
                        }
                        Err(e) => RpcResponse::Err {
                            message: format!("{:?}", e),
                        },
                    }
                }
                Err(e) => RpcResponse::Err {
                    message: format!("Invalid request: {}", e),
                },
            };

            let json = serde_json::to_string(&response).unwrap_or_else(|_| {
                serde_json::to_string(&RpcResponse::Err {
                    message: "Serialization failure.".into(),
                })
                .unwrap()
            });
            writer.write_all(json.as_bytes()).await.unwrap();
            writer.write_all(b"\n").await.unwrap();
        }

        Ok(())
    }

    pub async fn start(self: Arc<Self>, addr: SocketAddr) -> JoinHandle<()> {
        let listener = TcpListener::bind(addr).await.unwrap();
        tokio::spawn(async move {
            loop {
                if let Ok((stream, _)) = listener.accept().await {
                    let node = self.clone();
                    tokio::spawn(async move {
                        let _ = node.handle_client(stream).await;
                    });
                }
            }
        })
    }
}
