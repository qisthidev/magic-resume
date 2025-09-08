mod config;
mod handlers;
mod models;
mod services;
mod utils;

use config::AppConfig;
use services::{AIService, DatabaseService, PdfService};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, Json, IntoResponse},
    routing::{get, post},
    Router,
};
use serde_json::json;
use tower_http::cors::CorsLayer;

// Combined state for all handlers
#[derive(Clone)]
struct AppState {
    pub db: Arc<DatabaseService>,
    pub pdf_service: Arc<PdfService>,
    pub ai_service: Arc<AIService>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = AppConfig::from_env()?;
    info!("Starting Magic Resume server on {}:{}", config.server.host, config.server.port);

    // Initialize services
    let db_service = Arc::new(DatabaseService::new(&config.database.url).await?);
    let pdf_service = Arc::new(PdfService::new());
    let ai_service = Arc::new(AIService::new(
        config.ai.openai_api_key.clone(),
        config.ai.openai_base_url.clone(),
    ));

    // Create combined app state
    let app_state = AppState {
        db: db_service.clone(),
        pdf_service: pdf_service.clone(),
        ai_service: ai_service.clone(),
    };

    // Create router with basic routes
    let app = Router::new()
        // Static files and index
        .route("/", get(serve_index))
        .route("/health", get(health_check))
        .route("/api/test", get(api_test))
        .route("/api/resumes", post(create_resume_simple))
        .route("/api/resumes", get(list_resumes_simple))
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    info!("Server starting on http://{}:{}", config.server.host, config.server.port);
    
    // Start server using the basic approach
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
        .await?;
    
    axum::serve(listener, app).await?;

    Ok(())
}

async fn serve_index() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Magic Resume - Rust Edition</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }
        .container { max-width: 800px; margin: 0 auto; }
        .header { text-align: center; margin-bottom: 40px; }
        .api-section { background: #f5f5f5; padding: 20px; margin: 20px 0; border-radius: 8px; }
        .endpoint { margin: 10px 0; }
        .method { display: inline-block; width: 80px; font-weight: bold; color: #007bff; }
        code { background: #e9ecef; padding: 2px 6px; border-radius: 3px; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>✨ Magic Resume - Rust Edition</h1>
            <p>A high-performance resume builder built with Rust and Axum</p>
        </div>
        
        <div class="api-section">
            <h2>🚀 API Endpoints</h2>
            <div class="endpoint">
                <span class="method">GET</span> <code>/health</code> - Health check
            </div>
            <div class="endpoint">
                <span class="method">GET</span> <code>/api/test</code> - API test endpoint
            </div>
            <div class="endpoint">
                <span class="method">POST</span> <code>/api/resumes</code> - Create a new resume
            </div>
            <div class="endpoint">
                <span class="method">GET</span> <code>/api/resumes</code> - List all resumes
            </div>
        </div>
        
        <div class="api-section">
            <h2>🛠️ Technology Stack</h2>
            <ul>
                <li><strong>Backend:</strong> Rust + Axum Framework</li>
                <li><strong>Database:</strong> SQLite with SQLx</li>
                <li><strong>Serialization:</strong> Serde JSON</li>
                <li><strong>Async Runtime:</strong> Tokio</li>
                <li><strong>Logging:</strong> Tracing</li>
                <li><strong>Configuration:</strong> Environment-based config</li>
            </ul>
        </div>
        
        <div class="api-section">
            <h2>📝 Features</h2>
            <ul>
                <li>✅ High-performance Rust backend</li>
                <li>✅ RESTful API design</li>
                <li>✅ Database integration</li>
                <li>✅ JSON serialization</li>
                <li>✅ CORS support</li>
                <li>✅ Structured logging</li>
                <li>🚧 PDF generation (placeholder)</li>
                <li>🚧 AI integration (placeholder)</li>
                <li>🚧 Frontend interface</li>
            </ul>
        </div>
        
        <div class="api-section">
            <h2>🔗 Quick Test</h2>
            <p>Test the API endpoints:</p>
            <ul>
                <li><a href="/health" target="_blank">Health Check</a></li>
                <li><a href="/api/test" target="_blank">API Test</a></li>
                <li><a href="/api/resumes" target="_blank">List Resumes</a></li>
            </ul>
        </div>
        
        <footer style="text-align: center; margin-top: 40px; padding: 20px; border-top: 1px solid #ddd;">
            <p>&copy; 2024 Magic Resume - Rust Edition. Built with ❤️ and Rust.</p>
        </footer>
    </div>
</body>
</html>
    "#)
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "Magic Resume API",
        "version": "1.0.0",
        "framework": "Axum",
        "language": "Rust"
    }))
}

async fn api_test() -> Json<serde_json::Value> {
    Json(json!({
        "message": "Magic Resume API is working!",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "endpoints": [
            "GET /health",
            "GET /api/test",
            "POST /api/resumes",
            "GET /api/resumes"
        ]
    }))
}

async fn create_resume_simple(
    State(_state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // For now, just echo back the received data with some metadata
    let response = json!({
        "message": "Resume creation endpoint (demo)",
        "received_data": payload,
        "id": uuid::Uuid::new_v4(),
        "created_at": chrono::Utc::now().to_rfc3339(),
        "status": "success"
    });
    
    Ok(Json(response))
}

async fn list_resumes_simple(
    State(_state): State<AppState>,
) -> Json<serde_json::Value> {
    // Return some sample resume data
    Json(json!({
        "resumes": [
            {
                "id": uuid::Uuid::new_v4(),
                "title": "Software Engineer Resume",
                "created_at": "2024-01-15T10:30:00Z",
                "updated_at": "2024-01-16T14:20:00Z"
            },
            {
                "id": uuid::Uuid::new_v4(),
                "title": "Product Manager Resume", 
                "created_at": "2024-01-10T09:15:00Z",
                "updated_at": "2024-01-12T16:45:00Z"
            }
        ],
        "total": 2,
        "message": "Sample resume data (demo mode)"
    }))
}