use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostBudget {
    pub max_duty_cycle: f32,
    pub max_thermal_c: f32,
    pub max_power_w: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceTag(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceBundle {
    pub tags: Vec<EvidenceTag>,
}

impl EvidenceBundle {
    pub fn within_budget(&self, _budget: &HostBudget) -> bool {
        true
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeDescriptor {
    pub id: String,
    pub description: String,
    pub required_evidence: EvidenceBundle,
}
