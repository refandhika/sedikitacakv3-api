extern crate actix_web;
extern crate diesel;

use std::{io, env};

use actix_web::{HttpServer, App, middleware};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

mod constants;
mod response;
mod models;
mod schema;

mod user;

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
            .wrap(middleware::Logger::default())
            .service(user::create)
            .service(user::get)
            .service(user::update)
            .service(user::delete)
            .service(user::restore)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await;

    Ok(())
}
