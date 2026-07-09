use tauri::State;

use crate::{
    models::Portfolio,
    repositories::{self, RepositoryError},
    state::AppState,
};

#[tauri::command]
pub async fn create_portfolio(
    state: State<'_, AppState>,
    name: String,
    base_currency: String,
) -> Result<(), String> {
    let portfolio = Portfolio::new(&name, &base_currency).map_err(|e| e.to_string())?;
    repositories::portfolio::create(&state.db, &portfolio)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn list_portfolios(state: State<'_, AppState>) -> Result<Vec<Portfolio>, String> {
    repositories::portfolio::list(&state.db)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn rename_portfolio(
    state: State<'_, AppState>,
    id: String,
    name: String,
) -> Result<(), String> {
    let mut portfolio = repositories::portfolio::get_by_id(&state.db, &id)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Portfolio Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    portfolio.rename(&name).map_err(|e| e.to_string())?;

    repositories::portfolio::update(&state.db, &portfolio)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Portfolio Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    Ok(())
}

#[tauri::command]
pub async fn change_portfolio_base_currency(
    state: State<'_, AppState>,
    id: String,
    base_currency: String,
) -> Result<(), String> {
    let mut portfolio = repositories::portfolio::get_by_id(&state.db, &id)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Portfolio Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    portfolio.change_base_currency(&base_currency).map_err(|e| e.to_string())?;

    repositories::portfolio::update(&state.db, &portfolio)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Portfolio Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    Ok(())
}

#[tauri::command]
pub async fn delete_portfolio(state: State<'_, AppState>, id: String) -> Result<(), String> {
    repositories::portfolio::delete(&state.db, &id)
    .await
    .map_err(|e| match e {
            RepositoryError::NotFound => "Portfolio Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;
  Ok(())
}
