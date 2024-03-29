use std::cell::RefCell;
use sycamore::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement};
use crate::game::Game;
use crate::utils::consts::{MAX_PLAYERS, PLAYER_COLORS};


#[derive(Debug, Clone, Copy)]
pub struct PlayerConfig{
    pub player_count:i32,
    pub player_colors:Signal<Vec<String>>,
    pub player_is_ai:[bool; MAX_PLAYERS],
}

impl PlayerConfig {
    fn new()->PlayerConfig{
        PlayerConfig{
            player_count: 2,
            player_colors: create_signal(vec!["empty".to_string(); MAX_PLAYERS]),
            player_is_ai: [false; MAX_PLAYERS],
        }
    }
}


#[derive(Props, Clone, Copy)]
pub struct PlayerConfigProps {
    pub idx:i32,
    data:Signal<PlayerConfig>,
    done:Signal<bool>,
}


#[component]
pub fn PlayerSetup<G: Html>( props: PlayerConfigProps) -> View<G> {
    let checkbox_handle = move |e:Event| {
        let input_elem = e.target().unwrap()
            .dyn_ref::<HtmlInputElement>().unwrap().clone();
        let is_checked = input_elem.checked();
        let idx = props.idx as usize;
        let mut tmp_player_config = (props.data.get()).clone();
        tmp_player_config.player_is_ai[idx - 1] = is_checked;
        props.data.set(tmp_player_config);
    };

    view! {
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
pub fn Color_Setup< G: Html>( props: PlayerConfigProps) -> View<G> {
    let onchange_handle = move |e:Event| {
        let select_elem = e.target().unwrap()
            .dyn_ref::<HtmlSelectElement>().unwrap().clone();
        let selected_option = select_elem.value();
        let idx = props.idx as usize;
        let tmp_player_config = (props.data.get()).clone();

        let mut tmp_colors = tmp_player_config.player_colors.get_clone();
        tmp_colors[idx - 1] = selected_option;
        tmp_player_config.player_colors.set(tmp_colors);
        props.data.set(tmp_player_config);
    };

    let options:View<G> = View::new_fragment(
        PLAYER_COLORS.iter().map(|color| view! {
            option(value=color ) { (color) } }
        ).collect()
    );

    view! {
        select( class="form-select", style="width: fit-content",
            on:change=move |e| onchange_handle(e)) {
            option(value="empty"){"choose a color"}
            (options)
        }
    }
}


#[derive(Props,  Clone)]
pub struct PlayersSetupProps {
    pub game_ref: Rc<RefCell<Game>>,
    pub ui_state:Signal<bool>,
}


#[component]
pub fn PlayersSetup< G : Html>( props:PlayersSetupProps) -> View<G> {
    let max_players = 6i32;
    let min_players = 2i32;
    let next_sig = create_signal(true);

    let done_sig:Signal<bool> = create_signal( false);

    let player_config_sig:Signal<PlayerConfig> = create_signal(PlayerConfig::new());

    let error_msg_sig:Signal<String> = create_signal( "".to_string());

    let props2 = props.clone();

    let next = move || {
        let tmp_player_config = (player_config_sig.get()).clone();

        if next_sig.get() == true{
            player_config_sig.set(tmp_player_config);
            next_sig.set(!next_sig.get());

        }else {
            let mut validated = true;
            let mut _return  = false;

            let player_colors = &tmp_player_config.player_colors.get_clone();
            let num_players = tmp_player_config.player_count as usize;

            for i  in 0..num_players {
                if player_colors[i]  == "empty"{
                    validated = false;
                    break;
                }
            }

            if !validated{
                if !_return{
                    error_msg_sig.set("Not all players have picked a color".to_string());
                    _return = true;
                }
            }else{
                let mut found:Vec<String> = Vec::new();
                for i in 0..num_players{
                    if !found.contains(&player_colors[i]){
                        found.push(player_colors[i].clone());
                    }else{
                        validated = false;
                        break;
                    }
                }
            }
            if !validated{
                if !_return{
                    error_msg_sig.set("All players must have different colors".to_string());
                    _return = true;
                }
            }else{
                props.game_ref.borrow_mut().set_player_config(tmp_player_config);
                props.ui_state.set(false);
            }
        }

    };
    let test = move ||{
        let config = PlayerConfig::new();
        let mut colors = config.player_colors.get_clone();
        colors[0] = PLAYER_COLORS[0].to_string();
        colors[1] = PLAYER_COLORS[1].to_string();
        config.player_colors.set(colors);
        props2.game_ref.borrow_mut().set_player_config(config);
        props2.ui_state.set(false);
    };


    let player_options:Signal<View<G>> = create_signal(View::new_fragment(
        (min_players..max_players + 1).map(|x| view! {
            option(value=x, on:click= move |_| {
                let mut tmp_player_config = (player_config_sig.get()).clone();
                tmp_player_config.player_count = x;
                player_config_sig.set(tmp_player_config);
            }) { (x) }
        }
        ).collect()
    ));


    let player_config_iter_sig = create_memo( move||{
        let ret:Vec<i32> = (1..(player_config_sig.get()).player_count + 1).collect();
        ret
    });

    view! {
    div {
        (if next_sig.get() {
            view! {
                select( class="form-select", style="width: fit-content") {
                    option(){"Choose the number of players"}
                    (player_options.get_clone())
            }}
        } else {
            view! {
                Keyed( iterable=player_config_iter_sig, key=|x| *x, view=move |x| view! {
                    div{
                        PlayerSetup(idx=x, data=player_config_sig, done=done_sig)
                    }
            },)}
        })
            button(id="run", type="button", class="btn btn-primary", on:click= move |_| next()){
                "Next"
            }
            label(){(error_msg_sig.get_clone())}
            button(id="test", type="button", class="btn btn-primary", on:click= move |_| test()){
                "test"
            }
        }
    }
}
