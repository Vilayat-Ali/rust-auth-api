use actix_web::{web, App, HttpServer};
use auth_api::db::{create_table, init, Table};

// routes
use auth_api::routes::{login, register};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init().await;
    create_table("user", Table::generate_schema_user()).await;
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
