use crate::models::{ResumeData, User, CreateResumeRequest, UpdateResumeRequest, CreateUserRequest};
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone)]
pub struct DatabaseService {
    pool: SqlitePool,
}

impl DatabaseService {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Create tables manually since we don't have migrations set up yet
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                name TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS resumes (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                title TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                template_id TEXT,
                basic TEXT NOT NULL DEFAULT '{}',
                education TEXT NOT NULL DEFAULT '[]',
                experience TEXT NOT NULL DEFAULT '[]',
                projects TEXT NOT NULL DEFAULT '[]',
                custom_data TEXT NOT NULL DEFAULT '{}',
                skill_content TEXT NOT NULL DEFAULT '',
                active_section TEXT NOT NULL DEFAULT 'basic',
                dragging_project_id TEXT,
                menu_sections TEXT NOT NULL DEFAULT '[]',
                global_settings TEXT NOT NULL DEFAULT '{}',
                FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
            );
            "#,
        )
        .execute(&pool)
        .await?;

        // Create indexes
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_resumes_user_id ON resumes(user_id);")
            .execute(&pool)
            .await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_resumes_updated_at ON resumes(updated_at);")
            .execute(&pool)
            .await?;
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);")
            .execute(&pool)
            .await?;
        
        Ok(Self { pool })
    }

    // User operations
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO users (id, email, name, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(&request.email)
        .bind(&request.name)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(User {
            id,
            email: request.email,
            name: request.name,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        let row = sqlx::query(
            "SELECT id, email, name, created_at, updated_at FROM users WHERE id = ?"
        )
        .bind(user_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = row {
            let id: String = row.get("id");
            let email: String = row.get("email");
            let name: Option<String> = row.get("name");
            let created_at: String = row.get("created_at");
            let updated_at: String = row.get("updated_at");

            Ok(Some(User {
                id: Uuid::parse_str(&id)?,
                email,
                name,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Utc),
            }))
        } else {
            Ok(None)
        }
    }

    // Resume operations
    pub async fn create_resume(&self, user_id: Uuid, request: CreateResumeRequest) -> Result<ResumeData> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        // Create default data
        let default_basic = serde_json::json!({
            "birth_date": "",
            "name": "",
            "title": "",
            "email": "",
            "phone": "",
            "location": "",
            "icons": {},
            "employment_status": "",
            "photo": "",
            "photo_config": {
                "width": 90,
                "height": 120,
                "aspect_ratio": "1:1",
                "border_radius": "none",
                "custom_border_radius": 0,
                "visible": true
            },
            "field_order": [],
            "custom_fields": [],
            "github_key": "",
            "github_username": "",
            "github_contributions_visible": false,
            "layout": "left"
        });

        sqlx::query(
            r#"
            INSERT INTO resumes (
                id, user_id, title, created_at, updated_at, template_id,
                basic, education, experience, projects, custom_data,
                skill_content, active_section, dragging_project_id,
                menu_sections, global_settings
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(user_id.to_string())
        .bind(&request.title)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(&request.template_id)
        .bind(serde_json::to_string(&default_basic)?)
        .bind("[]")
        .bind("[]")
        .bind("[]")
        .bind("{}")
        .bind("")
        .bind("basic")
        .bind(None::<String>)
        .bind("[]")
        .bind("{}")
        .execute(&self.pool)
        .await?;

        Ok(ResumeData {
            id,
            title: request.title,
            created_at: now,
            updated_at: now,
            template_id: request.template_id,
            basic: default_basic,
            education: serde_json::json!([]),
            experience: serde_json::json!([]),
            projects: serde_json::json!([]),
            custom_data: serde_json::json!({}),
            skill_content: "".to_string(),
            active_section: "basic".to_string(),
            dragging_project_id: None,
            menu_sections: serde_json::json!([]),
            global_settings: serde_json::json!({}),
        })
    }

    pub async fn get_resume_by_id(&self, resume_id: Uuid) -> Result<Option<ResumeData>> {
        let row = sqlx::query("SELECT * FROM resumes WHERE id = ?")
            .bind(resume_id.to_string())
            .fetch_optional(&self.pool)
            .await?;

        if let Some(row) = row {
            let id: String = row.get("id");
            let title: String = row.get("title");
            let created_at: String = row.get("created_at");
            let updated_at: String = row.get("updated_at");
            let template_id: Option<String> = row.get("template_id");
            let basic: String = row.get("basic");
            let education: String = row.get("education");
            let experience: String = row.get("experience");
            let projects: String = row.get("projects");
            let custom_data: String = row.get("custom_data");
            let skill_content: String = row.get("skill_content");
            let active_section: String = row.get("active_section");
            let dragging_project_id: Option<String> = row.get("dragging_project_id");
            let menu_sections: String = row.get("menu_sections");
            let global_settings: String = row.get("global_settings");

            Ok(Some(ResumeData {
                id: Uuid::parse_str(&id)?,
                title,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Utc),
                template_id,
                basic: serde_json::from_str(&basic)?,
                education: serde_json::from_str(&education)?,
                experience: serde_json::from_str(&experience)?,
                projects: serde_json::from_str(&projects)?,
                custom_data: serde_json::from_str(&custom_data)?,
                skill_content,
                active_section,
                dragging_project_id,
                menu_sections: serde_json::from_str(&menu_sections)?,
                global_settings: serde_json::from_str(&global_settings)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_resumes(&self, user_id: Uuid) -> Result<Vec<ResumeData>> {
        let rows = sqlx::query("SELECT * FROM resumes WHERE user_id = ? ORDER BY updated_at DESC")
            .bind(user_id.to_string())
            .fetch_all(&self.pool)
            .await?;

        let mut resumes = Vec::new();
        for row in rows {
            let id: String = row.get("id");
            let title: String = row.get("title");
            let created_at: String = row.get("created_at");
            let updated_at: String = row.get("updated_at");
            let template_id: Option<String> = row.get("template_id");
            let basic: String = row.get("basic");
            let education: String = row.get("education");
            let experience: String = row.get("experience");
            let projects: String = row.get("projects");
            let custom_data: String = row.get("custom_data");
            let skill_content: String = row.get("skill_content");
            let active_section: String = row.get("active_section");
            let dragging_project_id: Option<String> = row.get("dragging_project_id");
            let menu_sections: String = row.get("menu_sections");
            let global_settings: String = row.get("global_settings");

            resumes.push(ResumeData {
                id: Uuid::parse_str(&id)?,
                title,
                created_at: chrono::DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at)?.with_timezone(&Utc),
                template_id,
                basic: serde_json::from_str(&basic)?,
                education: serde_json::from_str(&education)?,
                experience: serde_json::from_str(&experience)?,
                projects: serde_json::from_str(&projects)?,
                custom_data: serde_json::from_str(&custom_data)?,
                skill_content,
                active_section,
                dragging_project_id,
                menu_sections: serde_json::from_str(&menu_sections)?,
                global_settings: serde_json::from_str(&global_settings)?,
            });
        }

        Ok(resumes)
    }

    pub async fn update_resume(&self, resume_id: Uuid, request: UpdateResumeRequest) -> Result<Option<ResumeData>> {
        let now = Utc::now();

        // Get current resume
        let current = match self.get_resume_by_id(resume_id).await? {
            Some(resume) => resume,
            None => return Ok(None),
        };

        // Update fields
        let title = request.title.unwrap_or(current.title);
        let template_id = request.template_id.or(current.template_id);
        let basic = request.basic.map(|b| serde_json::to_value(b).unwrap()).unwrap_or(current.basic);
        let education = request.education.map(|e| serde_json::to_value(e).unwrap()).unwrap_or(current.education);
        let experience = request.experience.map(|e| serde_json::to_value(e).unwrap()).unwrap_or(current.experience);
        let projects = request.projects.map(|p| serde_json::to_value(p).unwrap()).unwrap_or(current.projects);
        let custom_data = request.custom_data.map(|c| serde_json::to_value(c).unwrap()).unwrap_or(current.custom_data);
        let skill_content = request.skill_content.unwrap_or(current.skill_content);
        let active_section = request.active_section.unwrap_or(current.active_section);
        let dragging_project_id = request.dragging_project_id.or(current.dragging_project_id);
        let menu_sections = request.menu_sections.map(|m| serde_json::to_value(m).unwrap()).unwrap_or(current.menu_sections);
        let global_settings = request.global_settings.map(|g| serde_json::to_value(g).unwrap()).unwrap_or(current.global_settings);

        sqlx::query(
            r#"
            UPDATE resumes SET
                title = ?, template_id = ?, basic = ?, education = ?,
                experience = ?, projects = ?, custom_data = ?,
                skill_content = ?, active_section = ?, dragging_project_id = ?,
                menu_sections = ?, global_settings = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&title)
        .bind(&template_id)
        .bind(serde_json::to_string(&basic)?)
        .bind(serde_json::to_string(&education)?)
        .bind(serde_json::to_string(&experience)?)
        .bind(serde_json::to_string(&projects)?)
        .bind(serde_json::to_string(&custom_data)?)
        .bind(&skill_content)
        .bind(&active_section)
        .bind(&dragging_project_id)
        .bind(serde_json::to_string(&menu_sections)?)
        .bind(serde_json::to_string(&global_settings)?)
        .bind(now.to_rfc3339())
        .bind(resume_id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(Some(ResumeData {
            id: resume_id,
            title,
            created_at: current.created_at,
            updated_at: now,
            template_id,
            basic,
            education,
            experience,
            projects,
            custom_data,
            skill_content,
            active_section,
            dragging_project_id,
            menu_sections,
            global_settings,
        }))
    }

    pub async fn delete_resume(&self, resume_id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM resumes WHERE id = ?")
            .bind(resume_id.to_string())
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}