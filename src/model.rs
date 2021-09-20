use sqlx::SqlitePool;
use serde;

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Asset {
        pub upc: i64,
        pub count: i32,
        pub unit: String,
        pub common_name: String
}

impl Asset {
        pub async fn register(&self, pool: &SqlitePool) -> Result<i64, sqlx::Error> {
                Ok(sqlx::query(
                        r#"INSERT INTO 'assets' ('upc', 'count', 'unit', 'common_name') VALUES (?, ?, ?, ?)"#)
                        .bind(self.upc).bind(self.count).bind(&self.unit).bind(&self.common_name)
                        .execute(&mut pool.acquire().await?).await?.last_insert_rowid())
        }

        pub async fn get(pool: &SqlitePool) -> Result<Asset, sqlx::Error> {
                Ok(sqlx::query_as::<_, Asset>(r#"SELECT * FROM assets WHERE common_name = "beans""#)
                        .fetch_one(&mut pool.acquire().await?).await?)
        }

        pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Asset>, sqlx::Error> {
                Ok(sqlx::query_as::<_, Asset>(r#"SELECT * FROM assets"#)
                        .fetch_all(&mut pool.acquire().await?).await?)
        }
}
