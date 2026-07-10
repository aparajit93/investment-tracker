use tauri::State;

use crate::{
    models::{Account, AccountType},
    repositories::{self, RepositoryError},
    state::AppState,
};

#[tauri::command]
pub async fn create_account(
    state: State<'_, AppState>,
    portfolio_id: String,
    name: String,
    institution: Option<String>,
    account_type: String,
    currency: String,
) -> Result<(), String> {
    let account_type = account_type
        .parse::<AccountType>()
        .map_err(|e| e.to_string())?;
    let account = Account::new(
        &portfolio_id,
        &name,
        institution.as_deref(),
        account_type,
        &currency,
    )
    .map_err(|e| e.to_string())?;
    repositories::account::create(&state.db, &account)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn list_accounts(
    state: State<'_, AppState>,
    portfolio_id: String,
) -> Result<Vec<Account>, String> {
    repositories::account::list(&state.db, &portfolio_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn rename_account(
    state: State<'_, AppState>,
    id: String,
    name: String,
) -> Result<(), String> {
    let mut account = repositories::account::get_by_id(&state.db, &id)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    account.rename(&name).map_err(|e| e.to_string())?;

    repositories::account::update(&state.db, &account)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;
    Ok(())
}

#[tauri::command]
pub async fn change_account_currency(
    state: State<'_, AppState>,
    id: String,
    currency: String,
) -> Result<(), String> {
    let mut account = repositories::account::get_by_id(&state.db, &id)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    account
        .change_currency(&currency)
        .map_err(|e| e.to_string())?;

    repositories::account::update(&state.db, &account)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    Ok(())
}

#[tauri::command]
pub async fn change_account_institution(
    state: State<'_, AppState>,
    id: String,
    institution: Option<String>,
) -> Result<(), String> {
    let mut account = repositories::account::get_by_id(&state.db, &id)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    account.change_institution(institution.as_deref());

    repositories::account::update(&state.db, &account)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    Ok(())
}

#[tauri::command]
pub async fn change_account_type(
    state: State<'_, AppState>,
    id: String,
    account_type: String,
) -> Result<(), String> {
    let mut account = repositories::account::get_by_id(&state.db, &id)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    let account_type = account_type
        .parse::<AccountType>()
        .map_err(|e| e.to_string())?;
    account.change_account_type(account_type);

    repositories::account::update(&state.db, &account)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    Ok(())
}

#[tauri::command]
pub async fn activate_account(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let mut account = repositories::account::get_by_id(&state.db, &id)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    account.activate();

    repositories::account::update(&state.db, &account)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    Ok(())
}

#[tauri::command]
pub async fn deactivate_account(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let mut account = repositories::account::get_by_id(&state.db, &id)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    account.deactivate();

    repositories::account::update(&state.db, &account)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;

    Ok(())
}

#[tauri::command]
pub async fn delete_account(state: State<'_, AppState>, id: String) -> Result<(), String> {
    repositories::account::delete(&state.db, &id)
        .await
        .map_err(|e| match e {
            RepositoryError::NotFound => "Account Not Found".to_string(),
            RepositoryError::Database(err) => err.to_string(),
        })?;
    Ok(())
}
