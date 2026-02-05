pub mod registry;

use crate::registry::DEFAULT_BIOPHYS_EVIDENCE;
use bioscale_upgrade_store::types::EvidenceBundle;

#[derive(Debug, Clone)]
pub struct EvidenceRegistry;

impl EvidenceRegistry {
    pub fn new() -> Self {
        Self
    }

    pub fn is_bundle_satisfied(&self, bundle: &EvidenceBundle) -> bool {
        !bundle.tags.is_empty() && !DEFAULT_BIOPHYS_EVIDENCE.is_empty()
    }
}
