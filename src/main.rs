use std::collections::HashMap;

use actix_web::{web, App, HttpServer};
use auth_api::db::DB;

// routes
use auth_api::routes::{login, register};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut user_schema: HashMap<&'static str, &'static str> = HashMap::new();
    user_schema.insert("id", "INTEGER PRIMARY KEY AUTOINCREMENT");
    user_schema.insert("username", "TEXT NOT NULL");
    user_schema.insert("email", "TEXT NOT NULL UNIQUE");
    user_schema.insert("password", "TEXT NOT NULL");

    let mut db = DB::connect("authdb".to_string()).await.unwrap();
    db.create_table("user", user_schema).await.unwrap();

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
