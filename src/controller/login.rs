use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::constants::CONNECTION_POOL_ERROR;
use crate::DBPool;

use crate::models::UserDB;
use crate::response::Claims;

// Login Request Struct

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Class Wide Function
fn verify_password(plain: &str, hashed: &str) -> bool {
    bcrypt::verify(plain, hashed).unwrap()
}

fn generate_jwt(user_id: String) -> String {
    let my_claims = Claims { sub: user_id, exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize };
    encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).unwrap()
}

// Routing

#[post("/login")]
async fn login(login_req: web::Json<LoginRequest>, pool: web::Data<DBPool>) -> HttpResponse {
    use crate::schema::users::dsl::*;

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let user: UserDB = match users.filter(email.eq(&login_req.email)).first(&mut conn) {
        Ok(user_res) => user_res,
        Err(_) => return HttpResponse::Unauthorized().json(serde_json::json!({"message": "Invalid email"})),
    };

    if !verify_password(&login_req.password, &user.password) {
        return HttpResponse::Unauthorized().json(serde_json::json!({"message": "Invalid password"}));
    }

    let token = generate_jwt(user.id.to_string());

    HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "user_id": user.id.to_string()
    }))
}

