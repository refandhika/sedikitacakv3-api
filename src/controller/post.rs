use actix_web::{post, get, delete, web, HttpResponse};
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, Queryable};
use uuid::Uuid;

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::{DBPool, DBPooledConnection};

use crate::models::PostDB;
use crate::models::PostCatDB;
use crate::models::UserDB;

// Post Request Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct PostRequest {
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub content: String,
    pub category_id: i32,
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
            category_id: self.category_id.clone(),
            tags: Some(self.tags.clone().unwrap_or("".to_string())),
            author_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: None,
            published: self.published,
        })
    }
}

#[derive(Queryable, Debug, Serialize)]
pub struct JoinedPost {
    #[serde(flatten)]
    pub post: PostDB,
    pub category: PostCatDB,
    pub user: UserDB
}

// Pagination Request Struct
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub cat: Option<String>
}

// Class Wide Function

fn create_post(post: PostDB, conn: &mut DBPooledConnection) -> Result<PostDB, Error> {
    use crate::schema::posts::dsl::*;
    diesel::insert_into(posts)
        .values((
            title.eq(post.title),
            subtitle.eq(post.subtitle),
            slug.eq(post.slug),
            content.eq(post.content),
            category_id.eq(post.category_id),
            tags.eq(post.tags),
            author_id.eq(post.author_id),
            created_at.eq(post.created_at),
            updated_at.eq(post.updated_at),
            deleted_at.eq(post.deleted_at),
            published.eq(post.published),
        ))
        .get_result(conn)
}

fn update_post(post: PostDB, post_slug: String, conn: &mut DBPooledConnection) -> Result<PostDB, Error> {
    use crate::schema::posts::dsl::*;
    diesel::update(posts.filter(slug.eq(post_slug)))
        .set((
            title.eq(post.title),
            subtitle.eq(post.subtitle),
            slug.eq(post.slug),
            content.eq(post.content),
            category_id.eq(post.category_id),
            tags.eq(post.tags),
            author_id.eq(post.author_id),
            updated_at.eq(Utc::now().naive_utc()),
            published.eq(post.published)
        ))
        .get_result(conn)
}

fn all_post_with_pagination(page: i32, limit: i32, cat: String, conn: &mut DBPooledConnection) -> Result<Vec<JoinedPost>, Error> {
    use crate::schema::posts::dsl::*;
    use crate::schema::post_categories::dsl::{post_categories, deleted_at as category_deleted_at, slug as category_slug};
    use crate::schema::users::dsl::{users, deleted_at as user_deleted_at};
    
    let mut query = posts
        .inner_join(post_categories)
        .inner_join(users)
        .filter(deleted_at.is_null())
        .filter(category_deleted_at.is_null())
        .filter(user_deleted_at.is_null())
        .order_by(id.desc())
        .limit(limit as i64)
        .offset(((page - 1) * limit) as i64)
        .into_boxed();

    if !cat.is_empty() {
        query = query.filter(category_slug.eq(cat));
    }

    let result: Vec<JoinedPost> = query
        .load::<(PostDB, PostCatDB, UserDB)>(conn)?
        .into_iter()
        .map(|(post, category, user)| JoinedPost { post, category, user })
        .collect();

    Ok(result)
}

fn get_single_post(post_slug: String, conn: &mut DBPooledConnection) -> Result<JoinedPost, Error> {
    use crate::schema::posts::dsl::*;
    use crate::schema::post_categories::dsl::{post_categories};
    use crate::schema::users::dsl::{users};

    posts
        .inner_join(post_categories)
        .inner_join(users)
        .filter(slug.eq(post_slug))
        .limit(1)
        .get_result(conn)
}

// Routing

#[post("/post")]
pub async fn create(post_req: web::Json<PostRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    match post_req.to_post_db() {
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

#[post("/post/{slug}")]
pub async fn update(path: web::Path<String>, post_req: web::Json<PostRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    let post_slug = path.into_inner();
    match post_req.to_post_db() {
        Ok(post_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match update_post(post_db, post_slug, &mut conn) {
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

#[get("/post/{slug}")]
pub async fn get(path: web::Path<String>, pool: web::Data<DBPool>) -> HttpResponse {
    let post_slug = path.into_inner();

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match get_single_post(post_slug, &mut conn) {
        Ok(post) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(post),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Post not found"})),
    }
}

#[delete("/post/{slug}")]
pub async fn delete(path: web::Path<String>, pool: web::Data<DBPool>) -> HttpResponse {
    let post_slug = path.into_inner();
    let current_time = Utc::now().naive_utc();

    use crate::schema::posts::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(posts.filter(slug.eq(post_slug)))
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

#[post("/post/{slug}/restore")]
pub async fn restore(path: web::Path<String>, pool: web::Data<DBPool>) -> HttpResponse {
    let post_slug = path.into_inner();
    
    use crate::schema::posts::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(posts.filter(slug.eq(post_slug)))
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

#[get("/posts")]
pub async fn all(query: web::Query<PaginationParams>, pool: web::Data<DBPool>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    let cat = query.cat.clone().unwrap_or("".to_string());

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match all_post_with_pagination(page, limit, cat, &mut conn) {
        Ok(posts) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(posts),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to retrieve posts"})),
    }
}