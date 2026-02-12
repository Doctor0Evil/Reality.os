use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowMetrics {
    pub fear: f64,
    pub block_rate: f64,
    pub force_repair_rate: f64,
    pub repair_rate: f64,
    pub clean_tech_rate: f64,
    pub support_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElasticityResult {
    pub cp_prev: f64,
    pub cp_curr: f64,
    pub rt_prev: f64,
    pub rt_curr: f64,
    pub rec_bounded: f64,
}

const EPSILON: f64 = 1e-6;
const GAMMA: f64 = 1.0;

fn constraint_pressure(m: &WindowMetrics) -> f64 {
    let cp =
        0.4 * m.fear +
        0.3 * m.block_rate +
        0.3 * m.force_repair_rate;
    cp.clamp(0.0, 1.0)
}

fn repair_throughput(m: &WindowMetrics) -> f64 {
    let rt =
        0.4 * m.repair_rate +
        0.3 * m.clean_tech_rate +
        0.3 * m.support_rate;
    rt.clamp(0.0, 1.0)
}

pub fn compute_elasticity(
    prev: &WindowMetrics,
    curr: &WindowMetrics
) -> ElasticityResult {

    let cp_prev = constraint_pressure(prev);
    let cp_curr = constraint_pressure(curr);

    let rt_prev = repair_throughput(prev);
    let rt_curr = repair_throughput(curr);

    let delta_cp = cp_curr - cp_prev;
    let delta_rt = rt_curr - rt_prev;

    let ratio = delta_rt / (delta_cp.abs() + EPSILON);
    let rec_bounded = (GAMMA * ratio).tanh();

    ElasticityResult {
        cp_prev,
        cp_curr,
        rt_prev,
        rt_curr,
        rec_bounded,
    }
}
