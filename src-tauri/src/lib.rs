mod app_state;
pub mod commands;

pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .manage(app_state::AppStateWrapper::new())
        .invoke_handler(tauri::generate_handler![
            commands::hello,
            commands::login,
            commands::notify,
        ])
        .run(tauri::generate_context!())
}
