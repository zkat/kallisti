use std::collections::HashMap;

use wasm_bindgen::{prelude::Closure, JsValue};
use wasm_bindgen_futures::future_to_promise;
use yew::prelude::*;

use crate::tauri_api::tauri;

#[function_component(NotificationButton)]
pub fn notification_button() -> Html {
    html! {
        <button type="button" class="btn" onclick={|_| {
            notify()
        }}>
            {"notify me"}
        </button>
    }
}

#[allow(unused_must_use)]
fn notify() {
    future_to_promise(async {
        tauri::invoke(
            "notify",
            Some(HashMap::from([
                ("title".into(), "hey"),
                ("message".into(), "Hello, world!"),
            ])),
        )
        .await
        .map(|x| {
            web_sys::console::log_1(&JsValue::from("foo"));
            x
        })
        .map_err(|e| JsValue::from_serde(&e).expect("Failed to convert error."))
    })
    .catch(&Closure::once(|_| {}));
}
