use actix_web::{post, get, delete, web, HttpResponse};
use chrono::{Utc, NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, Queryable, PgTextExpressionMethods, BoolExpressionMethods};
use bcrypt::{hash, DEFAULT_COST};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR, USER_BIRTH_NOTFOUND};
use crate::{DBPool, DBPooledConnection};

use crate::models::UserDB;
use crate::models::RoleDB;

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
    pub role_id: i32
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
            password: hash_password,
            role_id: self.role_id.clone()
        })
    }
}

#[derive(Queryable, Debug, Serialize)]
pub struct JoinedUser {
    #[serde(flatten)]
    pub user: UserDB,
    pub role: RoleDB
}

// Pagination Request Struct
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub search: Option<String>
}

// Class Wide Function

fn create_user(user: UserDB, conn: &mut DBPooledConnection) -> Result<UserDB, Error> {
    use crate::schema::users::dsl::*;
    diesel::insert_into(users).values(&user).get_result(conn)
}

fn update_user(user: UserDB, user_id: Uuid, conn: &mut DBPooledConnection) -> Result<UserDB, Error> {
    use crate::schema::users::dsl::*;
    diesel::update(users.filter(id.eq(user_id)))
        .set((
            name.eq(user.name),
            email.eq(user.email),
            phone.eq(user.phone),
            birth.eq(user.birth),
            linkedin.eq(user.linkedin),
            github.eq(user.github),
            password.eq(user.password),
            updated_at.eq(Utc::now().naive_utc()),
        ))
        .get_result(conn)
}

fn all_user_with_pagination(page: i32, limit: i32, search: String, conn: &mut DBPooledConnection) -> Result<Vec<UserDB>, Error> {
    use crate::schema::users::dsl::*;
    let mut query = users
        .filter(deleted_at.is_null())
        .order_by(id.desc())
        .limit(limit as i64)
        .offset(((page - 1) * limit) as i64)
        .into_boxed();
    
    if !search.is_empty() {
        query = query.filter(
            name.ilike(format!("%{}%", search))
                .or(email.ilike(format!("%{}%", search)))
        );
    }

    query.load::<UserDB>(conn)
}

fn get_single_user(user_id: Uuid, conn: &mut DBPooledConnection) -> Result<JoinedUser, Error> {
    use crate::schema::users::dsl::*;
    use crate::schema::roles::dsl::{roles};

    users
        .inner_join(roles)
        .filter(id.eq(user_id))
        .limit(1)
        .get_result(conn)
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
pub async fn update(path: web::Path<String>, user_req: web::Json<UserRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    let user_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid UUID format"),
    };
    match user_req.to_user_db() {
        Ok(user_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match update_user(user_db, user_id, &mut conn) {
                Ok(updated_user) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(updated_user),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error updating user: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[get("/user/{id}")]
pub async fn get(path: web::Path<String>, pool: web::Data<DBPool>) -> HttpResponse {
    let user_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid UUID format"),
    };
    
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match get_single_user(user_id, &mut conn) {
        Ok(user) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(user),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "User not found"})),
    }
}

#[delete("/user/{id}")]
pub async fn delete(path: web::Path<String>, pool: web::Data<DBPool>) -> HttpResponse {
    let user_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid UUID format"),
    };
    let current_time = Utc::now().naive_utc();

    use crate::schema::users::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(users.filter(id.eq(user_id)))
        .set(deleted_at.eq(Some(current_time)))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "User successfully deleted"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to delete user"})),
    }
}

#[post("/user/{id}/restore")]
pub async fn restore(path: web::Path<String>, pool: web::Data<DBPool>) -> HttpResponse {
    let user_id = match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid UUID format"),
    };
    
    use crate::schema::users::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(users.filter(id.eq(user_id)))
        .set(deleted_at.eq(None::<NaiveDateTime>))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "User successfully restored"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to restore user"})),
    }
}

#[get("/users")]
pub async fn all(query: web::Query<PaginationParams>, pool: web::Data<DBPool>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    let search = query.search.clone().unwrap_or("".to_string());

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match all_user_with_pagination(page, limit, search, &mut conn) {
        Ok(users) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(users),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to retrieve users"})),
    }
}