use std::cell::RefCell;
use sycamore::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement};
use crate::data_include::get_colors_array;
use crate::game::Game;


#[derive(Debug, Clone)]
pub struct PlayerConfig{
    pub player_count:i32,
    pub player_colors:Vec<String>,
    pub player_is_ai:Vec<bool>,
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

#[derive(Prop, Clone, Copy)]
pub struct PlayerConfigProps<'a> {
    pub idx:i32,
    data:&'a Signal<PlayerConfig>,
    done:&'a Signal<bool>,
}

#[component]
pub fn PlayerSetup<'a, G: Html>(cx: Scope<'a>, props: PlayerConfigProps<'a>) -> View<G> {

    let checkbox_handle = move |e:Event| {
        let input_elem = e.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().clone();
        let is_checked = input_elem.checked();
        let idx = props.idx as usize;
        let mut tmp_player_config = (*props.data.get()).clone();
        tmp_player_config.player_is_ai[idx - 1] = is_checked;
        props.data.set(tmp_player_config);
    };

    view! { cx,
        div(style=" padding-bottom:15px") {
            label{"Player " (props.idx)}
            div{}
            input(type="checkbox", id=format!("player_{}_is_ai", props.idx),
                on:change= move |e| checkbox_handle(e) )
            label(for=format!("player_{}_is_ai", props.idx) ){"is AI"}
            Color_Setup(idx=props.idx, data=props.data, done=props.done)
        }
    }
}


#[component]
pub fn Color_Setup<'a, G: Html>(cx: Scope<'a>, props: PlayerConfigProps<'a>) -> View<G> {

    let onchange_handle = move |e:Event| {
        let select_elem = e.target().unwrap().dyn_ref::<HtmlSelectElement>().unwrap().clone();
        let is_checked = select_elem.value();
        let idx = props.idx as usize;
        let mut tmp_player_config = (*props.data.get()).clone();
        tmp_player_config.player_colors[idx - 1] = is_checked;
        props.data.set(tmp_player_config);
    };
    let colors = get_colors_array();

    let options = View::new_fragment(
        colors.iter().map(|&color| view! { cx,
            option(value=color ) { (color) } }
        ).collect()
    );

    view! { cx,
        select( class="form-select", style="width: fit-content", on:change=move |e| onchange_handle(e)) {
            option(value="empty"){"choose a color"}
            (options)
        }
    }
}

#[derive(Prop,  Clone)]
pub struct PlayersSetupProps<'a> {
    pub game_ref:& 'a Signal<Rc<RefCell<Game>>>,
    pub done:& 'a Signal<bool>,
}

#[component]
pub fn PlayersSetup<'a, G : Html>(cx: Scope<'a>, props:PlayersSetupProps<'a>) -> View<G> {
    let max_players = 6i32;
    let min_players = 2i32;
    let next_sig = create_signal(cx, true);

    let done_sig:&Signal<bool> = create_signal(cx, false);

    let player_config_sig:&Signal<PlayerConfig> = create_signal(cx, PlayerConfig::new());

    let error_msg_sig:&Signal<String> = create_signal(cx, "".to_string());

    let next = move || {
        let mut tmp_player_config = (*player_config_sig.get()).clone();


        if *next_sig.get() == true{
            let player_count = &(*player_config_sig.get()).player_count;
            while tmp_player_config.player_colors.len() < *player_count as usize{
                tmp_player_config.player_colors.push("empty".to_string());
            }
            while tmp_player_config.player_is_ai.len() < *player_count as usize{
                tmp_player_config.player_is_ai.push(false);
            }
            player_config_sig.set(tmp_player_config);
            next_sig.set(!*next_sig.get());

        }else {
            let mut validated = true;
            let mut _return  = false;

            for color in &tmp_player_config.player_colors{
                if color == "empty"{
                    validated = false;
                    break;
                }
            }

            if !validated{
                if !_return{
                    error_msg_sig.set("All color fields have a color".to_string());
                    _return = true;
                }
            }else{
                let mut found:Vec<String> = Vec::new();
                for color in &tmp_player_config.player_colors{
                    if !found.contains(color){
                        found.push(color.clone());
                    }else{
                        validated = false;
                        break;
                    }
                }
            }
            if !validated{
                if !_return{
                    error_msg_sig.set("All players must have diiferent colors".to_string());
                    _return = true;
                }
            }else{
                (*props.game_ref.get()).borrow_mut().set_player_config(tmp_player_config);
                props.done.set(true);
            }
        }

    };


    let player_options = View::new_fragment(
        (min_players..max_players + 1).map(|x| view! { cx,
            option(value=x, on:click= move |_| {
                let mut tmp_player_config = (*player_config_sig.get()).clone();
                tmp_player_config.player_count = x;
                player_config_sig.set(tmp_player_config);
        }) { (x) } }
        ).collect()
    );

    let player_config_iter_sig = create_memo(cx, move||{
        let ret:Vec<i32> = (1..(*player_config_sig.get()).player_count + 1).collect();
        ret
    });

    view! { cx,
    div {
        (if *next_sig.get() {
            view! { cx,
                select( class="form-select", style="width: fit-content") {
                    option(){"Choose the number of players"}
                    (player_options)
            }}
        } else {
            view! { cx,
                Keyed( iterable=player_config_iter_sig, key=|x| *x, view=move |cx, x| view! { cx,
                    div{
                        PlayerSetup(idx=x, data=player_config_sig, done=done_sig)
                    }
            },)}
        })
            button(id="run", type="button", class="btn btn-primary", on:click= move |_| next()){
                "Next"
            }
            label(){(*error_msg_sig.get())}
        }
    }
}
