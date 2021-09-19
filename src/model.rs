use sqlx::Connection;
use sqlx::SqliteConnection;
use serde;

const SQLITE_URL: &str = "sqlite://snackotron.db";

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Asset {
        pub upc: i64,
        pub count: i32,
        pub unit: String,
        pub common_name: String
}

impl Asset {
        pub async fn register(&self) -> Result<i64, sqlx::Error> {
                let mut conn = SqliteConnection::connect(SQLITE_URL).await?;

                let id: i64 = sqlx::query(
                        r#"INSERT INTO 'assets' ('upc', 'count', 'unit', 'common_name') VALUES (?, ?, ?, ?)"#)
                        .bind(self.upc).bind(self.count).bind(&self.unit).bind(&self.common_name)
                        .execute(&mut conn).await?.last_insert_rowid();

                Ok(id)
        }

        pub async fn get() -> Result<Asset, sqlx::Error> {
                let mut conn = SqliteConnection::connect(SQLITE_URL).await?;

                Ok(sqlx::query_as::<_, Asset>(r#"SELECT * FROM assets WHERE common_name = "beans""#)
                        .fetch_one(&mut conn).await?)
        }

        pub async fn getAll() -> Result<Vec<Asset>, sqlx::Error> {
                let mut conn = SqliteConnection::connect(SQLITE_URL).await?;

                Ok(sqlx::query_as::<_, Asset>(r#"SELECT * FROM assets"#).fetch_all(&mut conn).await?)
        }
}
