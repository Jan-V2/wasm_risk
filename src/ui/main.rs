use std::cell::RefCell;
use sycamore::prelude::*;
use std::rc::Rc;
use gloo::console::log;
use crate::game::Game;
use crate::ui::player_setup::*;
use crate::ui::structs::{ArmyPlacementInfo, StartArmyPlacementInfo, UiInfo, UiUpdatable};


#[allow(non_camel_case_types)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum UiState {
    SETUP,
    ARMY_PLACEMENT_START,
    ARMY_PLACEMENT,
    TURN_START,
    TURN,
    COMBAT,
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
    let placement_sig = ui_info_struct.placement.clone();
    let placement_start_sig = ui_info_struct.start_placement.clone();
    let ui_state_sig = ui_info_struct.ui_state.clone();

    let arg_ref = props.game_ref.clone();


    view! { div {
        h1{}
        (if  ui_state_sig.get() == UiState::SETUP {
            view!{
                PlayersSetup(game_ref=arg_ref.get_clone(), ui_state=ui_state_sig)
            }
        }else if ui_state_sig.get() == UiState::ARMY_PLACEMENT_START{
            view!{
                ArmyPlacementStart(ui_state=ui_state_sig, ui_info=placement_start_sig)
            }
        }else if ui_state_sig.get() == UiState::ARMY_PLACEMENT  {
            view!{
                Turn_Ui(army_num=9u32, player_id=1u32, ui_state=ui_state_sig, ui_info=placement_sig)
            }
        }else if ui_state_sig.get() == UiState::GAME_END{
            view!{
                div { "game end" }
            }
        }else{
            view!{
                div { "default ui state" }
            }
        })
    }}
}


#[component(inline_props)]
pub fn Turn_Ui< G: Html>(army_num: u32,
                            player_id: u32,
                            ui_state: Signal<UiState>,
                            ui_info: Signal<ArmyPlacementInfo>,
) -> View<G> {
    gloo::console::log!("running setup");
    ui_info.set(ui_info.get().update(|tmp| {
        tmp.army_count = army_num;
        tmp.current_player = player_id
    }));


    let _ = create_memo( move || {
        let tmp = ui_info.get();
        if tmp.army_count == 0 {
            ui_state.set(UiState::GAME_END);
        }
    });

    view! {
        div{"turn test"}
            h1{"Player " (ui_info.get().current_player + 1 )}
            div{"You still have " (ui_info.get().army_count)  " armies to place"}
    }
}


#[component(inline_props)]
pub fn ArmyPlacementUi< G: Html>(ui_info: Signal<ArmyPlacementInfo>, ) -> View<G>
{
    view! {
            h1{"Player " (ui_info.get().current_player + 1 )}
            div{"You still have " (ui_info.get().army_count)  " armies to place"}
    }
}


#[component(inline_props)]
pub fn ArmyPlacementStart<G: Html>(
    ui_state: Signal<UiState>,
    ui_info: Signal<StartArmyPlacementInfo>,
) -> View<G> {
    let cause_crash = true;

    gloo::console::log!("running place start");
    create_effect(move || {
        let mut _ui_info = ui_info.get();
        if _ui_info.armies_per_player[_ui_info.current_player as usize] == 0 {
            if _ui_info.current_player + 1 < _ui_info.num_players {
                ui_info.set(ui_info.get().update(|s| {
                    s.current_player = s.current_player + 1;
                    s.is_done = false;
                    log!("updating current player");
                }));
            } else {
                log!("updating ui state");
                if cause_crash{
                    ui_state.set(UiState::GAME_END);  // <- this lines causes a panic
                }
            }
        }
    });

    view! {
            h1{"Player " (ui_info.get().current_player + 1 )}
            div{"You still have " (ui_info.get().armies_per_player[ui_info.get().current_player as usize])  " armies to place"}
    }
}
