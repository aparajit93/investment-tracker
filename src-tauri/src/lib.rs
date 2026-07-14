// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod db;
mod models;
mod repositories;
mod state;
mod utils;

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
        .invoke_handler(tauri::generate_handler![
            greet,
            // PORTFOLIO
            commands::portfolio::create_portfolio,
            commands::portfolio::list_portfolios,
            commands::portfolio::rename_portfolio,
            commands::portfolio::change_portfolio_base_currency,
            commands::portfolio::delete_portfolio,
            commands::portfolio::get_portfolio,
            // ACCOUNT
            commands::account::create_account,
            commands::account::list_accounts,
            commands::account::rename_account,
            commands::account::change_account_currency,
            commands::account::change_account_institution,
            commands::account::change_account_type,
            commands::account::activate_account,
            commands::account::deactivate_account,
            commands::account::delete_account
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            tauri::async_runtime::block_on(async move {
                let pool = db::initialize(&app_handle)
                    .await
                    .expect("Database Creation Failed");

                app.manage(AppState { db: pool });
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
