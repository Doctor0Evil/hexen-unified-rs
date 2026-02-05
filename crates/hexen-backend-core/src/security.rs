use bioscale_upgrade_store::types::{EvidenceBundle, HostBudget};
use hexen_evidence_registry::EvidenceRegistry;

#[derive(Debug)]
pub struct NeurorightsGuard {
    registry: EvidenceRegistry,
    budget: HostBudget,
}

impl NeurorightsGuard {
    pub fn new(registry: EvidenceRegistry, budget: HostBudget) -> Self {
        Self { registry, budget }
    }

    pub fn validate_bundle(&self, bundle: &EvidenceBundle) -> bool {
        self.registry.is_bundle_satisfied(bundle) && bundle.within_budget(&self.budget)
    }
}
