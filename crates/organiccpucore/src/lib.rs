use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganicCpuEnvelope {
    pub fatigue_max: f32,
    pub duty_cycle_max: f32,
    pub cognitive_load_max: f32,
    pub lifeforce_min: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BioState {
    pub fatigue_index: f32,
    pub duty_cycle: f32,
    pub cognitive_load_index: f32,
    pub lifeforce: f32,
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum EnvelopeViolation {
    #[error("fatigue_index {current} exceeds {max}")]
    Fatigue { current: f32, max: f32 },
    #[error("duty_cycle {current} exceeds {max}")]
    DutyCycle { current: f32, max: f32 },
    #[error("cognitive_load_index {current} exceeds {max}")]
    Cognitive { current: f32, max: f32 },
    #[error("lifeforce {current} below minimum {min}")]
    Lifeforce { current: f32, min: f32 },
}

impl OrganicCpuEnvelope {
    pub fn validate_state(&self, state: &BioState) -> Result<(), EnvelopeViolation> {
        if state.fatigue_index > self.fatigue_max + f32::EPSILON {
            return Err(EnvelopeViolation::Fatigue {
                current: state.fatigue_index,
                max: self.fatigue_max,
            });
        }
        if state.duty_cycle > self.duty_cycle_max + f32::EPSILON {
            return Err(EnvelopeViolation::DutyCycle {
                current: state.duty_cycle,
                max: self.duty_cycle_max,
            });
        }
        if state.cognitive_load_index > self.cognitive_load_max + f32::EPSILON {
            return Err(EnvelopeViolation::Cognitive {
                current: state.cognitive_load_index,
                max: self.cognitive_load_max,
            });
        }
        if state.lifeforce + f32::EPSILON < self.lifeforce_min {
            return Err(EnvelopeViolation::Lifeforce {
                current: state.lifeforce,
                min: self.lifeforce_min,
            });
        }
        Ok(())
    }
}

pub struct OrganicCpuCore {
    envelope: OrganicCpuEnvelope,
}

impl OrganicCpuCore {
    pub fn new(envelope: OrganicCpuEnvelope) -> Self {
        Self { envelope }
    }

    pub fn tick(&self, state: &BioState) -> Result<(), EnvelopeViolation> {
        self.envelope.validate_state(state)
    }

    pub fn tighten_envelope(&mut self, new_env: OrganicCpuEnvelope) {
        self.envelope.fatigue_max = new_env.fatigue_max.min(self.envelope.fatigue_max);
        self.envelope.duty_cycle_max = new_env.duty_cycle_max.min(self.envelope.duty_cycle_max);
        self.envelope.cognitive_load_max =
            new_env.cognitive_load_max.min(self.envelope.cognitive_load_max);
        self.envelope.lifeforce_min = new_env.lifeforce_min.max(self.envelope.lifeforce_min);
    }
}
