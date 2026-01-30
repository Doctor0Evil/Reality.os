use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PKParams {
    pub k_abs: f64,      // absorption rate constant
    pub k_elim: f64,     // elimination rate constant
    pub v_plasma: f64,   // plasma volume (L)
    pub v_brain: f64,    // brain volume (L)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PKState {
    pub t: f64,            // time (s)
    pub dose_lung: f64,    // nicotine mass at lung/oral site
    pub conc_plasma: f64,  // mg/L
    pub conc_brain: f64,   // mg/L
}

impl PKState {
    pub fn step(&mut self, dt: f64, p: &PKParams) {
        let d_abs = p.k_abs * self.dose_lung * dt;
        self.dose_lung -= d_abs;
        let elim = p.k_elim * self.conc_plasma * p.v_plasma * dt;
        let delta_plasma = d_abs - elim;
        self.conc_plasma += delta_plasma / p.v_plasma;
        let brain_eq = self.conc_plasma; // simple equilibrium; extend as needed
        self.conc_brain += (brain_eq - self.conc_brain) * dt * 0.5;
        self.t += dt;
    }
}
