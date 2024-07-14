extern crate actix_web;
extern crate diesel;

use std::{io, env};

use actix_web::{HttpServer, App, middleware as WebMiddleware, web};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

use crate::middleware::auth::AuthMiddleware;
use crate::controller::login;
use crate::controller::user;
use crate::controller::post;
use crate::controller::postcat;
use crate::controller::project;
use crate::controller::tech;
use crate::controller::role;

mod constants;
mod response;
mod models;
mod schema;
mod middleware;
mod controller;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let _ = dotenvy::dotenv();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let _ = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(WebMiddleware::Logger::default())
            .service(
                web::scope("/pub")
                .service(login::login)
                .service(user::get)
                .service(post::get)
            )
            .service(
                web::scope("/pro")
                .wrap(AuthMiddleware)
                //.service(user::create)
                .service(user::update)
                .service(user::delete)
                .service(user::restore)
                .service(post::create)
                .service(post::update)
                .service(post::delete)
                .service(post::restore)
                .service(postcat::get)
                .service(postcat::create)
                .service(postcat::update)
                .service(postcat::delete)
                .service(postcat::restore)
                .service(project::get)
                .service(project::create)
                .service(project::update)
                .service(project::delete)
                .service(project::restore)
                .service(tech::get)
                .service(tech::create)
                .service(tech::update)
                .service(tech::delete)
                .service(tech::restore)
                .service(role::get)
                .service(role::create)
                .service(role::update)
                .service(role::delete)
                .service(role::restore)
            )
            .app_data(web::Data::new(pool.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await;

    Ok(())
}
