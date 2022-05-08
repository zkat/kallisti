use yew::prelude::*;

use crate::components::Login;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Login />
        </>
    }
}
