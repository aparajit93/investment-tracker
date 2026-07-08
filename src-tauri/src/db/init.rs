use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use tauri::AppHandle;
// For the .path() function/method
use tauri::Manager;

const DB_FILE: &str = "investment_tracker.db";

pub async fn initialize(app: &AppHandle) -> Result<SqlitePool> {
    let dir = app.path().app_data_dir()?;

    std::fs::create_dir_all(&dir)?;

    let db_path = dir.join(DB_FILE);

    // SQLX sets the foreign_keys to ON by default.
    // If needed, set options explicitly as below
    // let opts = SqliteConnectOptions[...].foreign_keys(true);
    let opts = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(opts).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
