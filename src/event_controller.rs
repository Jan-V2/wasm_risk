use gloo::console::log as console_log;
use crate::model::Coord;
use crate::views::main::ViewsEnum;
use crate::utils::consts::DEBUG_MENU_STACK_POP;


/* todos:
 * move all code related to tracking the active state to this file
 * create a way of testing this component
 * make it so all senarios are covered
 * - setup
 * - inserting menues on top of menus
 * - the end of turns
 * */

#[macro_export]
macro_rules! bind {
    ($gettter:stmt, $var_name:ident) => {
        paste::paste! {
            let [<bind_ $var_name>] = $gettter;
            let $var_name = [<bind_ $var_name>].borrow();
        }
    };
}

#[macro_export]
macro_rules! bind_mut {
    ($gettter:stmt, $var_name:ident) => {
        paste::paste! {
            let [<bind_ $var_name>] = $gettter;
            let mut $var_name = [<bind_ $var_name>].borrow_mut();
        }
    };
}


/* todos:
 * refactor events into using enums with values?
 * refactor event reciever and stack controller into sperate objects
 *      the event flow needs to be one way for borrowing reasons
 * route all events to this object
 * route events from this obj to game
 * create tests
 * */

struct EventController{

    
    menu_stack: MenuStack,
}

impl EventController{
    
    pub fn new()->EventController{
        EventController{
            menu_stack: MenuStack::new(true),
        }
            
    }
    pub fn handle_canvas_click(&mut self, clicked_coord: Coord) {
        if self.state_turn.in_setup {
            return;
        }
        let prov_id_opt = self.lookup_coord(&clicked_coord);
        if prov_id_opt.is_some() {
            let prov_id = prov_id_opt.unwrap();
            match self.menu_stack.get() {
                ViewsEnum::Turn => { self.handle_canvas_turn(prov_id) }
                ViewsEnum::ArmyPlacement => { self.handle_canvas_army_placement(prov_id) }
                ViewsEnum::Combat => { self.handle_canvas_noop(ViewsEnum::Combat) }
                ViewsEnum::DiceRolls => { self.handle_canvas_noop(ViewsEnum::DiceRolls) }
                ViewsEnum::Message => {self.handle_canvas_noop(ViewsEnum::Message)}
                ViewsEnum::Next_Turn => {panic!("next turn menu can't be activated?")  }
            }
        }
    }

    pub fn handle_canvas_noop(&mut self, state: ViewsEnum) {
        self.log(format!("in state: {:?} the canvas is not handled", state))
    }

    pub fn push_message_view(&mut self, msg:String){
        bind_mut!(self.get_message(), msg_menu);
        msg_menu.message = msg;
        self.menu_stack.push(ViewsEnum::Message)
    }

    
    pub fn push_army_placement(&mut self, armies: u32, player_id:u32) {
        bind_mut!(self.get_army_placement(), menu);
        menu.armies = armies;
        menu.player_id = player_id;
        self.menu_stack.push(ViewsEnum::ArmyPlacement);
    }


}


pub trait EventReciever{
    fn handle_canvas_turn(&mut self, prov_id: u32);
    fn handle_canvas_army_placement(&mut self, prov_id: u32);
    fn handle_canvas_move(&mut self, _prov_id: u32);
    fn handle_end_turn(&mut self, can_reinforce:bool);
    fn handle_ui_retreat(&mut self); 
    fn handle_ui_dice_next(&mut self);
    fn handle_ui_combat_roll(&mut self, is_attack: bool); 
}

pub struct  MenuStack{
    menu_stack: Vec<ViewsEnum>,
    current: ViewsEnum,
    initialized: bool,
    debug: bool,
}

impl MenuStack{
    // todo clean up the api
    // calling pop on the ActiveMenu struct removes a menu
    // but calling self.pop_menu() inside game also loads the menu
    // ideally, you'd only have to call one place instead of multible
    pub fn new(debug: bool) ->  MenuStack{
         MenuStack{
            menu_stack: vec![],
            current: Default::default(),
            initialized: false,
            debug,
        }
    }

    fn print_stack(&self) {
        console_log!("printing stack len=", self.menu_stack.len());
        console_log!(format!("current = {:?}", self.current));
        for item in &self.menu_stack {
            console_log!("printing stack");
            console_log!(format!("{:?}\n", item));
        }
    }

    pub fn get(&self) -> ViewsEnum {
        self.current.clone()
    }

    pub fn get_next(&mut self) -> Option<ViewsEnum> {
        if self.menu_stack.len() == 0 {
            return None;
        }
        return Some(self.menu_stack[0].clone());
    }

    pub fn get_num_queued(&self) -> u32 {
        return self.menu_stack.len() as u32;
    }

    pub fn pop(&mut self) -> ViewsEnum {
        // this assumption is that the data is already loaded into the menu,
        // and only needs to be displayed
        // this means the the previous state of the menu is restored
        if DEBUG_MENU_STACK_POP {
            console_log!(format!("Debug: menu stack pre pop {:?}", self.menu_stack));
            console_log!(format!("Debug: current pre pop {:?}", self.current));
        }
        if self.menu_stack.is_empty() {
            panic!(
                "can't pop menu stack, stack empty. current menu {:?}",
                self.current
            )
        }
        self.current = self.menu_stack.pop().unwrap();
        if DEBUG_MENU_STACK_POP {
            console_log!(format!("Debug: menu stack post pop {:?}", self.menu_stack));
            console_log!(format!("Debug: current post pop {:?}", self.current));
        }
        self.get()
    }

    pub fn push(&mut self, menu: ViewsEnum) {
        if self.initialized {
            self.menu_stack.push(self.current.clone());
        } else {
            // overwrite current on first push, so current does not need to be an option
            self.initialized = true;
        }
        self.current = menu;
        if self.debug {
            console_log!(format!("pushed {:?} to stack", self.current));
            self.print_stack();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.menu_stack.is_empty()
    }

    pub fn set_current(&mut self, view: ViewsEnum) {
        if self.debug {
            console_log!(format!("setting current {:?}", view));
            self.print_stack();
        }
        self.current = view;
    }
}
