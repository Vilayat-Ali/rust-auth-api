use sqlx::{migrate::MigrateDatabase, Sqlite};
use std::collections::HashMap;

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

pub enum Table {
    User,
}

impl Table {
    pub fn generate_schema_user<'a>() -> HashMap<&'a str, &'a str> {
        let mut schema: HashMap<&str, &str> = HashMap::new();
        schema.insert("username", "TEXT NOT NULL");
        schema.insert("email", "TEXT NOT NULL");
        schema.insert("password", "TEXT NOT NULL");
        schema
    }
}

pub async fn create_table<'a>(table_name: &'a str, schema: HashMap<&'a str, &'a str>) {
    let db = sqlx::SqlitePool::connect(DB_URL).await.unwrap();
    let sql_query = format!(
        "CREATE TABLE IF NOT EXISTS {} ({});",
        table_name,
        schema
            .iter()
            .enumerate()
            .map(|data| {
                let index = data.0;
                let schema_data = data.1;
                let mut query = format!(" {} {}", schema_data.0, schema_data.1);
                if index == *&schema.len() - 1 {
                    query.push_str(", ");
                }
                query
            })
            .collect::<Vec<String>>()
            .join("")
    );
    println!("{}", sql_query);
    let result = sqlx::query(&sql_query).execute(&db).await.unwrap();
}
