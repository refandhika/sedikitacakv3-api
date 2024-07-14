use actix_web::{post, get, delete, web, HttpResponse};
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::{DBPool, DBPooledConnection};

use crate::models::TechDB;

// Post Request Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct TechRequest {
    pub title: String,
    pub icon: Option<String>
}

impl TechRequest {
    pub fn to_tech_db(&self) -> Result<TechDB, String> {
        Ok(TechDB {
            id: 1,
            title: self.title.clone(),
            icon: self.icon.clone(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: None
        })
    }
}

// Class Wide Function

fn create_tech(tech: TechDB, conn: &mut DBPooledConnection) -> Result<TechDB, Error> {
    use crate::schema::techs::dsl::*;
    diesel::insert_into(techs)
        .values((
            title.eq(tech.title),
            icon.eq(tech.icon),
            created_at.eq(tech.created_at),
            updated_at.eq(tech.updated_at),
            deleted_at.eq(tech.deleted_at)
        ))
        .get_result(conn)
}

fn update_tech(tech: TechDB, tech_id: i32, conn: &mut DBPooledConnection) -> Result<TechDB, Error> {
    use crate::schema::techs::dsl::*;
    diesel::update(techs.filter(id.eq(tech_id)))
        .set((
            title.eq(tech.title),
            icon.eq(tech.icon),
            updated_at.eq(Utc::now().naive_utc())
        ))
        .get_result(conn)
}

// Routing

#[post("/tech")]
pub async fn create(tech_req: web::Json<TechRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    match tech_req.to_tech_db() {
        Ok(tech_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match create_tech(tech_db, &mut conn) {
                Ok(inserted_tech) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(inserted_tech),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error inserting tech: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[post("/tech/{id}")]
pub async fn update(path: web::Path<i32>, tech_req: web::Json<TechRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    let tech_id = path.into_inner();
    match tech_req.to_tech_db() {
        Ok(tech_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match update_tech(tech_db, tech_id, &mut conn) {
                Ok(updated_tech) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(updated_tech),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error updating tech: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[get("/tech/{id}")]
pub async fn get(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let tech_id = path.into_inner();

    use crate::schema::techs::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match techs.filter(id.eq(tech_id)).first::<TechDB>(&mut conn) {
        Ok(tech) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(tech.get_by_id()),
        Err(_) => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON)
            .json("Tech not found"),
    }
}

#[delete("/tech/{id}")]
pub async fn delete(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let tech_id = path.into_inner();
    let current_time = Utc::now().naive_utc();

    use crate::schema::techs::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(techs.filter(id.eq(tech_id)))
        .set(deleted_at.eq(Some(current_time)))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Post successfully deleted"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to delete post"})),
    }
}

#[post("/tech/{id}/restore")]
pub async fn restore(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let tech_id = path.into_inner();
    
    use crate::schema::techs::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(techs.filter(id.eq(tech_id)))
        .set(deleted_at.eq(None::<NaiveDateTime>))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Post successfully restored"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to restore post"})),
    }
}