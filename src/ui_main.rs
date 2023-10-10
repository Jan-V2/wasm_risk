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
    CARD_SELECT,
    GAME_END
}

#[derive(Prop)]
pub struct UiMainProps {
    pub game_ref:Rc<RefCell<Game>>
}

#[component]
pub fn UiSide< G: Html>(cx: Scope, props:UiMainProps) -> View<G> {
    let setup_done_sig:&Signal<UiState> = create_signal(cx, UiState::SETUP);
    let arg_ref = create_signal(cx,  props.game_ref.clone());
    let player_width_turn:&Signal<u32> = create_signal(cx, 0);
    view!{cx, div {
        (if  *setup_done_sig.get() == UiState::SETUP {
            view!{ cx,
                PlayersSetup(game_ref=arg_ref, ui_state=setup_done_sig)
            }

        }else if *setup_done_sig.get() == UiState::TURN_START || *setup_done_sig.get() == UiState::TURN  {
            view!{ cx,
                Turn_Ui(game_ref=arg_ref, player_with_turn=player_width_turn)
            }
        }else{
            view!{ cx,
                div { "setup finished" }
            }
        })
    }}
}




#[component(inline_props)]
pub fn ArmyPlacementStart<'a, G: Html>(cx: Scope<'a>,
                    game_ref:& 'a Signal<Rc<RefCell<Game>>>,
                    ui_state:& 'a Signal<UiState>,
) -> View<G> {

    let is_done = create_signal(cx, true);
    let current_player = create_signal(cx, 0u32);
    let num_armies = create_signal(cx, 30u32);
    // todo make it so the armies are placed for one player after another
    let _ = create_memo(cx, move ||{
        if *is_done.get(){
            let max_players = (*game_ref.get()).borrow_mut().model.players.len() as u32;
            if *current_player.get() + 1 < max_players{

            }else {

            }
        }
    });

    view!{cx,
        div {"turn ui " (current_player.get()) }

                ArmyPlacementUi(is_done=is_done, game_ref =game_ref,
                    player_with_turn=current_player, num_armies=num_armies
                )


    }
}



#[derive(Prop)]
pub struct TurnUiProps<'a> {
    game_ref:& 'a Signal<Rc<RefCell<Game>>>,
    player_with_turn:&'a Signal<u32>,
}


#[component]
pub fn Turn_Ui<'a, G: Html>(cx: Scope<'a>, props:TurnUiProps<'a>) -> View<G> {
    let show = create_signal( cx,true);
    let num_armies_sig = create_signal(cx, 10u32);
    let player_with_turn = create_signal(cx, 0u32);
    view!{cx,
        div {"turn ui " (props.player_with_turn.get()) }
        (if  *show.get(){
            view!{cx,
                ArmyPlacementUi(is_done=show, game_ref = props.game_ref,
                    player_with_turn=player_with_turn, num_armies=num_armies_sig
                )
                }
            }else{
                view!{cx,
                    div{}
                }
            }
        )
    }
}



#[component(inline_props)]
pub fn ArmyPlacementUi<'a, G: Html>(cx: Scope<'a>,
                                       game_ref:& 'a Signal<Rc<RefCell<Game>>>,
                                       player_with_turn:& 'a Signal<u32>,
                                       num_armies:& 'a Signal<u32>,
                                       is_done: &'a Signal<bool>) -> View<G>
{
    let num_armies_sig = create_rc_signal( *num_armies.get());
    let armies_sig2 = num_armies_sig.clone();
    let show = create_rc_signal(true);
    let show_clone = show.clone();
    let _ = create_memo(cx,  move|| {is_done.set(*show.get())} );
    (*game_ref.get()).borrow_mut().set_army_placement_sig(num_armies_sig.clone(), move  || {show_clone.set(false)} );

    view!{cx,
        div{
            h1{"Player " (*player_with_turn.get())}
            div{"You still have " (*armies_sig2.get())  " armies to place"}
        }
    }
}
