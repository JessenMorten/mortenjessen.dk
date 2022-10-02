use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub value: String,
    pub on_change: Callback<String>,
    pub input_type: String,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let Props { value, on_change, input_type } = props.clone();

    let oninput = Callback::from(move |input_event: InputEvent| {
        on_change.emit(get_value_from_input_event(input_event));
    });

    html! {
        <input
            placeholder={"search ..."}
            class={"mb-4 transition-all bg-white shadow-xl text-red-500 dark:text-red-400 dark:bg-slate-800 dark:focus:bg-slate-700 font-semibold w-full rounded-xl px-4 py-2 text-lg outline-none"}
            type={input_type} {value} {oninput} />
    }
}
