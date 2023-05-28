use sqlx::{migrate::MigrateDatabase, sqlite::SqliteRow, Row, Sqlite};

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
    let db: sqlx::Pool<Sqlite> = sqlx::SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS user ( id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, username TEXT NOT NULL, email TEXT NOT NULL, password TEXT NOT NULL);").execute(&db).await.unwrap();
}

pub async fn insert_user<'a>(username: &'a str, email: &'a str, password: &'a str) {
    let db: sqlx::Pool<Sqlite> = sqlx::SqlitePool::connect(DB_URL).await.unwrap();
    sqlx::query(&format!(
        "INSERT INTO user (username, email, password) VALUES ('{}', '{}', '{}');",
        username, email, password
    ))
    .execute(&db)
    .await
    .unwrap();
}

pub async fn get_user_by_email<'a>(email: &'a str) -> (String, String, String) {
    let db: sqlx::Pool<Sqlite> = sqlx::SqlitePool::connect(DB_URL).await.unwrap();
    let row: Result<SqliteRow, sqlx::Error> = sqlx::query(&format!(
        "SELECT username, email, password FROM user WHERE email = '{}';",
        email
    ))
    .fetch_one(&db)
    .await;

    match row {
        Ok(row_data) => {
            return (
                row_data.get::<String, usize>(0),
                row_data.get::<String, usize>(1),
                row_data.get::<String, usize>(2),
            );
        }
        Err(_) => return (String::default(), String::default(), String::default()),
    };
}
