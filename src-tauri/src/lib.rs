// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod db;
mod state;
mod models;
mod utils;
mod repositories;
mod commands;

use state::AppState;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, 
            // PORTFOLIO
            commands::portfolio::create_portfolio,
            commands::portfolio::list_portfolios,
            commands::portfolio::rename_portfolio])
        .setup(|app| {
            let app_handle = app.handle().clone();

            tauri::async_runtime::block_on(async move {
                let pool = db::initialize(&app_handle).await.expect("Database Creation Failed");

                app.manage(AppState { db: pool});
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
