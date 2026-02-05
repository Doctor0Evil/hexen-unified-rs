use std::collections::HashMap;

use cybernano_viability_kernel::{LifeforceState, SwarmState7D, ViabilityKernel};
use cybernano_vector_cyberrank::{self, CandidateAction, RankWeights};
use organiccpualn::{
    donutloopledger::{DonutloopEntry, DonutloopLedger},
    evolvestream::{EvolutionLogWriter, EvolutionProposalRecord},
    neurorights::NeurorightsPolicyDocument,
    rohmodel::{RohInputs, RohModelShard},
    stake::StakeShard,
};
use serde::{Deserialize, Serialize};

mod types;
pub mod metrics;

use crate::types::{NeuroRightsPolicy, Scope, TokenKind, UpdateProposal};

/// Final decision of the sovereignty core.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DecisionOutcome {
    Allowed,
    Rejected { reason: String },
}

/// High‑trust sovereignty kernel wired to RoH, neurorights, stake, Tsafe, and donutloop.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SovereigntyCore<LW: EvolutionLogWriter> {
    // Risk-of-Harm model shard (RoH ≤ 0.30, weights sum to 1.0, single active row).
    roh_model: RohModelShard,

    // Stake / multisig governance shard.
    stake: StakeShard,

    // Neurorights policy as loaded from .neurorights.json.
    neurorights_doc: NeurorightsPolicyDocument,

    // Compiled neurorights policy used for fast guards in the Tsafe path.
    neurorights: NeuroRightsPolicy,

    // Internal donutloop ledger (.donutloop.aln, hash-linked, append-only).
    donutloop: DonutloopLedger,

    // Global RoH ceiling (must be 0.30 from .rohmodel.aln).
    roh_ceiling: f32,

    // Viability kernel (7D polytope) for Tsafe checks.
    viability_kernel: ViabilityKernel,

    // CyberRank weights for Tsafe action selection.
    rank_weights: RankWeights,

    // Evolution log writer for .evolve.jsonl.
    evolvelog: LW,
}

impl<LW: EvolutionLogWriter> SovereigntyCore<LW> {
    /// Construct a new sovereignty core, validating all invariants up front.
    pub fn new(
        roh_model: RohModelShard,
        stake: StakeShard,
        neurorights_doc: NeurorightsPolicyDocument,
        viability_kernel: ViabilityKernel,
        rank_weights: RankWeights,
        evolvelog: LW,
        donutloop: DonutloopLedger,
    ) -> anyhow::Result<Self> {
        // 1. RoH invariants (ceiling 0.30, weights ≥ 0, sum 1.0, exactly one active row). [file:1]
        roh_model.validate_invariants()?;

        // 2. Extract and hard‑lock global RoH ceiling. [file:1]
        let roh_ceiling = roh_model.roh_ceiling();
        if (roh_ceiling - 0.30).abs() > f32::EPSILON {
            anyhow::bail!("RohModelShard.roh_ceiling must be 0.30");
        }

        // 3. Stake invariants: exactly one Host per subject, multisig rules well‑formed. [file:1]
        stake.validate()?;

        // 4. Neurorights invariants: dreamstate, noncommercial, forbidDecisionUse, etc. [file:4]
        neurorights_doc.validate()?;

        // 5. Compile fast neurorights view used by Tsafe path. [file:3][file:4]
        let neurorights = NeuroRightsPolicy::from_document(&neurorights_doc)?;

        // 6. Donutloop chain invariants: append‑only, hash‑linked, consistent prev_hexstamp. [file:1]
        donutloop.validate_chain()?;

        // 7. Viability kernel polytope invariants. [file:3]
        viability_kernel.validate()?;

        Ok(Self {
            roh_model,
            stake,
            neurorights_doc,
            neurorights,
            donutloop,
            roh_ceiling,
            viability_kernel,
            rank_weights,
            evolvelog,
        })
    }

