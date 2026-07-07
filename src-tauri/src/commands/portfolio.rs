use tauri::State;

use crate::{models::Portfolio, repositories, state::AppState};

#[tauri::command]
pub async fn create_portfolio(state: State<'_, AppState>, name: String, base_currency: String) -> Result<(), String> {
    let portfolio = Portfolio::new(&name, &base_currency).map_err(|e| e.to_string())?;
    repositories::portfolio::create(&state.db, &portfolio).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn list_portfolios(state: State<'_, AppState>) -> Result<Vec<Portfolio>, String> {
  repositories::portfolio::list(&state.db).await.map_err(|e| e.to_string())
}
