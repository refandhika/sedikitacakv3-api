use actix_web::{post, get, delete, web, HttpResponse};
use chrono::{Utc, NaiveDate};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::result::Error;
use diesel::RunQueryDsl;
use bcrypt::{hash, DEFAULT_COST};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR, USER_BIRTH_NOTFOUND};
use crate::{DBPool, DBPooledConnection};

use crate::models::UserDB;
use crate::response::UserResponse;

// User Request Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub birth: String,
    pub linkedin: Option<String>,
    pub github: Option<String>,
}

impl UserRequest {
    pub fn to_user_db(&self) -> Result<UserDB, String> {
        let birth_date = match NaiveDate::parse_from_str(&self.birth, "%Y-%m-%d") {
            Ok(date) => Some(date),
            Err(_) => return Err("Invalid date format".to_string())
        };
        let hash_password = hash(&self.password, DEFAULT_COST).expect("Failed to hash password");

        Ok(UserDB {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: None,
            name: self.name.clone(),
            email: self.email.clone(),
            phone: Some(self.phone.clone()),
            birth: Some(birth_date.expect(USER_BIRTH_NOTFOUND)),
            linkedin: Some(self.linkedin.clone().unwrap_or("".to_string())),
            github: Some(self.github.clone().unwrap_or("".to_string())),
            password: hash_password
        })
    }
}

// Class Wide Function

fn create_user(user: UserDB, conn: &mut DBPooledConnection) -> Result<UserDB, Error> {
    use crate::schema::users::dsl::*;
    diesel::insert_into(users).values(&user).get_result(conn)
}

// Routing

#[post("/user")]
pub async fn create(user_req: web::Json<UserRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    match user_req.to_user_db() {
        Ok(user_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match create_user(user_db, &mut conn) {
                Ok(inserted_user) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(inserted_user),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error inserting user: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[post("/user/{id}")]
pub async fn update(user_req: web::Json<UserRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(user_req.to_user_db())
}

#[get("/user/{id}")]
pub async fn get(_path: web::Path<(String,)>) -> HttpResponse {
    let found_user: Option<UserResponse> = None;

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
pub async fn delete(_path: web::Path<(String,)>) -> HttpResponse {
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}