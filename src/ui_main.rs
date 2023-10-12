use std::cell::RefCell;
use sycamore::prelude::*;
use std::rc::Rc;
use crate::game::Game;
use crate::ui_player_setup::*;

#[allow(non_camel_case_types) ]
#[derive(PartialEq)]
pub enum UiState{
    SETUP,
    ARMY_PLACEMENT_START,
    ARMY_PLACEMENT,
    TURN_START,
    TURN,
    COMBAT,
    GAME_END,
    CARD_SELECT,

}

#[derive(Prop)]
pub struct UiMainProps {
    pub game_ref:Rc<RefCell<Game>>
}

pub struct ArmyPlacementInfo{
    army_count :RcSignal<u32>,
    is_done:RcSignal<bool>,
}
impl ArmyPlacementInfo{
    fn new()->ArmyPlacementInfo{
        ArmyPlacementInfo{
            army_count: Default::default(),
            is_done: create_rc_signal(true),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UiInfo{
    pub army_count:u32,
    pub is_done:bool,
    pub army_placement:bool,
    pub updated:bool,
    pub current_player:u32,
}

impl UiInfo{
    pub fn new() -> UiInfo{
        UiInfo{
            army_count: 0,
            is_done: false,
            army_placement: false,
            updated: false,
            current_player: 0,
        }
    }
}

#[component]
pub fn UiSide< G: Html>(cx: Scope, props:UiMainProps) -> View<G> {
    let setup_done_sig:&Signal<UiState> = create_signal(cx, UiState::SETUP);
    let arg_ref = create_signal(cx,  props.game_ref.clone());


    let ui_info_rc = props.game_ref.borrow().get_ui_info_clone();
    let ui_info_sig = create_signal(cx, *ui_info_rc.get().clone());

    let _ = create_memo(cx, move ||{
        if *ui_info_rc.get() != *ui_info_sig.get(){
            if ui_info_sig.get().updated {
                let mut tmp = *ui_info_sig.get();
                tmp.updated = false;
                ui_info_rc.set(tmp.clone());
                ui_info_sig.set(tmp);
            }else{
                let mut tmp = *ui_info_rc.get();
                tmp.updated = false;
                ui_info_sig.set(tmp.clone());
                ui_info_rc.set(tmp);
            }
        }
    });

    view!{cx, div {
        (if  *setup_done_sig.get() == UiState::SETUP {
            view!{ cx,
                PlayersSetup(game_ref=arg_ref, ui_state=setup_done_sig)
            }
        }else if *setup_done_sig.get() == UiState::ARMY_PLACEMENT_START{
            view!{cx,
                ArmyPlacementStart(ui_state=setup_done_sig, ui_info=ui_info_sig)
            }
        }else if *setup_done_sig.get() == UiState::TURN_START  {
            view!{ cx,
                Turn_Ui(army_num=9u32, player_id=1u32, ui_state=setup_done_sig, ui_info=ui_info_sig) // this comp also sets it
            }
        }else if *setup_done_sig.get() == UiState::GAME_END{
            view!{ cx,
                div { "game end" }
            }
        }else{
            view!{ cx,
                div { "default ui state" }
            }
        })
    }}
}


#[derive(Prop)]
pub struct TurnUiProps<'a> {
    game_ref:& 'a Signal<Rc<RefCell<Game>>>,
    player_with_turn:&'a Signal<u32>,
}


#[component(inline_props)]
pub fn Turn_Ui<'a, G: Html>(cx: Scope<'a>,
                            army_num:u32,
                            player_id:u32,
                            ui_state:& 'a Signal<UiState>,
                            ui_info: & 'a Signal<UiInfo>,
) -> View<G> {

    let mut tmp = *ui_info.get();
    let mut run_setup = false;
    if !run_setup{
        gloo::console::log!("running setup");
        tmp.updated = true;
        tmp.army_placement = true;
        tmp.army_count = army_num;
        tmp.current_player = player_id;
        ui_info.set(tmp);
        run_setup = true;
    }else{
        gloo::console::log!("running else");
        if tmp.army_count == 0{
            ui_state.set(UiState::GAME_END);
        }
    }


    let _ = create_memo(cx, move ||{
        let tmp = *ui_info.get();
        if tmp.army_count == 0{
            ui_state.set(UiState::GAME_END);
        }
    });

    view!{cx,
        div{"turn test"}
            h1{"Player " (ui_info.get().current_player + 1 )}
            div{"You still have " (ui_info.get().army_count)  " armies to place"}
    }
}


#[component(inline_props)]
pub fn ArmyPlacementUi<'a, G: Html>(cx: Scope<'a>,
                                    ui_info: & 'a Signal<UiInfo>,) -> View<G>
{

    view!{cx,
            h1{"Player " (ui_info.get().current_player + 1 )}
            div{"You still have " (ui_info.get().army_count)  " armies to place"}
    }
}


#[component(inline_props)]
pub fn ArmyPlacementStart<'a, G: Html>(cx: Scope<'a>,
                    ui_state:& 'a Signal<UiState>,
                    ui_info: & 'a Signal<UiInfo>,
) -> View<G> {

    gloo::console::log!("running place start");
    let mut tmp = *ui_info.get();
    tmp.updated = true;
    tmp.army_placement = true;
    tmp.is_done = false;
    tmp.army_count = 2;
    ui_info.set(tmp);



    let _ = create_memo(cx, move ||{

        if ui_info.get().army_count == 0{
            let max_players =  2;
            if ui_info.get().current_player + 1 < max_players{
                let mut tmp = *ui_info.get();
                tmp.updated = true;
                tmp.current_player = tmp.current_player +1;
                tmp.is_done = false;
                tmp.army_count = 2;
                ui_info.set(tmp);
            }else {
                let mut tmp = *ui_info.get();
                tmp.updated = true;
                tmp.is_done = false;
                ui_info.set(tmp);
                ui_state.set(UiState::TURN_START)
            }
        }
    });

    view!{cx,
            h1{"Player " (ui_info.get().current_player + 1 )}
            div{"You still have " (ui_info.get().army_count)  " armies to place"}
    }
}





