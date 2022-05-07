use kallisti_command_error::KallistiCommandError;
use tauri::api::notification::Notification;

#[tauri::command]
pub fn notify(
    app_handle: tauri::AppHandle,
    title: &str,
    message: &str,
) -> Result<(), KallistiCommandError> {
    let id = app_handle.config().tauri.bundle.identifier.clone();
    Notification::new(id)
        .title(title)
        .body(message)
        .show()
        .map_err(|e| KallistiCommandError::GenericError(format!("{}", e)))?;
    Ok(())
}
