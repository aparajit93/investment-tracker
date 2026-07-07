use anyhow::Result;
use sqlx::sqlite::SqlitePool;

use crate::models::Portfolio;

pub async fn create(pool: &SqlitePool, portfolio: &Portfolio) -> Result<()> {
    sqlx::query("INSERT INTO portfolios (id, name, base_currency, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)")
    .bind(&portfolio.id)
    .bind(&portfolio.name)
    .bind(&portfolio.base_currency)
    .bind(&portfolio.created_at)
    .bind(&portfolio.updated_at)
    .execute(pool)
    .await?;    
    Ok(())
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<Portfolio>, sqlx::Error> {
    sqlx::query_as::<_, Portfolio>("SELECT 
    id, name, base_currency, created_at, updated_at 
    FROM portfolios 
    ORDER BY created_at ASC").fetch_all(pool).await
}