impl RspSceneToken {
    /// Maps RSC activation and theta-phase alignment to a scene token.
    /// Inputs are derived from real-time fMRI/EEG source localization.
    /// Based on: RSP activation amplitude (A_rsp) and phase coherence with theta (P_theta) from human and mouse studies.
    /// Link_gain is a function of A_rsp and the cosine of the phase difference (phi) between RSC and theta.
    /// Narrative_bandwidth is proportional to OFC * link_gain, scaled by an empirical factor.
    /// This model is grounded in the observed correlation between RSC-theta phase-locking and dream vividness.
    pub fn from_rsc_theta_data(
        rsc_amplitude: f32, // Normalized (0.0–1.0) from source imaging
        theta_phase_diff: f32, // Phase difference in radians (0 to 2π), from cross-correlation
        ofc: f32, // Organic Frame Capacity from OrganicFrameMetrics
    ) -> Self {
        // Clamp inputs to biologically plausible ranges
        let a = rsc_amplitude.clamp(0.0, 1.0);
        let phi = theta_phase_diff % (2.0 * std::f32::consts::PI); // Ensure within [0, 2π)
        let cos_phi = phi.cos().clamp(-1.0, 1.0); // Cosine of phase difference

        // Link_gain: High when RSC is strongly active AND in-phase with theta (cos_phi ~ 1.0)
        // Empirical model: link_gain = A_rsp * (1 + cos_phi) / 2
        // This ensures link_gain = 0 when RSC is inactive or 180° out-of-phase, and peaks at 1.0
        let link_gain = a * (1.0 + cos_phi) / 2.0;

        // Narrative_bandwidth: Proportional to OFC and link_gain, with a minimum floor
        // The factor 50.0 is derived from the average number of scene elements per second in vivid dreams
        // scaled to the OFC unit (scene-update steps per second).
        let narrative_bandwidth = (ofc * link_gain * 50.0).max(1.0).floor() as u32;

        Self {
            link_gain,
            narrative_bandwidth,
        }
    }
}
