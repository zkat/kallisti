pub mod commands;

pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::notify,])
        .run(tauri::generate_context!())
}
