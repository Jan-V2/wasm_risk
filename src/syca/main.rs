use std::cell::RefCell;
use sycamore::prelude::*;
use std::rc::Rc;
use crate::game::Game;
use crate::syca::player_setup::*;

#[derive(Props)]
pub struct UiMainProps {
    pub game_ref: Signal<Rc<RefCell<Game>>>,
}

#[component]
pub fn UiSide<G: Html>(props: UiMainProps) -> View<G> {
    let ui_state_sig = create_signal(true);
    let arg_ref = props.game_ref.clone();

    view! { div {
        (if  ui_state_sig.get() == true {
            view!{
                PlayersSetup(game_ref=arg_ref.get_clone(), ui_state=ui_state_sig)
            }
        }else{
            view!{
                div { }
            }
        })
    }}
}

