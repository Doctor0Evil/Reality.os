#[derive(Debug, Clone)]
pub struct NeuralRope7D {
    pub components: [f32; 7],
}

#[derive(Debug, Clone)]
pub struct NeuralRope5D {
    pub components: [f32; 5],
}

const PROJECTION_5X7: [[f32; 7]; 5] = [
    // Row 0: emphasize SN3 and delta dominance
    [0.0, 0.0, 0.6, 0.2, 0.0, 0.0, 0.6],
    // Row 1: emphasize S? and spindle density
    [0.0, 0.0, 0.1, 0.7, 0.0, 0.6, 0.0],
    // Row 2: theta/alpha and SN1
    [0.6, 0.0, 0.0, 0.0, 0.6, 0.0, 0.0],
    // Row 3: SN2 and spindle density
    [0.0, 0.7, 0.0, 0.0, 0.0, 0.7, 0.0],
    // Row 4: mixed stabilizer
    [0.2, 0.2, 0.2, 0.2, 0.2, 0.2, 0.2],
];

impl NeuralRope7D {
    pub fn project_to_5d(&self) -> NeuralRope5D {
        let mut out = [0.0_f32; 5];
        for (i, row) in PROJECTION_5X7.iter().enumerate() {
            let mut acc = 0.0_f32;
            for j in 0..7 {
                acc += row[j] * self.components[j];
            }
            out[i] = acc;
        }
        NeuralRope5D { components: out }
    }
}
