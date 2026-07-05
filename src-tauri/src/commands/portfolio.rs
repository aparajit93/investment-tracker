use tauri::State;

use crate::{models::Portfolio, repositories, state::AppState};

#[tauri::command]
pub async fn create_portfolio(state: State<'_, AppState>, name: String, base_currency: String) -> Result<(), String> {
    let portfolio = Portfolio::new(&name, &base_currency).map_err(|e| e.to_string())?;
    repositories::portfolio::create(&state.db, &portfolio).await.map_err(|e| e.to_string())?;
    Ok(())
}
