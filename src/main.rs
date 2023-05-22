// routes
use auth_api::routes::docs::serve_docs;
use auth_api::routes::{login, register};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .route("/register", web::post().to(register))
                .route("/login", web::post().to(login)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
