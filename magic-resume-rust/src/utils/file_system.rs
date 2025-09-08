use anyhow::Result;
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

pub async fn ensure_directory_exists(path: &str) -> Result<()> {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(path).await?;
    }
    Ok(())
}

pub fn generate_unique_filename(original_name: &str) -> String {
    let extension = Path::new(original_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    if extension.is_empty() {
        format!("{}", Uuid::new_v4())
    } else {
        format!("{}.{}", Uuid::new_v4(), extension)
    }
}

pub async fn save_file(directory: &str, filename: &str, content: &[u8]) -> Result<String> {
    ensure_directory_exists(directory).await?;
    
    let file_path = Path::new(directory).join(filename);
    fs::write(&file_path, content).await?;
    
    Ok(file_path.to_string_lossy().to_string())
}

pub async fn delete_file(file_path: &str) -> Result<()> {
    let path = Path::new(file_path);
    if path.exists() {
        fs::remove_file(path).await?;
    }
    Ok(())
}