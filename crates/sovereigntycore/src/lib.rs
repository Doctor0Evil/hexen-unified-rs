use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use organiccpualn::{
    rohmodel::{RohModelShard, RohInputs},
    stake::StakeShard,
    donutloopledger::{DonutloopEntry, DonutloopLedger},
    evolvestream::EvolutionProposalRecord,
    neurorights::NeurorightsPolicyDocument,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DecisionOutcome {
    Allowed,
    Rejected { reason: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SovereigntyCore {
    pub roh_model: RohModelShard,
    pub stake: StakeShard,
    pub neurorights: NeurorightsPolicyDocument,
    pub donutloop: DonutloopLedger,
    pub roh_ceiling: f32, // always 0.30 from .rohmodel.aln
}

impl SovereigntyCore {
    pub fn new(
        roh_model: RohModelShard,
        stake: StakeShard,
        neurorights: NeurorightsPolicyDocument,
        donutloop: DonutloopLedger,
    ) -> anyhow::Result<Self> {
        let roh_ceiling = roh_model.roh_ceiling();
        // validate invariants at init
        roh_model.validate_invariants()?;
        stake.validate()?;
        neurorights.validate()?;
        donutloop.validate_chain()?;
        Ok(Self {
            roh_model,
            stake,
            neurorights,
            donutloop,
            roh_ceiling,
        })
    }

    pub fn evaluate_update(
        &mut self,
        proposal: EvolutionProposalRecord,
    ) -> anyhow::Result<DecisionOutcome> {
        // 1. RoH guard (monotone + ceiling)
        if proposal.roh_after > self.roh_ceiling + f32::EPSILON {
            return Ok(DecisionOutcome::Rejected {
                reason: "RoH_after exceeds global ceiling".into(),
            });
        }
        if proposal.roh_after > proposal.roh_before + f32::EPSILON {
            return Ok(DecisionOutcome::Rejected {
                reason: "RoH_after > RoH_before (no relaxation allowed)".into(),
            });
        }

        // 2. Stake / multisig guard
        self.stake
            .check_signers_for_scope(&proposal.scope, &proposal.signer_roles)?;

        // 3. Neurorights guard (dream sensitivity, forbidden decision uses, etc.)
        self.neurorights
            .enforce_for_proposal(&proposal, &self.roh_model)?;

        // 4. Token guard (SMART vs EVOLVE scopes)
        proposal.enforce_token_scopes(&self.stake)?;

        // 5. Donutloop append with RoH monotone + hashlink check
        let entry = DonutloopEntry::from_proposal(&proposal);
        self.donutloop.append(entry)?;

        Ok(DecisionOutcome::Allowed)
    }
}
