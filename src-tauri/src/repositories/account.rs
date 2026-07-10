use anyhow::Result;
use sqlx::sqlite::SqlitePool;

use crate::models::Account;

use super::RepositoryError;

pub async fn create(pool: &SqlitePool, account: &Account) -> Result<(), RepositoryError> {
    sqlx::query("INSERT INTO accounts (id, portfolio_id, name, institution, account_type, currency, is_active, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
    .bind(&account.id)
    .bind(&account.portfolio_id)
    .bind(&account.name)
    .bind(&account.institution)
    .bind(account.account_type)
    .bind(&account.currency)
    .bind(account.is_active)
    .bind(account.created_at)
    .bind(account.updated_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list(pool: &SqlitePool, portfolio_id: &str) -> Result<Vec<Account>, RepositoryError> {
    let accounts = sqlx::query_as::<_, Account>(
        "SELECT 
    id, portfolio_id, name, institution, account_type, currency, is_active, created_at, updated_at 
    FROM accounts
    WHERE portfolio_id = $1 
    ORDER BY created_at ASC",
    )
    .bind(portfolio_id)
    .fetch_all(pool)
    .await?;

    Ok(accounts)
}

pub async fn update(pool: &SqlitePool, account: &Account) -> Result<(), RepositoryError> {
    let result = sqlx::query(
        "UPDATE accounts
    SET name = $1,
        institution = $2,
        account_type = $3,
        currency = $4,
        is_active = $5,
        updated_at = $6
    WHERE id = $7",
    )
    .bind(&account.name)
    .bind(&account.institution)
    .bind(account.account_type)
    .bind(&account.currency)
    .bind(account.is_active)
    .bind(account.updated_at)
    .bind(&account.id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(RepositoryError::NotFound);
    }

    Ok(())
}

pub async fn get_by_id(pool: &SqlitePool, id: &str) -> Result<Account, RepositoryError> {
    let account = sqlx::query_as::<_, Account>(
        "SELECT 
    id, portfolio_id, name, institution, account_type, currency, is_active, created_at, updated_at 
    FROM accounts 
    WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match account {
        Some(account) => Ok(account),
        None => Err(RepositoryError::NotFound),
    }
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<(), RepositoryError> {
    let result = sqlx::query("DELETE FROM accounts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(RepositoryError::NotFound);
    }

    Ok(())
}
