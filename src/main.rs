use actix_files as fs;
use actix_web::{web, App, HttpServer};

// routes
use auth_api::routes::docs::serve_docs;
use auth_api::routes::{login, register};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .route("/register", web::get().to(register::hello))
                    .route("/login", web::get().to(login::hello)),
            )
            .service(fs::Files::new("/pages", ".").show_files_listing())
            .route("/docs", web::get().to(serve_docs))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
