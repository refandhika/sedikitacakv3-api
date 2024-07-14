use actix_web::{post, get, delete, web, HttpResponse};
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use uuid::Uuid;

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::{DBPool, DBPooledConnection};

use crate::models::RoleDB;

// Role Request Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct RoleRequest {
    pub name: String,
    pub level: String,
    pub can_modify_user: bool,
    pub can_edit: bool,
    pub can_view: bool,
    pub is_guest: bool
}

impl RoleRequest {
    pub fn to_role_db(&self) -> Result<RoleDB, String> {
        Ok(RoleDB {
            id: 1,
            name: self.name.clone(),
            level: self.level.clone(),
            can_modify_user: self.can_modify_user.clone(),
            can_edit: self.can_edit.clone(),
            can_view: self.can_view.clone(),
            is_guest: self.is_guest.clone(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: None,
        })
    }
}

// Class Wide Function

fn create_role(role: RoleDB, conn: &mut DBPooledConnection) -> Result<RoleDB, Error> {
    use crate::schema::roles::dsl::*;
    diesel::insert_into(roles)
        .values((
            name.eq(role.name),
            level.eq(role.level),
            can_modify_user.eq(role.can_modify_user),
            can_edit.eq(role.can_edit),
            can_view.eq(role.can_view),
            is_guest.eq(role.is_guest),
            created_at.eq(role.created_at),
            updated_at.eq(role.updated_at),
            deleted_at.eq(role.deleted_at)
        ))
        .get_result(conn)
}

fn update_role(role: RoleDB, role_id: i32, conn: &mut DBPooledConnection) -> Result<RoleDB, Error> {
    use crate::schema::roles::dsl::*;
    diesel::update(roles.filter(id.eq(role_id)))
        .set((
            name.eq(role.name),
            level.eq(role.level),
            can_modify_user.eq(role.can_modify_user),
            can_edit.eq(role.can_edit),
            can_view.eq(role.can_view),
            is_guest.eq(role.is_guest),
            updated_at.eq(Utc::now().naive_utc())
        ))
        .get_result(conn)
}

// Routing

#[post("/role")]
pub async fn create(role_req: web::Json<RoleRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    match role_req.to_role_db() {
        Ok(role_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match create_role(role_db, &mut conn) {
                Ok(inserted_role) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(inserted_role),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error inserting role: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[post("/role/{id}")]
pub async fn update(path: web::Path<i32>, role_req: web::Json<RoleRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    let role_id = path.into_inner();
    match role_req.to_role_db() {
        Ok(role_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match update_role(role_db, role_id, &mut conn) {
                Ok(updated_role) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(updated_role),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error updating role: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[get("/role/{id}")]
pub async fn get(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let role_id = path.into_inner();

    use crate::schema::roles::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match roles.filter(id.eq(role_id)).first::<RoleDB>(&mut conn) {
        Ok(role) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(role.get_by_id()),
        Err(_) => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON)
            .json("Role not found"),
    }
}

#[delete("/role/{id}")]
pub async fn delete(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let role_id = path.into_inner();
    let current_time = Utc::now().naive_utc();

    use crate::schema::roles::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(roles.filter(id.eq(role_id)))
        .set(deleted_at.eq(Some(current_time)))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Role successfully deleted"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to delete role"})),
    }
}

#[post("/role/{id}/restore")]
pub async fn restore(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let role_id = path.into_inner();
    
    use crate::schema::roles::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(roles.filter(id.eq(role_id)))
        .set(deleted_at.eq(None::<NaiveDateTime>))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Role successfully restored"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to restore role"})),
    }
}