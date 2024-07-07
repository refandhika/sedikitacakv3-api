use diesel::{Insertable, Selectable, Queryable};
use chrono::{Utc, NaiveDateTime, NaiveDate, TimeZone};
use serde::{Serialize};
use uuid::Uuid;

use crate::constants::USER_BIRTH_NOTFOUND;

use crate::schema::users;
use crate::schema::posts;

use crate::response::UserResponseWithoutPass;
use crate::response::PostResponse;

// User Database Struct

#[derive(Queryable, Selectable, Insertable, Serialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserDB {
    pub id: Uuid,
    pub name: String,
    pub email: String,
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
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostDB {
    pub id: i32,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub content: String,
    pub category: Option<String>,
    pub tags: Option<String>,
    pub author_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub published: bool
}

impl PostDB {
    pub fn get_by_id(&self) -> PostResponse {
        PostResponse {
            id: self.id.clone(),
            title: self.title.clone(),
            subtitle: self.subtitle.clone(),
            slug: self.slug.clone(),
            content: self.content.clone(),
            category: self.category.clone(),
            tags: self.tags.clone(),
            author_id: self.author_id.to_string(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            updated_at: Utc.from_utc_datetime(&self.updated_at),
            deleted_at: self.deleted_at.map(|dt| Utc.from_utc_datetime(&dt)),
            published: self.published.clone()
        }
    }
}
