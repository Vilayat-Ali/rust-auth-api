use sqlx::{sqlite::SqliteQueryResult, Connection, Error, Executor, SqliteConnection};
use std::collections::HashMap;

pub struct DB {
    pub db_name: String,
    pub conn: SqliteConnection,
}

impl DB {
    pub async fn connect(db_name: String) -> Result<Self, Error> {
        let conn: SqliteConnection = SqliteConnection::connect("sqlite::memory:").await?;
        Ok(Self { db_name, conn })
    }

    fn query_logger(query: &str) {
        println!(
            "\n\n--------------------------------------------------------------------------------\n\nQuery: {}\n\n--------------------------------------------------------------------------------\n\n",
            query
        );
    }

    pub async fn create_table(
        &mut self,
        table_name: &str,
        schema: HashMap<&'static str, &'static str>,
    ) -> Result<SqliteQueryResult, Error> {
        let mut sql_query: String = format!("CREATE TABLE IF NOT EXISTS {} ( ", table_name);

        for (index, schema_data) in schema.iter().enumerate() {
            sql_query.push_str(&format!("{} {}", schema_data.0, schema_data.1));

            if index != (schema.len() - 1) {
                sql_query.push_str(", ");
            } else {
                sql_query.push_str(");");
            }
        }

        DB::query_logger(sql_query.as_str());

        let res = self.conn.execute(sqlx::query(sql_query.as_str())).await?;
        Ok(res)
    }

    pub async fn insert_into_table<'a>(
        &mut self,
        table_name: &str,
        data_model: HashMap<&'a str, &'a str>,
    ) -> Result<SqliteQueryResult, Error> {
        let sql_query: String = format!(
            "INSERT INTO {} ({}) VALUES ({});",
            table_name,
            &data_model
                .iter()
                .map(|x| format!("{}", x.0))
                .collect::<Vec<String>>()
                .join(","),
            &data_model
                .iter()
                .map(|x| format!("'{}'", x.1))
                .collect::<Vec<String>>()
                .join(","),
        );

        DB::query_logger(sql_query.as_str());

        let res = self.conn.execute(sqlx::query(sql_query.as_str())).await?;
        Ok(res)
    }

    pub async fn remove_from_table(&self, data_model: HashMap<&'static str, &'static str>) {}

    pub async fn update_from_table(&self, data_model: HashMap<&'static str, &'static str>) {}
}
