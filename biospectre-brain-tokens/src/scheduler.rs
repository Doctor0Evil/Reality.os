use crate::tokens::BiophysicalTokenBundle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageKind {
    Wake,
    N1,
    N2,
    N3,
    Rem,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct EpochMeta {
    pub stage: StageKind,
    pub sn3: f32,
    pub s_unknown: f32,
    pub eco_flops: u64,
    pub eco_energy_nj: f32,
}

#[derive(Debug, Clone)]
pub struct WorkloadRequest {
    pub id: String,
    pub required_brain_tokens: f32,
    pub required_dracula_quanta: f32,
    pub required_eco_nj: f32,
    pub allow_heavy_only_in_n3: bool,
}

pub struct BrainTokenScheduler;

impl BrainTokenScheduler {
    pub fn decide(
        tokens: &BiophysicalTokenBundle,
        epoch: &EpochMeta,
        request: &WorkloadRequest,
    ) -> bool {
        // Gate heavy workloads to N3 with high SN3, optional.
        if request.allow_heavy_only_in_n3 {
            if epoch.stage != StageKind::N3 || epoch.sn3 < 2.0 {
                return false;
            }
        }

        // Never schedule if eco-energy spike is extreme vs epoch baseline.
        if epoch.eco_energy_nj > request.required_eco_nj * 5.0 {
            return false;
        }

        tokens.can_schedule_workload(
            request.required_brain_tokens,
            request.required_dracula_quanta,
            request.required_eco_nj,
        )
    }
}
