use actix_web::{post, get, delete, web, HttpResponse};
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, JoinOnDsl, Queryable, Table};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::{DBPool, DBPooledConnection};

use crate::models::ProjectDB;
use crate::models::TechDB;

// Project Request Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectRequest {
    pub title: String,
    pub content: String,
    pub source: Option<String>,
    pub url: Option<String>,
    pub demo: Option<String>,
    pub relevant: bool,
    pub published: bool,
    pub tech_ids: Vec<i32>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct ProjectTechJoin {
    #[serde(flatten)]
    pub project: ProjectDB,
    pub techs: Vec<TechDB>
}

// Pagination Request Struct
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub rlv: Option<bool>
}

// Class Wide Function

fn create_project(project: ProjectRequest, conn: &mut DBPooledConnection) -> Result<ProjectTechJoin, Error> {
    use crate::schema::projects::dsl::*;
    use crate::schema::projects_techs::dsl::{projects_techs, project_id as pt_project_id, tech_id as pt_tech_id};
    use crate::schema::techs::dsl::{techs, id as techs_id};

    let project_db: ProjectDB = ProjectDB {
        id: 1,
        title: project.title.clone(),
        content: project.content.clone(),
        source: Some(project.source.clone().unwrap_or("".to_string())),
        url: Some(project.url.clone().unwrap_or("".to_string())),
        demo: Some(project.demo.clone().unwrap_or("".to_string())),
        relevant: project.relevant,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        deleted_at: None,
        published: project.published,
    };
    
    let inserted_project: ProjectDB = diesel::insert_into(projects)
        .values((
            title.eq(project_db.title),
            content.eq(project_db.content),
            source.eq(project_db.source),
            url.eq(project_db.url),
            demo.eq(project_db.demo),
            relevant.eq(project_db.relevant),
            created_at.eq(project_db.created_at),
            updated_at.eq(project_db.updated_at),
            deleted_at.eq(project_db.deleted_at),
            published.eq(project_db.published),
        ))
        .get_result(conn)?;

    for tech_id_item in project.tech_ids {
        let _ = diesel::insert_into(projects_techs)
            .values((
                pt_project_id.eq(inserted_project.id),
                pt_tech_id.eq(tech_id_item),
            ))
            .execute(conn);
    }

    
    let project = projects
        .find(inserted_project.id)
        .first::<ProjectDB>(conn)?;

    let tech_list = projects_techs
        .filter(pt_project_id.eq(inserted_project.id))
        .inner_join(
            techs.on(pt_tech_id.eq(techs_id))
        )
        .select(techs::all_columns())
        .load::<TechDB>(conn)?;

    Ok(ProjectTechJoin {
        project,
        techs: tech_list
    })
}

fn update_project(project: ProjectRequest, project_id: i32, conn: &mut DBPooledConnection) -> Result<ProjectTechJoin, Error> {
    
    use crate::schema::projects::dsl::*;
    use crate::schema::projects_techs::dsl::{projects_techs, project_id as pt_project_id, tech_id as pt_tech_id};
    use crate::schema::techs::dsl::{techs, id as techs_id};

    let project_db: ProjectDB = ProjectDB {
        id: 1,
        title: project.title.clone(),
        content: project.content.clone(),
        source: Some(project.source.clone().unwrap_or("".to_string())),
        url: Some(project.url.clone().unwrap_or("".to_string())),
        demo: Some(project.demo.clone().unwrap_or("".to_string())),
        relevant: project.relevant,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        deleted_at: None,
        published: project.published,
    };
    
    let _: ProjectDB = diesel::update(projects.filter(id.eq(project_id)))
        .set((
            title.eq(project_db.title),
            content.eq(project_db.content),
            source.eq(project_db.source),
            url.eq(project_db.url),
            demo.eq(project_db.demo),
            relevant.eq(project_db.relevant),
            updated_at.eq(Utc::now().naive_utc()),
            published.eq(project_db.published)
        ))
        .get_result(conn)?;

    diesel::delete(projects_techs.filter(pt_project_id.eq(project_id)))
        .execute(conn)?;

    for tech_id_item in project.tech_ids {
        let _ = diesel::insert_into(projects_techs)
            .values((
                pt_project_id.eq(project_id),
                pt_tech_id.eq(tech_id_item),
            ))
            .execute(conn);
    }
    
    let project = projects
        .find(project_id)
        .first::<ProjectDB>(conn)?;

    let tech_list = projects_techs
        .filter(pt_project_id.eq(project_id))
        .inner_join(
            techs.on(pt_tech_id.eq(techs_id))
        )
        .select(techs::all_columns())
        .load::<TechDB>(conn)?;

    Ok(ProjectTechJoin {
        project,
        techs: tech_list
    })
}

fn all_project_with_pagination(page: i32, limit: i32, rlv: bool, conn: &mut DBPooledConnection) -> Result<Vec<ProjectTechJoin>, Error> {
    use crate::schema::projects::dsl::*;
    use crate::schema::projects_techs::dsl::{projects_techs, project_id as pt_project_id, tech_id as pt_tech_id};
    use crate::schema::techs::dsl::{techs, id as techs_id};

    let projects_list = projects
        .filter(deleted_at.is_null())
        .filter(relevant.eq(rlv))
        .order_by(id.desc())
        .limit(limit as i64)
        .offset(((page - 1) * limit) as i64)
        .load::<ProjectDB>(conn)?;

    let mut result = Vec::new();

    for project in projects_list {
        let tech_list = projects_techs
            .filter(pt_project_id.eq(project.id))
            .inner_join(
                techs.on(pt_tech_id.eq(techs_id))
            )
            .select(techs::all_columns())
            .load::<TechDB>(conn)?;

        result.push(ProjectTechJoin {
            project,
            techs: tech_list,
        })
    }

    Ok(result)
}

fn get_single_project(project_id: i32, conn: &mut DBPooledConnection) -> Result<ProjectTechJoin, Error> {
    use crate::schema::projects::dsl::*;
    use crate::schema::projects_techs::dsl::{projects_techs, project_id as pt_project_id, tech_id as pt_tech_id};
    use crate::schema::techs::dsl::{techs, id as techs_id};

    let project = projects
        .find(project_id)
        .first::<ProjectDB>(conn)?;

    let tech_list = projects_techs
        .filter(pt_project_id.eq(project_id))
        .inner_join(
            techs.on(pt_tech_id.eq(techs_id))
        )
        .select(techs::all_columns())
        .load::<TechDB>(conn)?;

    Ok(ProjectTechJoin {
        project,
        techs: tech_list
    })
}

// Routing

#[post("/project")]
pub async fn create(project_req: web::Json<ProjectRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let project_req_inner = project_req.into_inner();
    match create_project(project_req_inner, &mut conn) {
        Ok(inserted_project) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(inserted_project),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Error inserting project: {}"})),
    }
}

#[post("/project/{id}")]
pub async fn update(path: web::Path<i32>, project_req: web::Json<ProjectRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    let project_id = path.into_inner();

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let project_req_inner = project_req.into_inner();
    match update_project(project_req_inner, project_id, &mut conn) {
        Ok(inserted_project) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(inserted_project),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Error updating project: {}"})),
    }
}

#[get("/project/{id}")]
pub async fn get(path: web::Path<i32>, pool: web::Data<DBPool>) -> HttpResponse {
    let project_id = path.into_inner();

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match get_single_project(project_id, &mut conn) {
        Ok(project) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(project),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Project not found"})),
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