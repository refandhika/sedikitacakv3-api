use actix_web::{post, get, delete, web, HttpResponse};
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, PgTextExpressionMethods, BoolExpressionMethods};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::{DBPool, DBPooledConnection};

use crate::models::HobbyDB;

// Hobby Request Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct HobbyRequest {
    pub title: String,
    pub content: String,
    pub image: Option<String>,
    pub item_order: i32,
    pub active: bool,
    pub published: bool,
    pub order: i32
}

impl HobbyRequest {
    pub fn to_hobby_db(&self) -> Result<HobbyDB, String> {
        Ok(HobbyDB {
            id: 1,
            title: self.title.clone(),
            content: self.content.clone(),
            image: Some(self.image.clone().unwrap_or("".to_string())),
            item_order: self.item_order.clone(),
            active: self.active,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: None,
            published: self.published,
            order: self.order.clone(),
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

fn create_hobby(hobby: HobbyDB, conn: &mut DBPooledConnection) -> Result<HobbyDB, Error> {
    use crate::schema::hobbies::dsl::*;
    let next_order =  get_next_order(conn)?;
    diesel::insert_into(hobbies)
        .values((
            title.eq(hobby.title),
            content.eq(hobby.content),
            image.eq(hobby.image),
            item_order.eq(hobby.item_order),
            active.eq(hobby.active),
            created_at.eq(hobby.created_at),
            updated_at.eq(hobby.updated_at),
            deleted_at.eq(hobby.deleted_at),
            published.eq(hobby.published),
            order.eq(next_order)
        ))
        .get_result(conn)
}

fn update_hobby(hobby: HobbyDB, hobby_id: i32, conn: &mut DBPooledConnection) -> Result<HobbyDB, Error> {
    use crate::schema::hobbies::dsl::*;
    diesel::update(hobbies.filter(id.eq(hobby_id)))
        .set((
            title.eq(hobby.title),
            content.eq(hobby.content),
            image.eq(hobby.image),
            item_order.eq(hobby.item_order),
            active.eq(hobby.active),
            updated_at.eq(Utc::now().naive_utc()),
            published.eq(hobby.published),
            order.eq(hobby.order)
        ))
        .get_result(conn)
}

fn all_hobby_with_pagination(page: i32, limit: i32, search: String, is_published:bool, conn: &mut DBPooledConnection) -> Result<Vec<HobbyDB>, Error> {
    use crate::schema::hobbies::dsl::*;
    let mut query = hobbies
        .filter(deleted_at.is_null())
        .order_by(order.desc())
        .limit(limit as i64)
        .offset(((page - 1) * limit) as i64)
        .into_boxed();

    if !search.is_empty() {
        query = query.filter(
            title.ilike(format!("%{}%", search))
                .or(content.ilike(format!("%{}%", search)))
        );
    }

    if is_published {
        query = query.filter(published.eq(is_published));
    }

    query.load::<HobbyDB>(conn)
}

fn get_next_order(conn: &mut DBPooledConnection) -> Result<i32, Error> {
    use crate::schema::hobbies::dsl::*;

    let next_order: i32 = hobbies
        .select(diesel::dsl::max(order))
        .first::<Option<i32>>(conn)?
        .map_or(1, |max_order| max_order + 1);

    Ok(next_order)
}

// Routing

#[post("/hobby")]
pub async fn create(hobby_req: web::Json<HobbyRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    match hobby_req.to_hobby_db() {
        Ok(hobby_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match create_hobby(hobby_db, &mut conn) {
                Ok(inserted_hobby) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(inserted_hobby),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error inserting hobby: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[post("/hobby/{id}")]
pub async fn update(path: web::Path<i32>, hobby_req: web::Json<HobbyRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    let hobby_id = path.into_inner();
    match hobby_req.to_hobby_db() {
        Ok(hobby_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match update_hobby(hobby_db, hobby_id, &mut conn) {
                Ok(updated_hobby) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(updated_hobby),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error updating hobby: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[get("/hobby/{id}")]
pub async fn get(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let hobby_id = path.into_inner();

    use crate::schema::hobbies::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match hobbies.filter(id.eq(hobby_id)).first::<HobbyDB>(&mut conn) {
        Ok(hobby) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(hobby.get_by_id()),
        Err(_) => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON)
            .json("Hobby not found"),
    }
}

#[delete("/hobby/{id}")]
pub async fn delete(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let hobby_id = path.into_inner();
    let current_time = Utc::now().naive_utc();

    use crate::schema::hobbies::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(hobbies.filter(id.eq(hobby_id)))
        .set(deleted_at.eq(Some(current_time)))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Hobby successfully deleted"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to delete hobby"})),
    }
}

#[post("/hobby/{id}/restore")]
pub async fn restore(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let hobby_id = path.into_inner();
    
    use crate::schema::hobbies::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(hobbies.filter(id.eq(hobby_id)))
        .set(deleted_at.eq(None::<NaiveDateTime>))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Hobby successfully restored"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to restore hobby"})),
    }
}

#[get("/hobbies")]
pub async fn all(query: web::Query<PaginationParams>, pool: web::Data<DBPool>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    let search = query.search.clone().unwrap_or("".to_string());

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match all_hobby_with_pagination(page, limit, search, false, &mut conn) {
        Ok(hobbies) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(hobbies),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to retrieve hobbies"})),
    }
}

#[get("/hobbies/active")]
pub async fn active(query: web::Query<PaginationParams>, pool: web::Data<DBPool>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    let search = query.search.clone().unwrap_or("".to_string());

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match all_hobby_with_pagination(page, limit, search, true, &mut conn) {
        Ok(hobbies) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(hobbies),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to retrieve hobbies"})),
    }
}