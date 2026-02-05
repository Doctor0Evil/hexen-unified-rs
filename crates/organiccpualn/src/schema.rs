use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OcpuProfileAln {
    pub subject_id: String,
    pub roh_ceiling: f32,
    pub fatigue_max: f32,
    pub duty_cycle_max: f32,
    pub cognitive_load_max: f32,
    pub lifeforce_min: f32,
}

impl OcpuProfileAln {
    pub fn to_envelope(&self) -> organiccpucore::OrganicCpuEnvelope {
        organiccpucore::OrganicCpuEnvelope {
            fatigue_max: self.fatigue_max,
            duty_cycle_max: self.duty_cycle_max,
            cognitive_load_max: self.cognitive_load_max,
            lifeforce_min: self.lifeforce_min,
        }
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.roh_ceiling > 0.30 + f32::EPSILON {
            anyhow::bail!("RoH ceiling must be ≤ 0.30");
        }
        Ok(())
    }
}

// Mapper from Kubernetes OrganicCpuProfile CRD to ALN-bound structs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrganicCpuProfileSpec {
    pub subjectId: String,
    pub rohCeiling: f32,
    pub envelopes: EnvelopesSpec,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvelopesSpec {
    pub fatigueMax: f32,
    pub dutyCycleMax: f32,
    pub cognitiveLoadMax: f32,
    pub lifeforceMin: f32,
}

impl From<OrganicCpuProfileSpec> for OcpuProfileAln {
    fn from(s: OrganicCpuProfileSpec) -> Self {
        Self {
            subject_id: s.subjectId,
            roh_ceiling: s.rohCeiling,
            fatigue_max: s.envelopes.fatigueMax,
            duty_cycle_max: s.envelopes.dutyCycleMax,
            cognitive_load_max: s.envelopes.cognitiveLoadMax,
            lifeforce_min: s.envelopes.lifeforceMin,
        }
    }
}
