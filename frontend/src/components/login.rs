use kallisti_command_error::KallistiCommandError;
use kallisti_models::LoginInfo;
use matrix_sdk::{ruma::UserId, Client};
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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
    let info = info.clone();
    matrix_sdk::executor::spawn(async move {
        log_in(&info).await.map_or_else(
            |e| {
                let val = JsValue::from_serde(&e).expect("Failed to convert error.");
                gloo::console::log!(&val);
            },
            |_| {
                gloo::console::log!("Logged in successfully");
            },
        );
    });
}

async fn log_in(info: &LoginInfo) -> Result<(), KallistiCommandError> {
    let user = <&UserId>::try_from(info.user_id.as_str()).map_err(|e| {
        KallistiCommandError::UserIdParseError {
            message: format!("{}", e),
            id: info.user_id.clone(),
        }
    })?;
    let client = Client::builder()
        .user_id(user)
        .build()
        .await
        .map_err(|e| KallistiCommandError::MatrixClientCreationError(e.to_string()))?;

    matrix_sdk::executor::spawn(async move {
        client.sync(Default::default()).await;
    });
    Ok(())
}
