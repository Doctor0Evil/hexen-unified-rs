use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SwarmState7D {
    pub intensity: f32,
    pub duty_cycle: f32,
    pub cumulative_load: f32,
    pub implant_power: f32,
    pub neuromod_amp: f32,
    pub cognitive_load: f32,
    pub legal_complexity: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LifeforceState {
    pub cy: f32,
    pub zen: f32,
    pub chi: f32,
    pub integrity: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ViabilityKernel {
    // matrices loaded from .vkernel.aln
    pub mode_id: String,
    pub a: Vec<Vec<f32>>,
    pub b: Vec<f32>,
    pub min_integrity: f32,
    pub min_chi: f32,
}

impl ViabilityKernel {
    pub fn is_viable(
        &self,
        state: &SwarmState7D,
        lifeforce: &LifeforceState,
    ) -> bool {
        if lifeforce.integrity < self.min_integrity || lifeforce.chi < self.min_chi {
            return false;
        }
        let x = [
            state.intensity,
            state.duty_cycle,
            state.cumulative_load,
            state.implant_power,
            state.neuromod_amp,
            state.cognitive_load,
            state.legal_complexity,
        ];
        for (row, &b_i) in self.a.iter().zip(self.b.iter()) {
            let mut acc = 0.0;
            for (w, v) in row.iter().zip(x.iter()) {
                acc += w * v;
            }
            if acc > b_i + f32::EPSILON {
                return false;
            }
        }
        true
    }

    pub fn safe_filter(
        &self,
        state: &SwarmState7D,
        lifeforce: &LifeforceState,
        nominal: &SwarmState7D,
    ) -> SwarmState7D {
        if self.is_viable(state, lifeforce) {
            nominal.clone()
        } else {
            // conservative: zero control when outside kernel
            SwarmState7D {
                intensity: 0.0,
                duty_cycle: 0.0,
                cumulative_load: state.cumulative_load,
                implant_power: 0.0,
                neuromod_amp: 0.0,
                cognitive_load: state.cognitive_load,
                legal_complexity: state.legal_complexity,
            }
        }
    }
}
