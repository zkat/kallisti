use yew::prelude::*;

use crate::components::{Login, NotificationButton};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Login />
            <NotificationButton />
        </>
    }
}
