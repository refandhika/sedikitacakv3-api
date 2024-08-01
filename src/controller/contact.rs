use actix_web::{post, web, HttpResponse, HttpRequest};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use diesel::{RunQueryDsl, ExpressionMethods};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

use crate::errors::ContactError;

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR};
use crate::{DBPool, DBPooledConnection};

use crate::models::ContactDB;

// Contact Request Struct
#[derive(Debug, Deserialize, Serialize)]
pub struct ContactRequest {
    pub subject: String,
    pub name: String,
    pub email: String,
    pub content: String
}

impl ContactRequest {
    pub fn to_hobby_db(&self, http_req: HttpRequest) -> Result<ContactDB, String> {
        let ip_address = http_req.connection_info().realip_remote_addr().unwrap_or("").to_string();

        Ok(ContactDB {
            id: 1,
            subject: self.subject.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            content: self.content.clone(),
            created_at: Utc::now().naive_utc(),
            ip_address: Some(ip_address),
        })
    }
}
// Class Wide Function

fn create_contact(contact: ContactDB, conn: &mut DBPooledConnection) -> Result<ContactDB, ContactError> {
    let emsg = Message::builder()
        .from("Sedikit Acak CF <cf@sedikitacak.com>".parse().unwrap())
        .reply_to(format!("{} <{}>", contact.name, contact.email).parse().unwrap())
        .to("Refa Andhika <refandika@gmail.com>".parse().unwrap())
        .subject(contact.subject.clone())
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(contact.content.clone()))
        .unwrap();

    let user = env::var("SMTP.USER").expect("SMTP.USER");
    let pass = env::var("SMTP.PASS").expect("SMTP.PASS");
    let creds = Credentials::new(user.to_owned(), pass.to_owned());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&emsg)?;
    
    use crate::schema::contacts::dsl::*;
    let new_contact = diesel::insert_into(contacts)
        .values((
            subject.eq(contact.subject),
            name.eq(contact.name),
            email.eq(contact.email),
            content.eq(contact.content),
            ip_address.eq(contact.ip_address),
            created_at.eq(contact.created_at)
        ))
        .get_result(conn);

    Ok(new_contact?)
}

// Routing

#[post("/contact")]
pub async fn send(contact_req: web::Json<ContactRequest>, http_req: HttpRequest, pool: web::Data<DBPool>) -> HttpResponse {
    match contact_req.to_hobby_db(http_req) {
        Ok(contact_db) => {
            let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
            match create_contact(contact_db, &mut conn) {
                Ok(inserted_contact) => HttpResponse::Created()
                    .content_type(APPLICATION_JSON)
                    .json(inserted_contact),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type(APPLICATION_JSON)
                    .json(format!("Failed storing contact: {}", e)),
            }
        }
        Err(e) => HttpResponse::BadRequest()
            .content_type(APPLICATION_JSON)
            .json(e),
    }
}
