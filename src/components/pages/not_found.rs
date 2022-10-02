use crate::components::{
    app::Route,
    ui::{
        button::{Button, ButtonColor, ButtonState},
        common::H2,
        page::Page,
    },
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    let history = use_history().unwrap();
    let go_home = Callback::once(move |_| history.push(Route::Home));

    html! {
        <Page title={"mortenjessen.dk"}>
            <H2>{"page not found"}</H2>
            <Button
                onclick={go_home}
                state={ButtonState::Enabled}
                color={ButtonColor::Red}>{"Go home"}</Button>
        </Page>
    }
}
