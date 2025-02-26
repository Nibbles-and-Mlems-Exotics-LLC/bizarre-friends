use std::path::PathBuf;

use sea_orm::Database;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let data_dir: PathBuf = dirs::data_local_dir().unwrap();
    let _database = Database::connect(format!(
        "sqlite://{}?mode=rwc",
        data_dir.join("bizarre_friends/db.sqlite").to_string_lossy()
    ))
    .await
    .unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
