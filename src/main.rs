use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::{JsCast,UnwrapThrowExt};


fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
}

#[derive(Debug,Clone,Properties,PartialEq)]
struct ButtonProps {
    click: Callback<()>,
    children: Children,
}

#[function_component]
fn Button(props: &ButtonProps) -> Html {
    let ButtonProps { children,click} = props.clone();

    let onclick = Callback::from(move |_| {
        click.emit(());
    });

    html!(
        <button class="px-4 py-2 rounded-md font-bold text-3xl text-white bg-blue-500" {onclick}> {children} </button>
    )
}

#[function_component]
fn App() -> Html {
    let state = use_state(|| 0);
    let by = use_state(|| 1);
    let by_error : UseStateHandle<Option<String>> = use_state(|| None);

    let incr_counter = {
        let state = state.clone();
        let by = by.clone();
        Callback::from(move |_| state.set(*state + *by))
    };

    let oninput = {
        let by_error = by_error.clone();
        let by = by.clone();
        Callback::from(move |event: InputEvent| {
        let s = get_value_from_input_event(event);

        match s.parse::<i32>() {
            Ok(e) => {
                by.set(e);
                by_error.set(None);
            }
            Err(_) => {
                by_error.set(Some(format!("{} can not be parsed into a integer.",s)));
            }
        };
    })
    };

    let decr_counter = {
        let state = state.clone();
        let by = by.clone();
        Callback::from(move |_| state.set(*state - *by))
    };

    let reset_counter = {
        let state = state.clone();
        Callback::from(move |_| state.set(0))
    };

    html!(
        <>

            <h1 class="text-5xl text-center mb-16 font-bold"> {*state} </h1>

            <hr />

            <div class="mx-auto w-1/4">
                <div class="mt-16">
                    <input class="w-full" type="number" value={format!("{}",*by)} {oninput} />
                    <p class="text-red-500">{(*by_error).clone().unwrap_or("".into())}</p>
                </div>

                <div class=" mt-5 flex justify-between gap-3 items-center ">
                    <Button click={incr_counter}> {"+"} </Button>
                    <Button click={reset_counter}> {"RESET"} </Button>
                    <Button click={decr_counter}> {"-"} </Button>
                </div>
            </div>
        </>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
