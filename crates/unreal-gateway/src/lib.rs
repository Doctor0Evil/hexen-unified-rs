use serde::Serialize;
use warp::Filter;
use cybernano_viability_kernel::{SwarmState7D, LifeforceState, ViabilityKernel};
use cybernano_vector_cyberrank::{CandidateAction, RankWeights};

#[derive(Serialize)]
struct HudSnapshot {
    swarm_state: SwarmState7D,
    lifeforce: LifeforceState,
    kernel_mode: String,
    actions: Vec<CandidateAction>,
}

pub async fn serve_hud(
    kernel: ViabilityKernel,
) {
    let route = warp::path!("hud" / "snapshot")
        .map(move || {
            // in real code: read from shared state
            let state = SwarmState7D {
                intensity: 0.2,
                duty_cycle: 0.3,
                cumulative_load: 0.1,
                implant_power: 0.0,
                neuromod_amp: 0.0,
                cognitive_load: 0.4,
                legal_complexity: 0.1,
            };
            let lifeforce = LifeforceState {
                cy: 0.9,
                zen: 0.8,
                chi: 0.95,
                integrity: 0.97,
            };
            let actions: Vec<CandidateAction> = Vec::new();
            let snapshot = HudSnapshot {
                swarm_state: state,
                lifeforce,
                kernel_mode: kernel.mode_id.clone(),
                actions,
            };
            warp::reply::json(&snapshot)
        });

    warp::serve(route).run(([0, 0, 0, 0], 8100)).await;
}
