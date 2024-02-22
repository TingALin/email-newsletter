use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {} !", &name)
// }

// #[derive(serde::Deserialize)]
// struct FormData {
//     email: String,
//     name: String,
// }

// async fn health_check() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // let connection = web::Data::new(connection);
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
        // .route("/", web::get().to(greet))
        // .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    // .bind(address)?
    .run();
    Ok(server)
}