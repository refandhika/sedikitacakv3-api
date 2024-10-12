extern crate actix_web;
extern crate diesel;

use std::{io, env, fs};
use std::path::Path;

use actix_web::{HttpServer, App, middleware as WebMiddleware, web};
use actix_governor::{Governor, GovernorConfigBuilder};
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
use crate::controller::hobby;
use crate::controller::setting;
use crate::controller::image;
use crate::controller::contact;

mod constants;
mod response;
mod models;
mod schema;
mod middleware;
mod controller;
mod errors;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let _ = dotenvy::dotenv();

    let upload_dir = Path::new("./uploads");
    if !upload_dir.exists() {
        fs::create_dir_all(upload_dir)?;
    }

    let governor_conf = GovernorConfigBuilder::default()
        .requests_per_second(10)
        .burst_size(50)
        .finish()
        .unwrap();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let _ = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Governor::new(&governor_conf))
            .wrap(WebMiddleware::Logger::default())
            .service(image::serve)
            .service(
                web::scope("/pub")
                .service(login::login)
                .service(user::get)
                //.service(user::create)
                .service(post::active)
                .service(post::get_by_slug)
                .service(project::active)
                .service(hobby::active)
                .service(setting::get)
                .service(image::get)
                .service(contact::send)
            )
            .service(
                web::scope("/pro")
                .wrap(AuthMiddleware)
                .service(user::all)
                .service(user::create)
                .service(user::update)
                .service(user::delete)
                .service(user::restore)
                .service(post::all)
                .service(post::get)
                .service(post::create)
                .service(post::update)
                .service(post::delete)
                .service(post::restore)
                .service(postcat::all)
                .service(postcat::active)
                .service(postcat::get)
                .service(postcat::create)
                .service(postcat::update)
                .service(postcat::delete)
                .service(postcat::restore)
                .service(project::all)
                .service(project::get)
                .service(project::create)
                .service(project::update)
                .service(project::delete)
                .service(project::restore)
                .service(tech::all)
                .service(tech::get)
                .service(tech::create)
                .service(tech::update)
                .service(tech::delete)
                .service(tech::restore)
                .service(role::all)
                .service(role::get)
                .service(role::create)
                .service(role::update)
                .service(role::delete)
                .service(role::restore)
                .service(hobby::all)
                .service(hobby::get)
                .service(hobby::create)
                .service(hobby::update)
                .service(hobby::delete)
                .service(hobby::restore)
                .service(setting::all)
                .service(setting::create)
                .service(setting::update)
                .service(setting::delete)
                .service(setting::restore)
                .service(image::all)
                .service(image::upload)
                .service(image::delete)
            )
            .app_data(web::Data::new(pool.clone()))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await;

    Ok(())
}
