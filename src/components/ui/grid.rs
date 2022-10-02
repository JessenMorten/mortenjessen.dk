use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct GridProps {
    pub children: Children,
}

#[function_component(Grid)]
pub fn grid(props: &GridProps) -> Html {
    html! {
        <div class={"mb-4 grid grid-cols-1 md:grid-cols-3 xl:grid-cols-5 gap-4"}>
            { for props.children.iter() }
        </div>
    }
}