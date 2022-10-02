use crate::components::{
    app::Route,
    ui::page::Page,
    ui::{
        card::{Card, CardColor},
        common::H2,
        grid::Grid,
    },
};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let history = use_history().unwrap();

    let open_http_status_codes = {
        let h = history.clone();
        Callback::from(move |_| h.to_owned().push(Route::HttpStatusCodes))
    };

    let open_docker_commands = {
        let h = history.clone();
        Callback::from(move |_| h.to_owned().push(Route::NotFound))
    };

    html! {
        <Page title={"mortenjessen.dk"}>
            <H2>{"things i keep forgetting"}</H2>

            <Grid>
                <Card
                    title="http status codes"
                    onclick={open_http_status_codes}
                    color={CardColor::Red} />
                <Card
                    title="docker commands"
                    onclick={open_docker_commands}
                    color={CardColor::Blue} />
            </Grid>
        </Page>
    }
}
