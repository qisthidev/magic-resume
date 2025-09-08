use crate::models::{CreateResumeRequest, UpdateResumeRequest};
use crate::services::{DatabaseService, PdfService};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseService>,
    pub pdf_service: Arc<PdfService>,
}

pub async fn create_resume(
    State(state): State<AppState>,
    Json(request): Json<CreateResumeRequest>,
) -> impl IntoResponse {
    // For now, use a dummy user ID. In a real app, extract from auth token
    let user_id = Uuid::new_v4();

    match state.db.create_resume(user_id, request).await {
        Ok(resume) => (StatusCode::CREATED, Json(resume)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to create resume",
                "message": e.to_string()
            })),
        )
            .into_response(),
    }
}

pub async fn get_resume(
    State(state): State<AppState>,
    Path(resume_id): Path<String>,
) -> impl IntoResponse {
    let resume_id = match Uuid::parse_str(&resume_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid resume ID"})),
            )
                .into_response()
        }
    };

    match state.db.get_resume_by_id(resume_id).await {
        Ok(Some(resume)) => (StatusCode::OK, Json(resume)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Resume not found"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to get resume",
                "message": e.to_string()
            })),
        )
            .into_response(),
    }
}

pub async fn update_resume(
    State(state): State<AppState>,
    Path(resume_id): Path<String>,
    Json(request): Json<UpdateResumeRequest>,
) -> impl IntoResponse {
    let resume_id = match Uuid::parse_str(&resume_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid resume ID"})),
            )
                .into_response()
        }
    };

    match state.db.update_resume(resume_id, request).await {
        Ok(Some(resume)) => (StatusCode::OK, Json(resume)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Resume not found"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to update resume",
                "message": e.to_string()
            })),
        )
            .into_response(),
    }
}

pub async fn delete_resume(
    State(state): State<AppState>,
    Path(resume_id): Path<String>,
) -> impl IntoResponse {
    let resume_id = match Uuid::parse_str(&resume_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid resume ID"})),
            )
                .into_response()
        }
    };

    match state.db.delete_resume(resume_id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Resume not found"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to delete resume",
                "message": e.to_string()
            })),
        )
            .into_response(),
    }
}

pub async fn export_resume_pdf(
    State(state): State<AppState>,
    Path(resume_id): Path<String>,
) -> impl IntoResponse {
    let resume_id = match Uuid::parse_str(&resume_id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid resume ID"})),
            )
                .into_response()
        }
    };

    let resume = match state.db.get_resume_by_id(resume_id).await {
        Ok(Some(resume)) => resume,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Resume not found"})),
            )
                .into_response()
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to get resume",
                    "message": e.to_string()
                })),
            )
                .into_response()
        }
    };

    match state.pdf_service.generate_resume_pdf(&resume).await {
        Ok(pdf_bytes) => {
            let headers = [
                ("Content-Type", "application/pdf"),
                (
                    "Content-Disposition",
                    &format!("attachment; filename=\"{}.pdf\"", resume.title),
                ),
            ];
            (StatusCode::OK, headers, pdf_bytes).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to generate PDF",
                "message": e.to_string()
            })),
        )
            .into_response(),
    }
}

pub async fn list_user_resumes(State(state): State<AppState>) -> impl IntoResponse {
    // For now, use a dummy user ID. In a real app, extract from auth token
    let user_id = Uuid::new_v4();

    match state.db.get_user_resumes(user_id).await {
        Ok(resumes) => (StatusCode::OK, Json(resumes)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to list resumes",
                "message": e.to_string()
            })),
        )
            .into_response(),
    }
}