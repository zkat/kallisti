use kallisti_command_error::KallistiCommandError;
use matrix_sdk::{ruma::UserId, Client};

use crate::app_state::AppStateWrapper;

#[tauri::command]
pub async fn login(
    state: tauri::State<'_, AppStateWrapper>,
    user_id: &str,
    password: &str,
) -> Result<(), KallistiCommandError> {
    let user_id =
        UserId::try_from(user_id).map_err(|e| KallistiCommandError::UserIdParseError {
            id: user_id.to_string(),
            message: e.to_string(),
        })?;

    let client = Client::new_from_user_id(user_id.clone())
        .await
        .map_err(|e| KallistiCommandError::MatrixClientCreationError(e.to_string()))?;

    client.login(user_id.localpart(), password, None, None).await
        .map_err(|e| KallistiCommandError::MatrixLoginError(e.to_string()))?;

    println!("Logged in as {}", user_id);

    let cloned_client = client.clone();
    tauri::async_runtime::spawn(async move {
        println!("Syncing client.");
        cloned_client.sync(Default::default()).await
    });

    let lock = state.lock().expect("Failed to lock app state");
    let mut kallisti = lock.borrow_mut();

    kallisti.insert_client(user_id, client);

    Ok(())
}
