use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankVector {
    pub safety: f32,
    pub legal: f32,
    pub biomech: f32,
    pub psych: f32,
    pub rollback: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CandidateAction {
    pub id: String,
    pub rank: RankVector,
    pub is_viable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RankWeights {
    pub safety: f32,
    pub legal: f32,
    pub biomech: f32,
    pub psych: f32,
    pub rollback: f32,
}

impl RankWeights {
    pub fn score(&self, r: &RankVector) -> f32 {
        self.safety * r.safety
            + self.legal * r.legal
            + self.biomech * r.biomech
            + self.psych * r.psych
            + self.rollback * r.rollback
    }
}

pub fn tsafe_select(
    candidates: &[CandidateAction],
    weights: &RankWeights,
) -> Option<CandidateAction> {
    candidates
        .iter()
        .filter(|c| c.is_viable)
        .max_by(|a, b| {
            let sa = weights.score(&a.rank);
            let sb = weights.score(&b.rank);
            sa.partial_cmp(&sb).unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned()
}
