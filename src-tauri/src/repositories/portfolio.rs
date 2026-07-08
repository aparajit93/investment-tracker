use anyhow::Result;
use sqlx::sqlite::SqlitePool;

use crate::models::Portfolio;

use super::error::RepositoryError;

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

pub async fn update(pool: &SqlitePool, portfolio: &Portfolio) -> Result<(), RepositoryError> {
    let result = sqlx::query("UPDATE portfolios
    SET name = $1,
        base_currency = $2,
        updated_at = $3
    WHERE id = $4")
    .bind(&portfolio.name)
    .bind(&portfolio.base_currency)
    .bind(portfolio.updated_at)
    .bind(&portfolio.id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(RepositoryError::NotFound);
    }

    Ok(())
}