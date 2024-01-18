use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use marble::impl_visibility;
use marble::wrap::{*};
use marble::traits::{*};
use crate::game::Game;
use gloo::console::log as console_log;


pub struct ViewTurn {
    head:WrpDiv,
    player_label:WrpDiv,
    btn_next:WrpBtn,
    game_ref:Rc<RefCell<Game>>,
    pub can_reinforce:bool,
    pub player_id:u32,
}

impl View for ViewTurn {
    fn update(&mut self) {
        if self.can_reinforce{
            self.btn_next.inline_txt("Reinforce")
        }else {
            self.btn_next.inline_txt("End turn")
        }
        self.player_label.inline_txt(format!("Player: {}", self.player_id +1).as_str())
    }
}

impl ViewTurn {
    pub fn reset(&mut self, player_id:u32){
        self.can_reinforce = true;
        self.player_id = player_id;
        self.update();
    }
}

impl_visibility!(ViewTurn);

pub fn create_view_turn(glob: Rc<RefCell<Game>>, mount_id:&str) -> Rc<RefCell<ViewTurn>>{
    console_log!("creating turn view");

    let mut player_label= Div();
    let mut btn_next =  Button();

    let head = Div().child(
        player_label.get_clone().txt("Player: ")
    ).child(
        btn_next.get_clone().txt("End turn")
    ).style("margin-bottom: 15px;")
        .mount(mount_id);

    let mut state = ViewTurn {
        head,
        player_label,
        btn_next,
        game_ref: glob,
        can_reinforce: true,
        player_id: 0,
    };
    state.update();
    let state_ref = Rc::new(RefCell::new(state));

    state_ref.borrow().btn_next.set_state_handler(
        state_ref.clone(), |mut s:RefMut<ViewTurn> |{
            s.can_reinforce = !s.can_reinforce;
            s.update();
        },
        "end turn"
    );
    return state_ref
}