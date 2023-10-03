use std::cell::RefCell;
use sycamore::prelude::*;
use std::rc::Rc;
use crate::game::Game;
use crate::ui_player_setup::*;

#[derive(Prop)]
pub struct UiMainProps {
    pub game_ref:Rc<RefCell<Game>>
}

#[component]
pub fn UiSide< G: Html>(cx: Scope, props:UiMainProps) -> View<G> {
    let setup_done_sig:&Signal<bool> = create_signal(cx, false);
    let arg_ref = create_signal(cx,  props.game_ref.clone());
    view!{cx, div {
        (if  *setup_done_sig.get(){
            view!{ cx,
                div { "setup finished" }
            }
        }else{
            view!{ cx,
                PlayersSetup(game_ref=arg_ref, done=setup_done_sig)
            }
        })
    }}
}

