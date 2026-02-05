use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ApprovalRequest {
    pub upgrade_id: String,
}

#[derive(Debug, Serialize)]
pub struct ApprovalResponse {
    pub accepted: bool,
    pub reason: String,
}

pub fn router() -> Router {
    Router::new().route("/api/approval", post(handle_approval))
}

async fn handle_approval(Json(req): Json<ApprovalRequest>) -> Json<ApprovalResponse> {
    Json(ApprovalResponse {
        accepted: false,
        reason: format!("upgrade {} requires EvidenceBundle", req.upgrade_id),
    })
}
