use kallisti_command_error::KallistiCommandError;

#[tauri::command]
pub fn hello(name: &str) -> Result<String, KallistiCommandError> {
    // This is a very simplistic example but it shows how to return a Result
    // and use it in the front-end.
    if name.contains(' ') {
        Err(KallistiCommandError::GenericError("Name should not contain spaces".to_string()))
    } else {
        Ok(format!("Hello, {}", name))
    }
}
