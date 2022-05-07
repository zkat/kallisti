use std::collections::HashMap;

use kallisti_error::KallistiError;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

use crate::tauri_api::tauri;

#[function_component(App)]
pub fn app() -> Html {
    let welcome = use_state_eq(|| "".to_string());
    let name = use_state_eq(|| "bl ah".to_string());

    // Execute tauri command via effects.
    // The effect will run every time `name` changes.
    {
        let welcome = welcome.clone();
        use_effect_with_deps(
            move |name| {
                update_welcome_message(welcome, name.clone());
                || ()
            },
            (*name).clone(),
        );
    }

    let message = (*welcome).clone();

    html! {
        <div>
            <h2 class={"heading"}>{message}</h2>
        </div>
    }
}

fn update_welcome_message(welcome: UseStateHandle<String>, name: String) {
    spawn_local(async move {
        match hello(&name).await {
            Ok(message) => {
                welcome.set(message);
            }
            Err(e) => {
                let window = window().unwrap();
                window
                    .alert_with_message(&format!(
                        "Error: {:?}", e
                    ))
                    .unwrap();
            }
        }
    });
}

async fn hello(name: &str) -> Result<String, KallistiError> {
    tauri::invoke(
        "hello",
        Some(HashMap::from([("name".into(), name.to_string())])),
    )
    .await
    .map(|n| n.as_string().expect("Failed to convert result to string."))
}
