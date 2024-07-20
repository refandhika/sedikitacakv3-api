use actix_web::{post, get, delete, web, HttpResponse};
use chrono::{Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use diesel::result::Error;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::{DBPool, DBPooledConnection};

use crate::models::SettingDB;

// Setting Request Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct SettingRequest {
    pub param: String,
    pub value: String,
    pub note: Option<String>
}

impl SettingRequest {
    pub fn to_setting_db(&self) -> Result<SettingDB, String> {
        Ok(SettingDB {
            id: 1,
            param: self.param.clone(),
            value: self.value.clone(),
            note: Some(self.note.clone().unwrap_or("".to_string())),
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
    pub limit: Option<i32>
}

// Class Wide Function

fn create_setting(setting: SettingDB, conn: &mut DBPooledConnection) -> Result<SettingDB, Error> {
    use crate::schema::settings::dsl::*;
    diesel::insert_into(settings)
        .values((
            param.eq(setting.param),
            value.eq(setting.value),
            note.eq(setting.note),
            created_at.eq(setting.created_at),
            updated_at.eq(setting.updated_at),
            deleted_at.eq(setting.deleted_at),
        ))
        .get_result(conn)
}

fn update_setting(setting: SettingDB, setting_param: String, conn: &mut DBPooledConnection) -> Result<SettingDB, Error> {
    use crate::schema::settings::dsl::*;
    diesel::update(settings.filter(param.eq(setting_param)))
        .set((
            param.eq(setting.param),
            value.eq(setting.value),
            note.eq(setting.note),
            updated_at.eq(Utc::now().naive_utc())
        ))
        .get_result(conn)
}

fn all_setting_with_pagination(page: i32, limit: i32, conn: &mut DBPooledConnection) -> Result<Vec<SettingDB>, Error> {
    use crate::schema::settings::dsl::*;
    settings
        .filter(deleted_at.is_null())
        .order_by(id.desc())
        .limit(limit as i64)
        .offset(((page - 1) * limit) as i64)
        .load::<SettingDB>(conn)
}

// Routing

#[post("/setting")]
pub async fn create(setting_req: web::Json<SettingRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    match setting_req.to_setting_db() {
        Ok(setting_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match create_setting(setting_db, &mut conn) {
                Ok(inserted_setting) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(inserted_setting),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error inserting setting: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[post("/setting/{param}")]
pub async fn update(path: web::Path<String>, setting_req: web::Json<SettingRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    let setting_param = path.into_inner();
    match setting_req.to_setting_db() {
        Ok(setting_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match update_setting(setting_db, setting_param, &mut conn) {
                Ok(updated_setting) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(updated_setting),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Error updating setting: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}

#[get("/setting/{param}")]
pub async fn get(path: web::Path<String>, pool: web::Data<DBPool>) -> HttpResponse {
    let setting_param = path.into_inner();

    use crate::schema::settings::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match settings.filter(param.eq(setting_param)).first::<SettingDB>(&mut conn) {
        Ok(setting) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(setting.get_by_id()),
        Err(_) => HttpResponse::NotFound()
            .content_type(APPLICATION_JSON)
            .json("Setting not found"),
    }
}

#[delete("/setting/{param}")]
pub async fn delete(path: web::Path<String>, pool: web::Data<DBPool>) -> HttpResponse {
    let setting_param = path.into_inner();
    let current_time = Utc::now().naive_utc();

    use crate::schema::settings::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(settings.filter(param.eq(setting_param)))
        .set(deleted_at.eq(Some(current_time)))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Setting successfully deleted"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to delete setting"})),
    }
}

#[post("/setting/{param}/restore")]
pub async fn restore(path: web::Path<String>, pool: web::Data<DBPool>) -> HttpResponse {
    let setting_param = path.into_inner();
    
    use crate::schema::settings::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match diesel::update(settings.filter(param.eq(setting_param)))
        .set(deleted_at.eq(None::<NaiveDateTime>))
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Setting successfully restored"})),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to restore setting"})),
    }
}

#[get("/settings")]
pub async fn all(query: web::Query<PaginationParams>, pool: web::Data<DBPool>) -> HttpResponse {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    match all_setting_with_pagination(page, limit, &mut conn) {
        Ok(settings) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(settings),
        Err(_) => HttpResponse::InternalServerError()
            .content_type(APPLICATION_JSON)
            .json(serde_json::json!({"message": "Failed to retrieve technologies"})),
    }
}