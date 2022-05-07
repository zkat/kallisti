mod app_state;
pub mod commands;

pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .manage(app_state::AppState::new_wrapped())
        .invoke_handler(tauri::generate_handler![commands::hello])
        .invoke_handler(tauri::generate_handler![commands::notify])
        .run(tauri::generate_context!())
}
