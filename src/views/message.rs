use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use marble::impl_visibility;
use marble::wrap::{*};
use marble::traits::{*};
use crate::game::Game;
use gloo::console::log as console_log;


pub struct ViewMessage {
    head:WrpDiv,
    message_div:WrpDiv,
    btn_next:WrpBtn,
    game_ref:Rc<RefCell<Game>>,
    pub message:String,
}

impl View for ViewMessage {
    fn update(&mut self) {
      self.message_div.inline_txt(&*self.message.clone())
    }
}

impl ViewMessage {
    pub fn reset(&mut self, msg:String){
        self.message = msg;
        self.update();
    }
}

impl_visibility!(ViewMessage);

pub fn create_view_mesage(glob: Rc<RefCell<Game>>, mount_id:&str) -> Rc<RefCell<ViewMessage>>{
    console_log!("creating turn view");

    let mut message_div = Div();
    let mut btn_next =  Button();
    let default_msg = "Empty message";

    let head = Div().child(
        message_div.get_clone().txt(default_msg)
    ).child(
        btn_next.get_clone().txt("Next")
    ).style("margin-bottom: 15px;")
        .mount(mount_id);

    let mut state = ViewMessage {
        head,
        message_div,
        btn_next,
        game_ref: glob,
        message: default_msg.to_string(),
    };
    state.update();
    let state_ref = Rc::new(RefCell::new(state));

    state_ref.borrow().btn_next.set_state_handler(
        state_ref.clone(), | s:RefMut<ViewMessage> |{
            s.game_ref.borrow_mut().handle_message_next();
        },
        "message next"
    );
    return state_ref
}