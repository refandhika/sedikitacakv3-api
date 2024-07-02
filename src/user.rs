use actix_web::{post, get, delete, dev, web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::constants::APPLICATION_JSON;
use crate::response::Response;

pub type Users = dev::Response<User>;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            name,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {
    pub message: Option<String>,
}

impl UserRequest {
    pub fn to_create(&self) -> Option<User> {
        match &self.message {
            Some(message) => Some(User::new(message.to_string())),
            None => None,
        }
    }
}

#[post("/user")]
pub async fn create(user_req: web::Json<UserRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(user_req.to_create())
}

#[post("/user/{id}")]
pub async fn update(user_req: web::Json<UserRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(user_req.to_create())
}


#[get("/user/{id}")]
pub async fn get(path: web::Path<(String,)>) -> HttpResponse {
    let found_user: Option<User> = None;

    match found_user {
        Some(user) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(user),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

#[delete("/user/{id}")]
pub async fn delete(path: web::Path<(String,)>) -> HttpResponse {
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
