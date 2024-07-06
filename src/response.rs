use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: DateTime<Utc>,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub birth: DateTime<Utc>,
    pub linkedin: String,
    pub github: String,
    pub password: String,
}
