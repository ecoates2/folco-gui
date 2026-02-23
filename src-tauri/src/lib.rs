mod state;

use folco_core::SerializableFolderIconBase;
use state::AppState;
use tauri::Manager;

#[tauri::command]
fn get_folder_icon_base(state: tauri::State<AppState>) -> Result<SerializableFolderIconBase, String> {
    state.get_folder_icon_base()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new().expect("Failed to initialize app state");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_prevent_default::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![get_folder_icon_base])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
