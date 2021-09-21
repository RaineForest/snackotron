use serde;

use crate::db_constants::*;
use sqlx::{Done, FromRow};


#[derive(sqlx::Type, serde::Serialize)]
#[sqlx(rename = "package", rename_all = "lowercase")]
pub enum Package {
        Whole,
        Partial
}

#[derive(sqlx::FromRow, serde::Serialize)]
#[sqlx(rename = "pantry")]
pub struct Pantry {
        pub upc: i64,
        pub amount: i32,
        pub unit: String,
        pub package_type: Package,
        pub brand: String
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Food {
        pub id: i64,
        pub name: String,
        pub desc: String,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Tags {
        pub id: i64,
        pub food: i64, // Food::id
        pub upc: i64, // Pantry::upc
}

impl Pantry {
        pub async fn register(&self, pool: &DBPool) -> Result<u64, sqlx::Error> {
                Ok(sqlx::query(
                        r#"INSERT INTO pantry (upc, amount, unit, package_type, brand) VALUES ($1, $2, $3, $4, $5)"#)
                        .bind(self.upc).bind(self.amount).bind(&self.unit).bind(&self.package_type).bind(&self.brand)
                        .execute(&mut pool.acquire().await?).await?.rows_affected())
        }

        pub async fn get_all(pool: &DBPool) -> Result<Vec<Pantry>, sqlx::Error> {
                Ok(sqlx::query_as::<_, Pantry>(r#"SELECT upc, amount, unit, package_type as package_type, brand FROM pantry"#)
                        .fetch_all(&mut pool.acquire().await?).await?)
        }
}
