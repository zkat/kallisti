use kallisti_command_error::KallistiCommandError;
use tauri::api::notification::Notification;

use crate::app_state::AppStateWrapper;

#[tauri::command]
pub fn notify(
    app_handle: tauri::AppHandle,
    state: tauri::State<AppStateWrapper>,
    title: &str,
    message: &str,
) -> Result<(), KallistiCommandError> {
    let id = app_handle.config().tauri.bundle.identifier.clone();
    let locked = state.inner().lock().expect("faile to lock");
    let mut state = locked.borrow_mut();
    state.notifications += 1;
    Notification::new(id)
        .title(title)
        .body(format!("{} (#{})", message, state.notifications))
        .show()
        .map_err(|e| KallistiCommandError::GenericError(format!("{}", e)))?;
    Ok(())
}
