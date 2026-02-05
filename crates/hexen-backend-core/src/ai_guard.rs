use serde::{Deserialize, Serialize};
use organiccpualn::evolvestream::{EffectBounds, EvolutionProposalRecord};
use crate::intent::NeuromorphIntent; // your high-level intent type
use crate::sovereign_client::SovereignClient;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuardedIntentRequest {
    pub subject_id: String,
    pub intent: NeuromorphIntent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuardedIntentResponse {
    pub decision: String, // Allowed | Rejected
    pub reason: Option<String>,
}

pub async fn handle_guarded_intent(
    client: &SovereignClient,
    req: GuardedIntentRequest,
) -> anyhow::Result<GuardedIntentResponse> {
    // Map intent -> typed EffectBounds and RoH estimates
    let (scope, effect_bounds, roh_before, roh_after) = req.intent.to_effects()?;

    let proposal = EvolutionProposalRecord {
        proposalid: req.intent.proposal_id(),
        subjectid: req.subject_id.clone(),
        scope,
        kind: req.intent.kind(),
        module: req.intent.module(),
        updatekind: req.intent.update_kind(),
        effectbounds: EffectBounds {
            l2_delta_norm: effect_bounds.l2_delta_norm,
            irreversible: effect_bounds.irreversible,
        },
        roh_before,
        roh_after,
        tsafe_mode: req.intent.tsafe_mode(),
        signer_roles: req.intent.signer_roles(),
        tokenkind: req.intent.token_kind(),
        decision: "Pending".into(),
        hexstamp: String::new(),
        timestamp_utc: chrono::Utc::now().to_rfc3339(),
    };

    let sovereign_decision = client.evaluate_update(proposal).await?;
    Ok(match sovereign_decision {
        crate::sovereign_client::DecisionOutcome::Allowed => GuardedIntentResponse {
            decision: "Allowed".into(),
            reason: None,
        },
        crate::sovereign_client::DecisionOutcome::Rejected { reason } => {
            GuardedIntentResponse {
                decision: "Rejected".into(),
                reason: Some(reason),
            }
        }
    })
}
