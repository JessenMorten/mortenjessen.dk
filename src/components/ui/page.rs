use crate::components::{ui::common::H1, app::Route};
use yew::prelude::*;
use yew_router::prelude::{use_history, History};

#[derive(Properties, PartialEq)]
pub struct PageProps {
    pub children: Children,
    pub title: String,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
    let history = use_history().unwrap();

    let open_home = {
        let h = history.clone();
        Callback::from(move |_| h.to_owned().push(Route::Home))
    };

    html! {
        <div class={"container mx-auto p-5"}>
            <button onclick={open_home}>
                <H1>{props.title.to_owned()}</H1>
            </button>

            { for props.children.iter() }
            
            <div class="text-slate-500 mt-32 text-center">
                <p class="mb-4 font-semibold">{"© 2022 mortenjessen.dk"}</p>
                <p class="mb-4 ">
                    <a class="hover:underline" href="https://www.linkedin.com/in/morten-jessen-a1799319b/">{"linkedin"}</a>
                    {" | "}
                    <a class="hover:underline" href="https://github.com/JessenMorten">{"github"}</a>
                </p>
                <p class="mb-4 ">
                    <a class="hover:underline" href="https://github.com/JessenMorten/mortenjessen.dk">{"source"}</a>
                </p>
            </div>
        </div>

    }
}
