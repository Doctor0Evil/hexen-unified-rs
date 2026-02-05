use crate::types::{EvidenceBundle, EvidenceTag, UpgradeDescriptor};

pub fn default_bci_upgrade() -> UpgradeDescriptor {
    UpgradeDescriptor {
        id: "bci-safe-001".into(),
        description: "Baseline BCI neuromorphic-safe profile".into(),
        required_evidence: EvidenceBundle {
            tags: vec![
                EvidenceTag("a1f3c9b2".into()),
                EvidenceTag("8f09d5ee".into()),
            ],
        },
    }
}
