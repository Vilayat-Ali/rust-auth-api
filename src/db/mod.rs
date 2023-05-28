use sqlx::{migrate::MigrateDatabase, sqlite::SqliteRow, Sqlite};

const DB_URL: &str = "sqlite://auth.db";

pub async fn init() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

pub async fn create_user_table<'a>() {
    let db = sqlx::SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS user ( id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, username TEXT NOT NULL, email TEXT NOT NULL, password TEXT NOT NULL);").execute(&db).await.unwrap();
}

pub async fn insert_user<'a>(username: &'a str, email: &'a str, password: &'a str) {
    let db = sqlx::SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query(&format!(
        "INSERT INTO user (username, email, password) VALUES ('{}', '{}', '{}');",
        username, email, password
    ))
    .execute(&db)
    .await
    .unwrap();
}

pub async fn get_user_by_email<'a>(email: &'a str) -> SqliteRow {
    let db = sqlx::SqlitePool::connect(DB_URL).await.unwrap();
    let row = sqlx::query(&format!(
        "SELECT username, email FROM user WHERE email = '{}';",
        email
    ))
    .fetch_one(&db)
    .await
    .unwrap();
    row
}
