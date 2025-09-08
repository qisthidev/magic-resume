use axum::{
    http::{header, StatusCode},
    response::{Html, IntoResponse},
};
use std::path::Path;
use tokio::fs;

pub async fn serve_index() -> impl IntoResponse {
    match fs::read_to_string("static/index.html").await {
        Ok(contents) => Html(contents).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Index file not found").into_response(),
    }
}

pub async fn serve_static_file(uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path();
    
    // Remove leading slash and "static/" prefix
    let file_path = if path.starts_with("/static/") {
        &path[8..] // Remove "/static/"
    } else {
        &path[1..] // Remove leading "/"
    };
    
    let full_path = Path::new("static").join(file_path);
    
    // Security check: ensure the path is within the static directory
    let canonical_static = match fs::canonicalize("static").await {
        Ok(path) => path,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Static directory not found").into_response(),
    };
    
    let canonical_file = match fs::canonicalize(&full_path).await {
        Ok(path) => path,
        Err(_) => return (StatusCode::NOT_FOUND, "File not found").into_response(),
    };

    if !canonical_file.starts_with(canonical_static) {
        return (StatusCode::FORBIDDEN, "Access denied").into_response();
    }

    match fs::read(&full_path).await {
        Ok(contents) => {
            let content_type = get_content_type(&full_path);
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, content_type)],
                contents,
            )
                .into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

fn get_content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("json") => "application/json",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("woff") => "font/woff",
        Some("woff2") => "font/woff2",
        Some("ttf") => "font/ttf",
        Some("pdf") => "application/pdf",
        _ => "application/octet-stream",
    }
}