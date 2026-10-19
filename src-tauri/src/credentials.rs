use keyring_core::{Entry, Error};

fn entry_for(application_id: i32) -> Result<Entry, String> {
    Entry::new("resumay", &format!("app-{application_id}")).map_err(|e| format!("Creating entry: {e}"))
}

#[tauri::command]
pub fn save_credential(application_id: i32, password: &str) -> Result<(), String> {
    let entry = entry_for(application_id)?;
    entry.set_password(password).map_err(|e| format!("Setting password: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn get_credential(application_id: i32) -> Result<Option<String>, String> {
    let entry = entry_for(application_id)?;
    match entry.get_password() {
        Ok(pw) => Ok(Some(pw)),
        Err(Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Getting credential: {e}")),
    }
}

#[tauri::command]
pub fn delete_credential(application_id: i32) -> Result<(), String> {
    let entry = entry_for(application_id)?;
    match entry.delete_credential() {
        Ok(()) | Err(Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Deleting credential: {e}")),
    }
}
