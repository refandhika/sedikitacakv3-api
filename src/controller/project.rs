use actix_web::{post, get, delete, web, HttpResponse};
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::{DBPool, DBPooledConnection};

use crate::models::ProjectDB;

// Project Request Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectRequest {
    pub title: String,
    pub content: String,
    //pub tech_list_id: Option<i32>,
    pub source: Option<String>,
    pub url: Option<String>,
    pub demo: Option<String>,
    pub relevant: bool,
    pub published: bool,
}

impl ProjectRequest {
    pub fn to_project_db(&self) -> Result<ProjectDB, String> {
        Ok(ProjectDB {
            id: 1,
            title: self.title.clone(),
            content: self.content.clone(),
            //tech_list_id: Some(self.texh_list_id.clone()),
            source: Some(self.source.clone().unwrap_or("".to_string())),
            url: Some(self.url.clone().unwrap_or("".to_string())),
            demo: Some(self.demo.clone().unwrap_or("".to_string())),
            relevant: self.relevant,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            deleted_at: None,
            published: self.published,
        })
    }
}

// Pagination Request Struct
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub rlv: Option<bool>
}

// Class Wide Function

fn create_project(project: ProjectDB, conn: &mut DBPooledConnection) -> Result<ProjectDB, Error> {
    use crate::schema::projects::dsl::*;
    diesel::insert_into(projects)
        .values((
            title.eq(project.title),
            content.eq(project.content),
            //tech_list_id.eq(project.tech_list_id),
            source.eq(project.source),
            url.eq(project.url),
            demo.eq(project.demo),
            relevant.eq(project.relevant),
            created_at.eq(project.created_at),
            updated_at.eq(project.updated_at),
            deleted_at.eq(project.deleted_at),
            published.eq(project.published),
        ))
        .get_result(conn)
}

fn update_project(project: ProjectDB, project_id: i32, conn: &mut DBPooledConnection) -> Result<ProjectDB, Error> {
    use crate::schema::projects::dsl::*;
    diesel::update(projects.filter(id.eq(project_id)))
        .set((
            title.eq(project.title),
            content.eq(project.content),
            //tech_list_id.eq(project.tech_list_id),
            source.eq(project.source),
            url.eq(project.url),
            demo.eq(project.demo),
            relevant.eq(project.relevant),
            updated_at.eq(Utc::now().naive_utc()),
            published.eq(project.published)
        ))
        .get_result(conn)
}

fn all_project_with_pagination(page: i32, limit: i32, rlv: bool, conn: &mut DBPooledConnection) -> Result<Vec<ProjectDB>, Error> {
    use crate::schema::projects::dsl::*;
    projects
        .filter(deleted_at.is_null())
        .filter(relevant.eq(rlv))
        .order_by(id.desc())
        .limit(limit as i64)
        .offset(((page - 1) * limit) as i64)
        .load::<ProjectDB>(conn)
}

// Routing

#[post("/project")]
pub async fn create(project_req: web::Json<ProjectRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    match project_req.to_project_db() {
        Ok(project_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match create_project(project_db, &mut conn) {
                Ok(inserted_project) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(inserted_project),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error inserting project: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[post("/project/{id}")]
pub async fn update(path: web::Path<i32>, project_req: web::Json<ProjectRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    let project_id = path.into_inner();
    match project_req.to_project_db() {
        Ok(project_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match update_project(project_db, project_id, &mut conn) {
                Ok(updated_project) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(updated_project),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error updating project: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[get("/project/{id}")]
pub async fn get(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let project_id = path.into_inner();

    use crate::schema::projects::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match projects.filter(id.eq(project_id)).first::<ProjectDB>(&mut conn) {
        Ok(project) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(project.get_by_id()),
        Err(_) => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON)
            .json("Project not found"),
    }
}

#[delete("/project/{id}")]
pub async fn delete(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let project_id = path.into_inner();
    let current_time = Utc::now().naive_utc();

    use crate::schema::projects::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(projects.filter(id.eq(project_id)))
        .set(deleted_at.eq(Some(current_time)))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Project successfully deleted"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to delete project"})),
    }
}

#[post("/project/{id}/restore")]
pub async fn restore(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let project_id = path.into_inner();
    
    use crate::schema::projects::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(projects.filter(id.eq(project_id)))
        .set(deleted_at.eq(None::<NaiveDateTime>))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Project successfully restored"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to restore project"})),
    }
}

#[get("/projects")]
pub async fn all(query: web::Query<PaginationParams>, pool: web::Data<DBPool>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    let rlv = query.rlv.unwrap_or(true);

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match all_project_with_pagination(page, limit, rlv, &mut conn) {
        Ok(projects) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(projects),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to retrieve projects"})),
    }
}