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
    let info_placement_rc = ui_info_struct.placement.clone();
    let info_placement_start_rc = ui_info_struct.start_placement.clone();
    let ui_state_rc = ui_info_struct.ui_state.clone();


    let ui_state: Signal<UiState> = create_signal( UiState::SETUP);
    let arg_ref = props.game_ref.clone();


    let army_placement_sig = create_signal(ui_info_struct.placement.get().clone());
    let army_placement_start_sig = create_signal(ui_info_struct.start_placement.get().clone());


    let _ = create_memo(move || {
        if info_placement_rc.get() != army_placement_sig.get() {
            if army_placement_sig.get().updated {
                let mut tmp = army_placement_sig.get();
                tmp.updated = false;
                info_placement_rc.set(tmp.clone());
                army_placement_sig.set(tmp);
            } else {
                let mut tmp = info_placement_rc.get();
                tmp.updated = false;
                army_placement_sig.set(tmp.clone());
                info_placement_rc.set(tmp);
            }
        }
    });

    let _ = create_memo(move || {
        log!("checking eq");
        if info_placement_start_rc.get() != army_placement_start_sig.get() {
            let mut tmp = if army_placement_start_sig.get().updated {
                log!("updating placement start from sig");
                army_placement_start_sig.get()
            } else {
                log!("updating placement start from rc");
                info_placement_start_rc.get()
            };
            tmp.updated = false;
            info_placement_start_rc.set(tmp.clone());
            army_placement_start_sig.set(tmp);
        } else {
            log!("no diff")
        }
    });


    let _ = create_memo( move || {
        if ui_state_rc.get() != ui_state.get() {
            ui_state_rc.set(ui_state.get());
        }
    });


    view! { div {
        (if  ui_state.get() == UiState::SETUP {
            view!{
                PlayersSetup(game_ref=arg_ref.get_clone(), ui_state=ui_state)
            }
        }else if ui_state.get() == UiState::ARMY_PLACEMENT_START{
            view!{
                ArmyPlacementStart(ui_state=ui_state, ui_info=army_placement_start_sig)
            }
        }else if ui_state.get() == UiState::ARMY_PLACEMENT  {
            view!{
                Turn_Ui(army_num=9u32, player_id=1u32, ui_state=ui_state, ui_info=army_placement_sig)
            }
        }else if ui_state.get() == UiState::GAME_END{
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
    gloo::console::log!("running place start");
    let is_broken = false;

    if is_broken {
        let _ = create_memo(move || {
            let mut _ui_info = ui_info.get();
            if _ui_info.armies_per_player[_ui_info.current_player as usize] == 0 {
                if _ui_info.current_player + 1 < _ui_info.num_players {
                    ui_info.set(ui_info.get().update(|s| {
                        s.current_player = s.current_player + 1;
                        s.is_done = false;
                        log!("updating current player");
                    }));
                } else {
                    ui_state.set(UiState::ARMY_PLACEMENT)
                }
            }
        });
    } else {
        let _ = create_memo( move || {
            if ui_info.get().is_done {
                if ui_info.get().current_player + 1 < ui_info.get().num_players {
                    let mut tmp = ui_info.get();
                    tmp.updated = true;
                    tmp.current_player = tmp.current_player + 1;
                    tmp.is_done = false;
                    ui_info.set(tmp);
                } else {
                    ui_state.set(UiState::ARMY_PLACEMENT)
                }
            }
        });
    }


    view! {
            h1{"Player " (ui_info.get().current_player + 1 )}
            div{"You still have " (ui_info.get().armies_per_player[ui_info.get().current_player as usize])  " armies to place"}
    }
}