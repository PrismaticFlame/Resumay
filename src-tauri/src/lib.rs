// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod db;
mod latex;

use std::sync::Mutex;
use tauri::Manager;

pub struct DbState(pub Mutex<rusqlite::Connection>);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_application(state: tauri::State<DbState>, company: &str, position: &str, website: Option<&str>, status: &str, 
                    resume: Option<i32>, notes: Option<&str>, refer: Option<&str>, apply_date: &str) -> Result<i32, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::add_application(&conn, company, position, website, status, resume, notes, refer, apply_date).map_err(|e| e.to_string())
}

#[tauri::command]
fn list_applications(state: tauri::State<DbState>) -> Result<Vec<db::Application>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    db::list_applications(&conn).map_err(|e| e.to_string())
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
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let tex = r"\documentclass[12pt]{article}\begin{document}Hello world!$Hello world!$\end{document}";
                match latex::compile(&handle, tex).await {
                    Ok(pdf) => println!("PDF compiled: {} bytes", pdf.len()),
                    Err(e) => eprintln!("Compile failed:\n{e}"),
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, add_application, list_applications])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
