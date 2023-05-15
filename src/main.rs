use actix_web::{web, App, HttpServer};

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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
