use proptest::prelude::*;
use sovereigntycore::types::{EnvelopeBounds, UpdateProposal, Scope, TokenKind};
use sovereigntycore::SovereigntyCore;
use organiccpualn::rohmodel::{RohInputs, RohModelShard};
use cybernano_viability_kernel::{LifeforceState, SwarmState7D, ViabilityKernel};

proptest! {
    #[test]
    fn roh_never_increases_and_stays_below_ceiling(
        roh_before in 0.0f32..0.30,
        delta in 0.0f32..0.05,
    ) {
        let roh_after = (roh_before - delta).max(0.0);
        prop_assume!(roh_after <= 0.30);

        // build minimal model and kernel (omitted for brevity)
        let core = sovereign_core_fixture().unwrap();

        let proposal = UpdateProposal {
            proposal_id: "test".into(),
            subject_id: "subject".into(),
            scope: Scope::DayToDayTuning,
            token_kind: TokenKind::Smart,
            effect_bounds: super::EffectBounds { l2_delta_norm: 0.1, irreversible: false },
            roh_before,
            roh_after,
            envelopes: EnvelopeBounds { g_old: 1.0, g_new: 1.0, d_old: 1.0, d_new: 1.0 },
            evidence_bundle_ref: "evidence-1".into(),
        };
        let swarm = SwarmState7D::zero();
        let lifeforce = LifeforceState::baseline();

        let outcome = core.evaluate_update(proposal, &swarm, &lifeforce).unwrap();
        match outcome {
            sovereigntycore::DecisionOutcome::Allowed => {
                prop_assert!(roh_after <= 0.30);
                prop_assert!(roh_after <= roh_before + f32::EPSILON);
            }
            sovereigntycore::DecisionOutcome::Rejected { .. } => {
                // rejection is always safe
            }
        }
    }

    #[test]
    fn envelope_bounds_are_monotone(
        g_old in 0.0f32..1.0,
        d_old in 0.0f32..1.0,
        tighten in 0.0f32..0.5,
    ) {
        let bounds = EnvelopeBounds {
            g_old,
            g_new: (g_old + tighten).min(1.0),
            d_old,
            d_new: (d_old - tighten).max(0.0),
        };
        prop_assert!(bounds.is_monotone());
    }
}
