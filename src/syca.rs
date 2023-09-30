use std::ops::Range;
use sycamore::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Event, HtmlInputElement};
use crate::test_scoping;
use std::borrow::BorrowMut;
use wasm_bindgen::JsCast;
use crate::data_include::get_colors_array;


#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Prop)]
pub struct MyProps<'a> {
    value: &'a ReadSignal<i32>,
}

#[component]
pub fn App<'a, G: Html>(cx: Scope, props: MyProps<'a>) -> View<G> {
    let state = create_signal(cx, 0i32);
    let increment = move |_| state.set(*state.get() + 1);
    let decrement = move |e: Event| {
        state.set(*state.get() - 1);
        test_scoping("test".to_string());
    };
    let reset = move |_| state.set(0);


    view! { cx,
        div {
            p { "Value: " (state.get()) }
            button(on:click=increment) { "+" }
            button(on:click=decrement) { "-" }
            button(on:click=reset) { "Reset" }
        }
    }
}



#[component(inline_props)]
fn ColorInput<'a, G: Html>(cx: Scope<'a>, value: &'a ReadSignal<i32>) -> View<G> {
    view! { cx,
        div(class="my-component") {
            "My component"
            p {
                "Value: "
                (value.get())
            }
        }
    }
}


#[derive(Debug, Clone)]
pub struct PlayerConfig{
    player_count:i32,
    player_colors:Vec<String>,
    player_is_ai:Vec<bool>,
}

impl PlayerConfig {
    fn new()->PlayerConfig{
        PlayerConfig{
            player_count: 2,
            player_colors: vec![],
            player_is_ai: vec![],
        }
    }
}


struct ColorSelector {
    player_num:i32,
    is_ai:Signal<bool>,
    color:Signal<String>
}

#[component]
pub fn TestApp<G: Html>(cx: Scope) -> View<G> {
    let max_players = 6i32;
    let min_players = 2i32;
    let next_sig = create_signal(cx, true);


    let player_config_sig:&Signal<PlayerConfig> = create_signal(cx, PlayerConfig::new());

    let next = move || {
        let mut tmp_player_config = (*player_config_sig.get()).clone();
        let player_count = &(*player_config_sig.get()).player_count;
        while tmp_player_config.player_colors.len() < *player_count as usize{
            tmp_player_config.player_colors.push("empty".to_string());
        }
        while tmp_player_config.player_is_ai.len() < *player_count as usize{
            tmp_player_config.player_is_ai.push(false);
        }
        player_config_sig.set(tmp_player_config);
        next_sig.set(!*next_sig.get());
        console_log!("{:?}", *player_config_sig.get());
    };

    let state = create_memo(cx, move ||{
        return (*player_config_sig.get()).player_count;
    } );

    let num_players = create_signal(cx, 0);

    let player_options = View::new_fragment(
        (min_players..max_players + 1).map(|x| view! { cx, option(value=x, on:click= move |e:Event| {
            let mut tmp_player_config = (*player_config_sig.get()).clone();
            tmp_player_config.player_count = x;
            player_config_sig.set(tmp_player_config);
        }) { (x) } }
        ).collect()
    );

    let colors = get_colors_array().map(|v| v.to_string());



    let player_config_iter_sig = create_memo(cx, move||{
        let ret:Vec<i32> = (1..(*player_config_sig.get()).player_count + 1).collect();
        ret
    });

    let checkbox_handle = move |e:Event| {
        let input_elem = e.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().clone();
        let is_checked = input_elem.checked();
        let idx: usize = input_elem.value().parse().unwrap();
        let mut tmp_player_config = (*player_config_sig.get()).clone();
        tmp_player_config.player_is_ai[idx - 1] = is_checked;
        player_config_sig.set(tmp_player_config);
    };
/*
todo iterate over a vec of stucts containing signals that get passed to children?
*/

/*    let on_ai_tick_change =  |player_num:i32, value:bool|{
        (*player_config.borrow_mut().get()).player_is_ai[player_num as usize] = value;
    };*/

    view! { cx,
    div {
        (if *next_sig.get() {
            view! { cx, select( class="form-select", style="width: fit-content") {
                option(){
                    "Choose the number of players"
                }
                (player_options)
            }}
        } else {
            view! { cx,
                Keyed( iterable=player_config_iter_sig, key=|x| *x, view=move |cx, x| view! { cx,
                    div{
                        label{"Player " (x)}
                        input(type="checkbox", id=format!("player_{}_is_ai", x), value=x,
                            on:change= move |e| checkbox_handle(e) )

                        label(for=format!("player_{}_is_ai", x) ){"is AI"}
                    }

            },
            )}

            //(color_options)
        })
            button(id="run", type="button", class="btn btn-primary", on:click= move |_| next()){
                "Next"
            }
            ColorInput(value=state)
        }
    }
}

