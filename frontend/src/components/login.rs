use wasm_bindgen::JsValue;
use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <>
            <h3>{"Log In"}</h3>
            <form onsubmit={|e: FocusEvent| {
                e.prevent_default();
            }}>
                <input type="text" placeholder="Username" />
                <input type="password" placeholder="Password" />
                <button class="btn" type="submit">{"Log In"}</button>
            </form>
        </>
    }
}

