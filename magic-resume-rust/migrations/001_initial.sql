-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    name TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create resumes table
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

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_resumes_user_id ON resumes(user_id);
CREATE INDEX IF NOT EXISTS idx_resumes_updated_at ON resumes(updated_at);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);