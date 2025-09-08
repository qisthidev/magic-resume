use crate::services::{AIService, GrammarCheckRequest, PolishRequest};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;
use std::sync::Arc;

#[derive(Clone)]
pub struct AIState {
    pub ai_service: Arc<AIService>,
}

pub async fn check_grammar(
    State(state): State<AIState>,
    Json(request): Json<GrammarCheckRequest>,
) -> impl IntoResponse {
    match state.ai_service.check_grammar(request).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Grammar check failed",
                "message": e.to_string()
            })),
        )
            .into_response(),
    }
}

pub async fn polish_text(
    State(state): State<AIState>,
    Json(request): Json<PolishRequest>,
) -> impl IntoResponse {
    match state.ai_service.polish_text(request).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Text polishing failed",
                "message": e.to_string()
            })),
        )
            .into_response(),
    }
}