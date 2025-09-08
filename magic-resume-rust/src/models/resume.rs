use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// use sqlx::FromRow; // Removed for now due to macro issues
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoConfig {
    pub width: i32,
    pub height: i32,
    pub aspect_ratio: String, // "1:1" | "4:3" | "3:4" | "16:9" | "custom"
    pub border_radius: String, // "none" | "medium" | "full" | "custom"
    pub custom_border_radius: i32,
    pub visible: Option<bool>,
}

impl Default for PhotoConfig {
    fn default() -> Self {
        Self {
            width: 90,
            height: 120,
            aspect_ratio: "1:1".to_string(),
            border_radius: "none".to_string(),
            custom_border_radius: 0,
            visible: Some(true),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicFieldType {
    pub id: String,
    pub key: String,
    pub label: String,
    pub field_type: Option<String>, // "date" | "textarea" | "text" | "editor"
    pub visible: bool,
    pub custom: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldType {
    pub id: String,
    pub label: String,
    pub value: String,
    pub icon: Option<String>,
    pub visible: Option<bool>,
    pub custom: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicInfo {
    pub birth_date: String,
    pub name: String,
    pub title: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub icons: HashMap<String, String>,
    pub employment_status: String,
    pub photo: String,
    pub photo_config: PhotoConfig,
    pub field_order: Option<Vec<BasicFieldType>>,
    pub custom_fields: Vec<CustomFieldType>,
    pub github_key: String,
    pub github_username: String,
    pub github_contributions_visible: bool,
    pub layout: Option<String>, // "left" | "center" | "right"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub id: Uuid,
    pub school: String,
    pub major: String,
    pub degree: String,
    pub start_date: String,
    pub end_date: String,
    pub gpa: Option<String>,
    pub description: Option<String>,
    pub visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub id: Uuid,
    pub company: String,
    pub position: String,
    pub date: String,
    pub details: String,
    pub visible: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub role: String,
    pub date: String,
    pub description: String,
    pub visible: bool,
    pub link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalSettings {
    pub theme_color: Option<String>,
    pub font_family: Option<String>,
    pub base_font_size: Option<i32>,
    pub page_padding: Option<i32>,
    pub paragraph_spacing: Option<i32>,
    pub line_height: Option<f32>,
    pub section_spacing: Option<i32>,
    pub header_size: Option<i32>,
    pub subheader_size: Option<i32>,
    pub use_icon_mode: Option<bool>,
    pub center_subtitle: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeTheme {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomItem {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub date_range: String,
    pub description: String,
    pub visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuSection {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub enabled: bool,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeData {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub template_id: Option<String>,
    pub basic: serde_json::Value, // JSON field for BasicInfo
    pub education: serde_json::Value, // JSON field for Vec<Education>
    pub experience: serde_json::Value, // JSON field for Vec<Experience>
    pub projects: serde_json::Value, // JSON field for Vec<Project>
    pub custom_data: serde_json::Value, // JSON field for HashMap<String, Vec<CustomItem>>
    pub skill_content: String,
    pub active_section: String,
    pub dragging_project_id: Option<String>,
    pub menu_sections: serde_json::Value, // JSON field for Vec<MenuSection>
    pub global_settings: serde_json::Value, // JSON field for GlobalSettings
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResumeRequest {
    pub title: String,
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResumeRequest {
    pub title: Option<String>,
    pub template_id: Option<String>,
    pub basic: Option<BasicInfo>,
    pub education: Option<Vec<Education>>,
    pub experience: Option<Vec<Experience>>,
    pub projects: Option<Vec<Project>>,
    pub custom_data: Option<HashMap<String, Vec<CustomItem>>>,
    pub skill_content: Option<String>,
    pub active_section: Option<String>,
    pub dragging_project_id: Option<String>,
    pub menu_sections: Option<Vec<MenuSection>>,
    pub global_settings: Option<GlobalSettings>,
}

pub const THEME_COLORS: &[&str] = &[
    "#000000", "#1A1A1A", "#333333", "#4D4D4D", "#666666", "#808080", "#999999",
    "#0047AB", "#8B0000", "#FF4500", "#4B0082", "#2E8B57",
];