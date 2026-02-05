use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Scope {
    DayToDayTuning,
    ArchChange,
    LifeforceAlteration,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenKind {
    Smart,
    Evolve,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EffectBounds {
    pub l2_delta_norm: f32,
    pub irreversible: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvelopeBounds {
    pub g_old: f32,
    pub g_new: f32,
    pub d_old: f32,
    pub d_new: f32,
}

impl EnvelopeBounds {
    pub fn is_monotone(&self) -> bool {
        self.g_new + f32::EPSILON >= self.g_old && self.d_new <= self.d_old + f32::EPSILON
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateProposal {
    pub proposal_id: String,
    pub subject_id: String,
    pub scope: Scope,
    pub token_kind: TokenKind,
    pub effect_bounds: EffectBounds,
    pub roh_before: f32,
    pub roh_after: f32,
    pub envelopes: EnvelopeBounds,
    pub evidence_bundle_ref: String, // points into CRD / donutloop
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuroRightsPolicy {
    pub id: String,
    pub noncommercial_neural_data: bool,
    pub dream_state_sensitive: bool,
    pub forbid_decision_use: Vec<String>,
    pub roh_ceiling: f32, // must be 0.30
}
