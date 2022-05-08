use std::collections::HashMap;

use kallisti_models::LoginInfo;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::future_to_promise;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::tauri_api::tauri;

#[function_component(Login)]
pub fn login() -> Html {
    let model = use_state_eq(LoginInfo::default);

    let on_user_id_change = {
        let model = model.clone();
        move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            model.set(LoginInfo {
                user_id: input.value(),
                ..(*model).clone()
            })
        }
    };
    let on_password_change = {
        let model = model.clone();
        move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            model.set(LoginInfo {
                password: input.value(),
                ..(*model).clone()
            })
        }
    };

    html! {
        <>
            <h3>{"Log In"}</h3>
            <form onsubmit={move |e: FocusEvent| {
                invoke_login(&*model);
                e.prevent_default();
            }}>
                <div class="p-4">
                    <input onchange={on_user_id_change} class="input" type="text" placeholder="User ID" />
                </div>
                <div class="p-4">
                    <input onchange={on_password_change} class="input" type="password" placeholder="Password" />
                </div>
                <button class="btn" type="submit">{"Log In"}</button>
            </form>
        </>
    }
}

#[allow(unused_must_use)]
fn invoke_login(info: &LoginInfo) {
    let info_hash = HashMap::from([
        ("userId".into(), info.user_id.clone()),
        ("password".into(), info.password.clone()),
    ]);
    future_to_promise(async {
        tauri::invoke("login", Some(info_hash)).await.map_or_else(
            |e| {
                let val = JsValue::from_serde(&e).expect("Failed to convert error.");
                gloo::console::log!(&val);
            },
            |_| {
                gloo::console::log!("Logged in successfully");
            },
        );
        Ok(JsValue::NULL)
    });
}
