use bioscale_upgrade_store::types::HostBudget;

/// Neuromorphic job envelope with pre-computed duty math.
pub struct NeurotaskEnvelope {
    pub job_id: String,
    pub estimated_duty: f32,
}

pub fn within_budget(env: &NeurotaskEnvelope, budget: &HostBudget) -> bool {
    env.estimated_duty <= budget.max_duty_cycle
}
