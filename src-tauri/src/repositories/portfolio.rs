use anyhow::Result;
use sqlx::sqlite::SqlitePool;

use crate::models::Portfolio;

use super::RepositoryError;

pub async fn create(pool: &SqlitePool, portfolio: &Portfolio) -> Result<(), RepositoryError> {
    sqlx::query("INSERT INTO portfolios (id, name, base_currency, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)")
    .bind(&portfolio.id)
    .bind(&portfolio.name)
    .bind(&portfolio.base_currency)
    .bind(portfolio.created_at)
    .bind(portfolio.updated_at)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list(pool: &SqlitePool) -> Result<Vec<Portfolio>, RepositoryError> {
    let portfolios = sqlx::query_as::<_, Portfolio>(
        "SELECT 
    id, name, base_currency, created_at, updated_at 
    FROM portfolios 
    ORDER BY created_at ASC",
    )
    .fetch_all(pool)
    .await?;

    Ok(portfolios)
}

pub async fn update(pool: &SqlitePool, portfolio: &Portfolio) -> Result<(), RepositoryError> {
    let result = sqlx::query(
        "UPDATE portfolios
    SET name = $1,
        base_currency = $2,
        updated_at = $3
    WHERE id = $4",
    )
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

pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Portfolio, RepositoryError> {
    let portfolio = sqlx::query_as::<_, Portfolio>(
        "SELECT 
    id, name, base_currency, created_at, updated_at 
    FROM portfolios 
    WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match portfolio {
        Some(portfolio) => Ok(portfolio),
        None => Err(RepositoryError::NotFound),
    }
}
