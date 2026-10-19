// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod db;
mod latex;
mod credentials;

use std::sync::Mutex;
use tauri::Manager;
use tauri::AppHandle;
use keyring_core::{set_default_store};
// use tauri_plugin_shell::ShellExt;

pub struct DbState(pub Mutex<rusqlite::Connection>);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_application(state: tauri::State<DbState>, company: &str, position: &str, website: Option<&str>, username: Option<&str>, status: &str, 
                    resume: Option<i32>, notes: Option<&str>, refer: Option<&str>, apply_date: &str, password: Option<&str>) -> Result<i32, String> {
    let id = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        db::add_application(&conn, company, position, website, username, status, resume, notes, refer, apply_date).map_err(|e| e.to_string())?
    };
    if let Some(pw) = password.filter(|p| !p.is_empty()) {
        credentials::save_credential(id, pw)?;
    }
    Ok(id)
}

#[tauri::command]
fn list_applications(state: tauri::State<DbState>) -> Result<Vec<db::Application>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::list_applications(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
async fn compile_resume(app: AppHandle, tex: String) -> Result<Vec<u8>, String> {
    latex::compile(&app, &tex).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let conn = rusqlite::Connection::open(dir.join("resumay.db"))?;
            db::init(&conn)?;
            app.manage(DbState(Mutex::new(conn)));
            let store = dbus_secret_service_keyring_store::Store::new().map_err(|e| format!("create store: {e}"))?;
            set_default_store(store);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet, 
            add_application, 
            list_applications,
            credentials::save_credential,
            credentials::get_credential,
            credentials::delete_credential,
            compile_resume])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
