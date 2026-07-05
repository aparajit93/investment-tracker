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