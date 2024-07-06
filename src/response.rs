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
