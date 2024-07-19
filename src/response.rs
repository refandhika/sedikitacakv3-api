use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub message: String
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub birth: Option<DateTime<Utc>>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponseWithoutPass {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub birth: Option<DateTime<Utc>>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub content: String,
    pub tags: Option<String>,
    pub author_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub published: bool,
    pub category_id: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PostCatResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    //pub tech_list_id: Option<i32>,
    pub source: Option<String>,
    pub url: Option<String>,
    pub demo: Option<String>,
    pub relevant: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub published: bool
}

#[derive(Debug, Serialize)]
pub struct TechResponse {
    pub id: i32,
    pub title: String,
    pub icon: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>
}

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub id: i32,
    pub name: String,
    pub level: String,
    pub can_modify_user: bool,
    pub can_edit: bool,
    pub can_view: bool,
    pub is_guest: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct HobbyResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub image: Option<String>,
    pub item_order: i32,
    pub active: bool,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct SettingResponse {
    pub id: i32,
    pub param: String,
    pub value: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}