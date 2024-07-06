use diesel::{Insertable, Selectable, Queryable};
use chrono::{Utc, NaiveDateTime, NaiveDate, TimeZone};
use serde::{Serialize};
use uuid::Uuid;

use crate::constants::{USER_PHONE_NOTFOUND, USER_BIRTH_NOTFOUND, USER_LINKEDIN_NOTFOUND, USER_GITHUB_NOTFOUND, USER_DELETED_AT_NOTFOUND};

use crate::schema::users;

use crate::response::UserResponse;

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
    pub fn to_create(&self) -> UserResponse {
        UserResponse {
            id: self.id.to_string(),
            created_at: Utc.from_utc_datetime(&self.created_at),
            updated_at: Utc.from_utc_datetime(&self.updated_at),
            deleted_at: self.deleted_at.map(|dt| Utc.from_utc_datetime(&dt)).expect(USER_DELETED_AT_NOTFOUND),
            name: self.name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone().expect(USER_PHONE_NOTFOUND),
            birth: self.birth.map(|date| Utc.from_utc_date(&date).and_hms_opt(0, 0, 0)).expect(USER_BIRTH_NOTFOUND).expect(USER_BIRTH_NOTFOUND),
            linkedin: self.linkedin.clone().expect(USER_LINKEDIN_NOTFOUND),
            github: self.github.clone().expect(USER_GITHUB_NOTFOUND),
            password: self.password.clone()
        }
    }
}
