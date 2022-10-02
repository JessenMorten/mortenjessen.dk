use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/httpstatuscodes")]
    HttpStatusCodes,

    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html!(<crate::components::pages::home::HomePage />),
        Route::HttpStatusCodes => html!(<crate::components::pages::http_status_codes::HttpStatusCodesPage /> ),
        Route::NotFound => html! (<crate::components::pages::not_found::NotFoundPage />),
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
