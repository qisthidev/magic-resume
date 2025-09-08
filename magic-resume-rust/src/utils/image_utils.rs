use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use image::ImageFormat;
use std::io::Cursor;

pub fn resize_image(image_data: &[u8], max_width: u32, max_height: u32) -> Result<Vec<u8>> {
    let img = image::load_from_memory(image_data)?;
    let resized = img.resize(max_width, max_height, image::imageops::FilterType::Lanczos3);
    
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    resized.write_to(&mut cursor, image::ImageFormat::Png)?;
    
    Ok(buffer)
}

pub fn convert_to_base64(image_data: &[u8]) -> String {
    general_purpose::STANDARD.encode(image_data)
}

pub fn decode_base64(base64_data: &str) -> Result<Vec<u8>> {
    let cleaned = if base64_data.starts_with("data:") {
        // Remove data URL prefix (e.g., "data:image/png;base64,")
        base64_data.split(',').nth(1).unwrap_or(base64_data)
    } else {
        base64_data
    };
    
    Ok(general_purpose::STANDARD.decode(cleaned)?)
}

pub fn get_image_format(image_data: &[u8]) -> Result<ImageFormat> {
    image::guess_format(image_data).map_err(|e| anyhow::anyhow!("Failed to detect image format: {}", e))
}

pub fn is_supported_image_format(format: &ImageFormat) -> bool {
    matches!(format, ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::WebP)
}