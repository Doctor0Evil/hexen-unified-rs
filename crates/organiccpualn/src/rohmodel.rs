use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohWeights {
    pub energy_load: f32,
    pub thermal_load: f32,
    pub cognitive_load: f32,
    pub inflammation: f32,
    pub eco_impact: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohModelCore {
    pub id: String,
    pub weights: RohWeights,
    pub roh_ceiling: f32, // must be 0.30
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohModelShard {
    pub model: RohModelCore,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RohInputs {
    pub energy_load: f32,
    pub thermal_load: f32,
    pub cognitive_load: f32,
    pub inflammation: f32,
    pub eco_impact: f32,
}

impl RohModelShard {
    pub fn compute_roh(&self, inputs: &RohInputs) -> f32 {
        let w = &self.model.weights;
        let r = w.energy_load * inputs.energy_load
            + w.thermal_load * inputs.thermal_load
            + w.cognitive_load * inputs.cognitive_load
            + w.inflammation * inputs.inflammation
            + w.eco_impact * inputs.eco_impact;
        r.clamp(0.0, 1.0)
    }

    pub fn roh_ceiling(&self) -> f32 {
        self.model.roh_ceiling
    }

    pub fn validate_invariants(&self) -> anyhow::Result<()> {
        use anyhow::{anyhow, bail};
        let w = &self.model.weights;
        if self.model.roh_ceiling != 0.30 {
            bail!("RoH ceiling must be 0.30");
        }
        let weights = [
            w.energy_load,
            w.thermal_load,
            w.cognitive_load,
            w.inflammation,
            w.eco_impact,
        ];
        if weights.iter().any(|v| *v < 0.0) {
            bail!("RoH weights must be non‑negative");
        }
        let sum: f32 = weights.iter().sum();
        if (sum - 1.0).abs() > 1e-4 {
            bail!("RoH weights must sum to 1.0, got {}", sum);
        }
        Ok(())
    }
}
