use std::cell::RefCell;
use sycamore::prelude::*;
use std::rc::Rc;
use crate::game::Game;
use crate::ui::player_setup::*;
use crate::ui::structs::{ UiInfo};


#[allow(non_camel_case_types)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum UiState {
    SETUP,
    ARMY_PLACEMENT_START,
    ARMY_PLACEMENT,
    TURN,
    MOVE,
    COMBAT,
    DICE_ROLL,
    GAME_END,
    CARD_SELECT,

}

#[derive(Props)]
pub struct UiMainProps {
    pub game_ref: Signal<Rc<RefCell<Game>>>,
}

#[component]
pub fn UiSide<G: Html>(props: UiMainProps) -> View<G> {
    let ui_info_struct = UiInfo::new();
    props.game_ref.get_clone().borrow_mut().set_ui_info(ui_info_struct.clone());
    let ui_state_sig = ui_info_struct.ui_state.clone();

    let arg_ref = props.game_ref.clone();


    view! { div {
        h1{}
        (if  ui_state_sig.get() == UiState::SETUP {
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

