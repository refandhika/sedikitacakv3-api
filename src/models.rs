use diesel::{Insertable, Selectable, Queryable};
use chrono::{Utc, NaiveDateTime, NaiveDate, TimeZone};
use serde::{Serialize};
use uuid::Uuid;

use crate::constants::USER_BIRTH_NOTFOUND;

use crate::schema::users;
use crate::schema::posts;
use crate::schema::post_categories;
use crate::schema::projects;
use crate::schema::techs;
use crate::schema::roles;
use crate::schema::hobbies;
use crate::schema::settings;

use crate::response::*;

// User Database Struct

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDB {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub phone: Option<String>,
    pub birth: Option<NaiveDate>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl UserDB {
    pub fn get_by_id(&self) -> UserResponseWithoutPass {
        UserResponseWithoutPass {
            id: self.id.to_string(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            updated_at: Utc.from_utc_datetime(&self.updated_at),
            deleted_at: self.deleted_at.map(|dt| Utc.from_utc_datetime(&dt)),
            name: self.name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            birth: self.birth.map(|date| Utc.from_utc_date(&date).and_hms_opt(0, 0, 0)).expect(USER_BIRTH_NOTFOUND),
            linkedin: self.linkedin.clone(),
            github: self.github.clone()
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRelDB {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub birth: Option<NaiveDate>,
    pub linkedin: Option<String>,
    pub github: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostDB {
    pub id: i32,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub content: String,
    pub tags: Option<String>,
    pub author_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub published: bool,
    pub category_id: i32,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(table_name = post_categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostCatDB {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>
}

impl PostCatDB {
    pub fn get_by_id(&self) -> PostCatResponse {
        PostCatResponse {
            id: self.id.clone(),
            name: self.name.clone(),
            slug: self.slug.clone(),
            description: self.description.clone(),
            published: self.published.clone(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            updated_at: Utc.from_utc_datetime(&self.updated_at),
            deleted_at: self.deleted_at.map(|dt| Utc.from_utc_datetime(&dt))
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProjectDB {
    pub id: i32,
    pub title: String,
    pub content: String,
    //pub tech_list_id: Option<i32>,
    pub source: Option<String>,
    pub url: Option<String>,
    pub demo: Option<String>,
    pub relevant: bool,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl ProjectDB {
    pub fn get_by_id(&self) -> ProjectResponse {
        ProjectResponse {
            id: self.id.clone(),
            title: self.title.clone(),
            content: self.content.clone(),
            //tech_list_id: self.tech_list_id.clone()
            source: self.source.clone(),
            url: self.url.clone(),
            demo: self.demo.clone(),
            relevant: self.relevant.clone(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            updated_at: Utc.from_utc_datetime(&self.updated_at),
            deleted_at: self.deleted_at.map(|dt| Utc.from_utc_datetime(&dt)),
            published: self.published.clone(),
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(table_name = techs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TechDB {
    pub id: i32,
    pub title: String,
    pub icon: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl TechDB {
    pub fn get_by_id(&self) -> TechResponse {
        TechResponse {
            id: self.id.clone(),
            title: self.title.clone(),
            icon: self.icon.clone(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            updated_at: Utc.from_utc_datetime(&self.updated_at),
            deleted_at: self.deleted_at.map(|dt| Utc.from_utc_datetime(&dt)),
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(table_name = roles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RoleDB {
    pub id: i32,
    pub name: String,
    pub level: String,
    pub can_modify_user: bool,
    pub can_edit: bool,
    pub can_view: bool,
    pub is_guest: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl RoleDB {
    pub fn get_by_id(&self) -> RoleResponse {
        RoleResponse {
            id: self.id.clone(),
            name: self.name.clone(),
            level: self.level.clone(),
            can_modify_user: self.can_modify_user.clone(),
            can_edit: self.can_edit.clone(),
            can_view: self.can_view.clone(),
            is_guest: self.is_guest.clone(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            updated_at: Utc.from_utc_datetime(&self.updated_at),
            deleted_at: self.deleted_at.map(|dt| Utc.from_utc_datetime(&dt)),
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(table_name = hobbies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HobbyDB {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub image: Option<String>,
    pub item_order: i32,
    pub active: bool,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl HobbyDB {
    pub fn get_by_id(&self) -> HobbyResponse {
        HobbyResponse {
            id: self.id.clone(),
            title: self.title.clone(),
            content: self.content.clone(),
            image: self.image.clone(),
            item_order: self.item_order.clone(),
            active: self.active.clone(),
            published: self.published.clone(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            updated_at: Utc.from_utc_datetime(&self.updated_at),
            deleted_at: self.deleted_at.map(|dt| Utc.from_utc_datetime(&dt)),
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(table_name = settings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SettingDB {
    pub id: i32,
    pub param: String,
    pub value: String,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl SettingDB {
    pub fn get_by_id(&self) -> SettingResponse {
        SettingResponse {
            id: self.id.clone(),
            param: self.param.clone(),
            value: self.value.clone(),
            note: self.note.clone(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            updated_at: Utc.from_utc_datetime(&self.updated_at),
            deleted_at: self.deleted_at.map(|dt| Utc.from_utc_datetime(&dt)),
        }
    }
}
