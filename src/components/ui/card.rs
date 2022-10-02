use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum CardColor {
    Red,
    Blue,
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub onclick: Callback<MouseEvent>,
    pub color: CardColor,
    pub title: String,
}

const DEFAULT_CLASSES: &str = "
transition-all
hover:shadow-xl
p-4
text-white
font-semibold
rounded-xl
shadow
outline-none";

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let mut classes = vec![DEFAULT_CLASSES];

    classes.push(match props.color {
        CardColor::Red => "bg-gradient-to-r from-red-600 to-rose-400",
        CardColor::Blue => "bg-gradient-to-r from-blue-500 to-cyan-500",
    });

    html! {
        <div class={classes.join(" ")}>
            <h3 class={"text-center md:text-left text-2xl md:text-xl mb-4"}>{props.title.to_owned()}</h3>
            <button
            onclick={props.onclick.to_owned()}
            class={"transition-all
            group
            py-2
            w-full
            bg-slate-900
            px-4
            text-white
            font-semibold
            rounded-xl
            shadow
            hover:shadow-xl
            active:bg-slate-700
            outline-none"}
            >
            <svg class="mx-auto group-hover:animate-pulse" width="28" height="28" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="48" d="M268 112l144 144-144 144M392 256H100"/></svg>
            </button>
        </div>
    }
}
