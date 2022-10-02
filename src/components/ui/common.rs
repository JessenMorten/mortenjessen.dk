use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TextElementProps {
    pub children: Children,
}

#[function_component(P)]
pub fn paragraph(props: &TextElementProps) -> Html {
    html! {
        <p class={"mb-4 text-slate-900 dark:text-slate-100 text-lg"}>
        { for props.children.iter() }
        </p>
    }
}

#[function_component(H1)]
pub fn heading_one(props: &TextElementProps) -> Html {
    html! {
        <h1 class={"mb-8 text-slate-900 dark:text-slate-100 font-extrabold text-4xl sm:text-5xl md:text-6xl"}>
            { for props.children.iter() }
        </h1>
    }
}

#[function_component(H2)]
pub fn heading_two(props: &TextElementProps) -> Html {
    html! {
        <h1 class={"mb-4 text-slate-600 dark:text-slate-400 font-extrabold text-2xl sm:text-3xl md:text-4xl"}>
            { for props.children.iter() }
        </h1>
    }
}
