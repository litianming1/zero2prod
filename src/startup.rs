use std::net::TcpListener;
use actix_web::{dev::Server, web, App, HttpServer,middleware::Logger};
use crate::routes::{health_check, subscribe};
use sqlx::PgPool;

pub fn run(
    listener: TcpListener,
    db_pool:PgPool
) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move|| {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            // A new entry in our routing table for POST /subscriptions requests
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}