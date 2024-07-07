use actix_web::{post, get, delete, web, HttpResponse};
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use uuid::Uuid;

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::{DBPool, DBPooledConnection};

use crate::models::PostDB;

// Post Request Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct PostRequest {
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub content: String,
    pub category: String,
    pub tags: Option<String>,
    pub author_id: String,
    pub published: bool
}

impl PostRequest {
    pub fn to_post_db(&self) -> Result<PostDB, String> {
        let author_id = Uuid::parse_str(&self.author_id).map_err(|e| e.to_string())?;

        Ok(PostDB {
            id: 1,
            title: self.title.clone(),
            subtitle: Some(self.subtitle.clone().unwrap_or("".to_string())),
            slug: self.slug.clone(),
            content: self.content.clone(),
            category: Some(self.category.clone()),
            tags: Some(self.tags.clone().unwrap_or("".to_string())),
            author_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: None,
            published: self.published,
        })
    }
}

// Class Wide Function

fn create_post(post: PostDB, conn: &mut DBPooledConnection) -> Result<PostDB, Error> {
    use crate::schema::posts::dsl::*;
    diesel::insert_into(posts)
        .values((
            title.eq(&post.title),
            subtitle.eq(&post.subtitle),
            slug.eq(&post.slug),
            content.eq(&post.content),
            category.eq(&post.category),
            tags.eq(&post.tags),
            author_id.eq(&post.author_id),
            created_at.eq(post.created_at),
            updated_at.eq(post.updated_at),
            deleted_at.eq(post.deleted_at),
            published.eq(post.published),
        ))
        .get_result(conn)
}

fn update_post(post: PostDB, post_id: i32, conn: &mut DBPooledConnection) -> Result<PostDB, Error> {
    use crate::schema::posts::dsl::*;
    diesel::update(posts.filter(id.eq(post_id)))
        .set((
            title.eq(post.title),
            subtitle.eq(post.subtitle),
            slug.eq(post.slug),
            content.eq(post.content),
            category.eq(post.category),
            tags.eq(post.tags),
            author_id.eq(post.author_id),
            updated_at.eq(Utc::now().naive_utc()),
            published.eq(post.published)
        ))
        .get_result(conn)
}

// Routing

#[post("/post")]
pub async fn create(user_req: web::Json<PostRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    match user_req.to_post_db() {
        Ok(post_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match create_post(post_db, &mut conn) {
                Ok(inserted_post) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(inserted_post),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error inserting post: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[post("/post/{id}")]
pub async fn update(path: web::Path<i32>, user_req: web::Json<PostRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    let post_id = path.into_inner();
    match user_req.to_post_db() {
        Ok(post_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match update_post(post_db, post_id, &mut conn) {
                Ok(updated_post) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(updated_post),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error updating post: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[get("/post/{id}")]
pub async fn get(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let post_id = path.into_inner();

    use crate::schema::posts::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match posts.filter(id.eq(post_id)).first::<PostDB>(&mut conn) {
        Ok(post) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(post.get_by_id()),
        Err(_) => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON)
            .json("Post not found"),
    }
}

#[delete("/post/{id}")]
pub async fn delete(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let post_id = path.into_inner();
    let current_time = Utc::now().naive_utc();

    use crate::schema::posts::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(posts.filter(id.eq(post_id)))
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

#[post("/post/{id}/restore")]
pub async fn restore(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let post_id = path.into_inner();
    
    use crate::schema::posts::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(posts.filter(id.eq(post_id)))
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