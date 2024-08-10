use actix_web::{post, get, delete, web, HttpResponse};
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, PgTextExpressionMethods, BoolExpressionMethods};

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

// Pagination Request Struct
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub search: Option<String>
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

fn all_tech_with_pagination(page: i32, limit: i32, search: String, conn: &mut DBPooledConnection) -> Result<Vec<TechDB>, Error> {
    use crate::schema::techs::dsl::*;
    let mut query = techs
        .filter(deleted_at.is_null())
        .order_by(id.desc())
        .limit(limit as i64)
        .offset(((page - 1) * limit) as i64)
        .into_boxed();
    
    if !search.is_empty() {
        query = query.filter(title.ilike(format!("%{}%", search)));
    }

    query.load::<TechDB>(conn)
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
                    .json(format!("Error inserting technology: {}", e)),
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
                    .json(format!("Error updating technology: {}", e)),
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
            .json("Technology not found"),
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
            .json(serde_json::json!({"message": "Technology successfully deleted"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to delete technology"})),
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
            .json(serde_json::json!({"message": "Technology successfully restored"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to restore technology"})),
    }
}

#[get("/techs")]
pub async fn all(query: web::Query<PaginationParams>, pool: web::Data<DBPool>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    let search = query.search.clone().unwrap_or("".to_string());

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match all_tech_with_pagination(page, limit, search, &mut conn) {
        Ok(techs) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(techs),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to retrieve technologies"})),
    }
}