use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

pub async fn compile(app: &AppHandle, tex: &str) -> Result<Vec<u8>, String> {
    let dir = std::env::temp_dir().join("temp_latex");
    std::fs::create_dir_all(&dir).map_err(|e| format!("create dir: {e}"))?;

    let tex_path = dir.join("test.tex");
    std::fs::write(&tex_path, tex).map_err(|e| format!("write tex: {e}"))?;

    let output = app
        .shell()
        .sidecar("tectonic")
        .map_err(|e| format!("find sidecar: {e}"))?
        .args([tex_path.to_str().unwrap(), "--outdir", dir.to_str().unwrap()])
        .output()
        .await
        .map_err(|e| format!("run sidecar: {e}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }

    std::fs::read(dir.join("test.pdf")).map_err(|e| e.to_string())
}