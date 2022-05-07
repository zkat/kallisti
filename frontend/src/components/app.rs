use yew::prelude::*;

use crate::components::NotificationButton;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <NotificationButton />
        </>
    }
}
