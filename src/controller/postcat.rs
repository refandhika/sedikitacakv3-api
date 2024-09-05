use actix_web::{post, get, delete, web, HttpResponse};
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, PgTextExpressionMethods, BoolExpressionMethods};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::{DBPool, DBPooledConnection};

use crate::models::PostCatDB;

// Post Category Request Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct PostCatRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub published: bool
}

impl PostCatRequest {
    pub fn to_pcat_db(&self) -> Result<PostCatDB, String> {
        Ok(PostCatDB {
            id: 1,
            name: self.name.clone(),
            slug: self.slug.clone(),
            description: Some(self.description.clone().unwrap_or("".to_string())),
            published: self.published,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: None,
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

fn create_pcat(post_category: PostCatDB, conn: &mut DBPooledConnection) -> Result<PostCatDB, Error> {
    use crate::schema::post_categories::dsl::*;
    diesel::insert_into(post_categories)
        .values((
            name.eq(&post_category.name),
            slug.eq(&post_category.slug),
            description.eq(&post_category.description),
            published.eq(post_category.published),
            created_at.eq(post_category.created_at),
            updated_at.eq(post_category.updated_at),
            deleted_at.eq(post_category.deleted_at),
        ))
        .get_result(conn)
}

fn update_pcat(post_category: PostCatDB, pcat_id: i32, conn: &mut DBPooledConnection) -> Result<PostCatDB, Error> {
    use crate::schema::post_categories::dsl::*;
    diesel::update(post_categories.filter(id.eq(pcat_id)))
        .set((
            name.eq(post_category.name),
            slug.eq(post_category.slug),
            description.eq(post_category.description),
            published.eq(post_category.published),
            updated_at.eq(Utc::now().naive_utc()),
        ))
        .get_result(conn)
}

fn all_postcat_with_pagination(page: i32, limit: i32, search: String, conn: &mut DBPooledConnection) -> Result<Vec<PostCatDB>, Error> {
    use crate::schema::post_categories::dsl::*;
    let mut query = post_categories
        .filter(deleted_at.is_null())
        .order_by(id.desc())
        .into_boxed();

    if limit != 0 {
        query = query.limit(limit as i64)
        .offset(((page - 1) * limit) as i64)
    }

    if !search.is_empty() {
        query = query.filter(
            name.ilike(format!("%{}%", search))
                .or(description.ilike(format!("%{}%", search)))
        );
    }

    query.load::<PostCatDB>(conn)
}

// Routing

#[post("/post-category")]
pub async fn create(postcat_req: web::Json<PostCatRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    match postcat_req.to_pcat_db() {
        Ok(postcat_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match create_pcat(postcat_db, &mut conn) {
                Ok(inserted_postcat) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(inserted_postcat),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error inserting post category: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[post("/post-category/{id}")]
pub async fn update(path: web::Path<i32>, postcat_req: web::Json<PostCatRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    let pcat_id = path.into_inner();
    match postcat_req.to_pcat_db() {
        Ok(postcat_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match update_pcat(postcat_db, pcat_id, &mut conn) {
                Ok(updated_postcat) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(updated_postcat),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error updating post category: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[get("/post-category/{id}")]
pub async fn get(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let pcat_id = path.into_inner();

    use crate::schema::post_categories::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match post_categories.filter(id.eq(pcat_id)).first::<PostCatDB>(&mut conn) {
        Ok(pcat) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(pcat.get_by_id()),
        Err(_) => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON)
            .json("Post category not found"),
    }
}

#[delete("/post-category/{id}")]
pub async fn delete(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let pcat_id = path.into_inner();
    let current_time = Utc::now().naive_utc();

    use crate::schema::post_categories::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(post_categories.filter(id.eq(pcat_id)))
        .set(deleted_at.eq(Some(current_time)))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Post category successfully deleted"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to delete post category"})),
    }
}

#[post("/post-category/{id}/restore")]
pub async fn restore(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let pcat_id = path.into_inner();
    
    use crate::schema::post_categories::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(post_categories.filter(id.eq(pcat_id)))
        .set(deleted_at.eq(None::<NaiveDateTime>))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Post category successfully restored"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to restore post category"})),
    }
}

#[get("/post-categories")]
pub async fn all(query: web::Query<PaginationParams>, pool: web::Data<DBPool>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(0);
    let search = query.search.clone().unwrap_or("".to_string());

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match all_postcat_with_pagination(page, limit, search, &mut conn) {
        Ok(postcats) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(postcats),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to retrieve post categories"})),
    }
}