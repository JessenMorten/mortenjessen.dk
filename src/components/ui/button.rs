use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonState {
    Enabled,
    Disabled,
    Loading,
}

#[derive(Clone, PartialEq)]
pub enum ButtonColor {
    Red,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub children: Children,
    pub onclick: Callback<MouseEvent>,
    pub state: ButtonState,
    pub color: ButtonColor,
}

const DEFAULT_CLASSES: &str = "
transition-all
py-2
px-4
text-white
font-semibold
rounded-xl
shadow
outline-none";

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let mut classes = vec![DEFAULT_CLASSES];

    classes.push(match props.state {
        ButtonState::Enabled => "cursor-pointer",
        ButtonState::Disabled => "cursor-not-allowed",
        ButtonState::Loading => "cursor-wait animate-pulse",
    });

    classes.push(match props.color {
        ButtonColor::Red => "bg-red-600 hover:bg-red-700 active:bg-red-900 disabled:bg-red-400",
    });

    let is_disabled = match props.state {
        ButtonState::Disabled => true,
        ButtonState::Loading => true,
        _ => false,
    };

    html! {
        <button disabled={is_disabled} class={classes.join(" ")} onclick={&props.onclick}>
            { for props.children.iter() }
        </button>
    }
}