    /// Evaluate a low‑level evolution record (canonical evolve stream path). [file:1][file:3]
    pub fn evaluate_evolution_record(
        &mut self,
        proposal: EvolutionProposalRecord,
    ) -> anyhow::Result<DecisionOutcome> {
        // 1. RoH guard (monotone + ceiling).
        if proposal.roh_after > self.roh_ceiling + f32::EPSILON {
            return Ok(DecisionOutcome::Rejected {
                reason: "RoH_after exceeds global ceiling (0.30)".into(),
            });
        }
        if proposal.roh_after > proposal.roh_before + f32::EPSILON {
            return Ok(DecisionOutcome::Rejected {
                reason: "RoH_after > RoH_before (no relaxation allowed)".into(),
            });
        }

        // 2. Stake / multisig guard based on scope and signer roles. [file:1]
        self.stake
            .check_signers_for_scope(&proposal.scope, &proposal.signer_roles)?;

        // 3. Neurorights guard (dreamstate, non‑commercial, forbidden decision uses, etc.). [file:4]
        self.neurorights_doc
            .enforce_for_proposal(&proposal, &self.roh_model)?;

        // 4. Token guard (SMART vs EVOLVE scopes). [file:3]
        proposal.enforce_token_scopes(&self.stake)?;

        // 5. Donutloop append with RoH monotone + hashlink check. [file:1]
        let entry = DonutloopEntry::from_proposal(&proposal);
        self.donutloop.append(entry)?;

        Ok(DecisionOutcome::Allowed)
    }

    /// Evaluate a high‑level update proposal with Tsafe / CyberRank and neurorights guards. [file:3]
    pub fn evaluate_update(
        &mut self,
        proposal: UpdateProposal,
        swarm_state: &SwarmState7D,
        lifeforce: &LifeforceState,
    ) -> anyhow::Result<DecisionOutcome> {
        // 1. RoH monotone + neurorights ceiling.
        if proposal.roh_after > self.neurorights.roh_ceiling + f32::EPSILON {
            return Ok(DecisionOutcome::Rejected {
                reason: "RoH_after exceeds neurorights ceiling (0.30)".into(),
            });
        }
        if proposal.roh_after > proposal.roh_before + f32::EPSILON {
            return Ok(DecisionOutcome::Rejected {
                reason: "RoH_after > RoH_before (no relaxation allowed)".into(),
            });
        }

        // 2. Envelope monotone: G_new ≥ G_old, D_new ≤ D_old. [file:3]
        if !proposal.envelopes.is_monotone() {
            return Ok(DecisionOutcome::Rejected {
                reason: "Envelope monotonicity violated (no loosening allowed)".into(),
            });
        }

        // 3. Neurorights / token kind: dream‑sensitive lifeforce changes require EVOLVE. [file:3][file:4]
        if self.neurorights.dream_state_sensitive
            && matches!(proposal.scope, Scope::LifeforceAlteration)
            && matches!(proposal.token_kind, TokenKind::Smart)
        {
            return Ok(DecisionOutcome::Rejected {
                reason: "LifeforceAlteration requires EVOLVE token under dream_state_sensitive neurorights".into(),
            });
        }

        // 4. Viability kernel: current swarm state must be inside Tsafe region. [file:3]
        if !self.viability_kernel.is_viable(swarm_state, lifeforce) {
            return Ok(DecisionOutcome::Rejected {
                reason: "Current swarm_state outside viability kernel".into(),
            });
        }

        // 5. CyberRank Tsafe action selection: require at least one safe candidate. [file:3]
        let candidates: Vec<CandidateAction> = Vec::new();
        let tsafe_choice =
            cybernano_vector_cyberrank::tsafe_select(&candidates, &self.rank_weights);
        if tsafe_choice.is_none() {
            return Ok(DecisionOutcome::Rejected {
                reason: "No Tsafe candidate action available for this update".into(),
            });
        }

        // 6. Log to .evolve.jsonl via EvolutionLogWriter (typed, append‑only). [file:3]
        self.evolvelog.append_from_proposal(&proposal)?;

        Ok(DecisionOutcome::Allowed)
    }

    /// Expose read‑only view of RoH ceiling for metrics and external guards. [file:3]
    pub fn roh_ceiling(&self) -> f32 {
        self.roh_ceiling
    }

    /// Expose underlying neurorights document (read‑only). [file:4]
    pub fn neurorights_policy(&self) -> &NeurorightsPolicyDocument {
        &self.neurorights_doc
    }
}
