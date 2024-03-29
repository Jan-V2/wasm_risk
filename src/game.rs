use crate::model::{CombatState, Coord, Model, Player, Rules};
use crate::syca::player_setup::PlayerConfig;
use crate::utils::consts::DEBUG_MENU_STACK_POP;
use crate::utils::funcs::rand_int;
use crate::utils::structs::AttackDefendPair;
use crate::views::army_placement::ViewArmyPlacement;
use crate::views::combat::ViewCombat;
use crate::views::dice_roll::ViewDiceRoll;
use crate::views::info::{create_view_info, ViewInfo};
use crate::views::main::{create_view_main, ViewMain, ViewsEnum, ViewsStruct};
use crate::views::message::ViewMessage;
use crate::views::turn::ViewTurn;
use gloo::console::log as console_log;
use gloo_timers::callback::Timeout;
use js_sys::Math::sqrt;
use marble::traits::View;
use stack_stack::{stack, Stack};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use crate::{bind, bind_mut};
use crate::event_controller::EventReciever;


/*
todo start placement:
run placement with special flag.
depending on flag handeler changes
handeler handels single placement or start
todo should .update omit &mut or should .show require &mut

todo display player n's turn msg at the start of each turn
*/

pub struct GameTurnState {
    pub(super) targets: AttackDefendPair<Option<u32>>,
    pub(super) active_player: u32,
    pub(super) in_initial_placement_phase: bool, //
    pub(super) in_setup: bool,
}

impl GameTurnState {
    pub fn new() -> GameTurnState {
        GameTurnState {
            targets: Default::default(),
            active_player: 0,
            in_initial_placement_phase: true,
            in_setup: true,
        }
    }
}

macro_rules! create_getter {
    ($view_name:ident, $ty:ident) => {
        paste::paste! {
        impl Game{
            pub fn [<get_ $view_name>](&self)->Rc<RefCell<$ty>>{
                self.views.as_ref().unwrap().$view_name.clone()
            }
        }
        }
    };
}

create_getter!(turn, ViewTurn);
create_getter!(army_placement, ViewArmyPlacement);
create_getter!(combat, ViewCombat);
create_getter!(dice_rolls, ViewDiceRoll);
create_getter!(message, ViewMessage);



pub struct Game {
    pub model: Model,
    pub prov_lookup: ProvLookupTable,
    pub flag_scale: f64,
    pub logging: bool,
    pub state_turn: GameTurnState,
    pub view_main: Option<Rc<RefCell<ViewMain>>>,
    pub info_view: Rc<RefCell<ViewInfo>>,
    pub combat_state: CombatState,
    pub self_ref: Option<Rc<RefCell<Game>>>,
    pub event_reciever:Rc<RefCell<EventReciever>>,
}



impl Game {
    pub fn new(prov_lookup: ProvLookupTable, use_logging: bool) -> Game {
        return Game {
            model: Model::new_from_json(),
            prov_lookup,
            flag_scale: 0.5,
            logging: use_logging,
            state_turn: GameTurnState::new(),
            view_main: None,
            info_view: create_view_info("text_out", "setup".to_string()),
            combat_state: CombatState::default(),
            self_ref: None,
        };
    }

    // pops menu after timer for borrowing reasons
    // very janky indeed
    pub fn pop_menu_async(&self, time_ms: u32) {
        let game_ref = self.self_ref.as_ref().unwrap().clone();
        let t = Timeout::new(time_ms, move || {
            game_ref.borrow_mut().pop_menu();
            console_log!("popped menu async")
        });
        t.forget();
    }

    // pops a menu from the stack
    pub fn pop_menu(&mut self) {
        if self.menu_stack.is_empty() {
            // this means that the current players turn has ended
            //todo check if a player is knocked out and skip their turn
            self.log("menu stack empty moving on to next player".to_string());
            if self.state_turn.in_initial_placement_phase {
                self.army_placement_start_next(false);
            } else {
                self.setup_next_turn();
            }
            self.activate_current_menu();
        } else {
            let _ = self.menu_stack.pop();
            self.activate_current_menu();
        }
    }

    /*
       pub(super) fn apply_combat_result_to_map(&mut self, state_combat: &StateCombat) {
       self.log("combat finished handler".to_string()); let mut prov_attack =
       (*self.model.get_prov_from_id( &state_combat.prov_id_attacker).unwrap()).clone(); let mut
       prov_defend  = (*self.model.get_prov_from_id(
       &state_combat.prov_id_defender).unwrap()).clone();

       if state_combat.armies_defending == 0 && state_combat.armies_attacking > 0{
    // attacker won self.log("attack succeeded".to_string()); prov_attack.army_count = 1;
    // prov_defend.army_count = state_combat.armies_attacking; prov_defend.owner_id =
    // prov_attack.owner_id; }else { attack ongoing or failed self.log("attack
    // failed".to_string()); prov_attack.army_count = 1 + state_combat.armies_attacking;
    // prov_defend.army_count = state_combat.armies_defending; } self.log(format!("{:?}",
    // prov_attack)); self.model.set_prov(prov_attack); self.model.set_prov(prov_defend);
    // self.draw_board(); }
    */
    
    // adds or removes armies from a province, checking for 0
    pub(super) fn change_armies_in_prov(&mut self, num_armies: i32, prov_id: &u32) {
        let prov = self
            .model
            .get_prov_from_id_mut(prov_id)
            .expect(format!("prov with id {} could not be found", prov_id).as_str());
        let old_army_count = prov.army_count;

        let new_army_count = old_army_count as i32 + num_armies;
        if new_army_count > -1 {
            prov.army_count = new_army_count as u32;
        } else {
            prov.army_count = 0;
        }
    }

    // sets the army count in a province
    pub(super) fn set_armies_in_prov(&mut self, num_armies: u32, prov_id: &u32) {
        let prov = self
            .model
            .get_prov_from_id_mut(prov_id)
            .expect(format!("prov with id {} could not be found", prov_id).as_str());
        prov.army_count = num_armies;
    }

    // used in setup to assign armies randomly for testing
    pub(super) fn assign_provs_random(&mut self) {
        gloo::console::log!(format!("players len = {}", self.model.get_player_count()));
        let player_count = self.model.get_player_count() as usize;
        let prov_total = self.model.provinces.len() as u32;

        let mut provs_available: Vec<u32> = vec![];
        let provs_per_player = prov_total / player_count as u32;
        let mut remainder = prov_total % player_count as u32;

        for player_id in 0..player_count {
            provs_available.push(provs_per_player);
            if remainder > 0 {
                provs_available[player_id] += 1;
                remainder -= 1;
            }
        }

        let mut provs_vec: Vec<_> = (0..prov_total).collect();

        for p in 0..player_count {
            for _ in 0..provs_available[p] {
                let idx = rand_int(0, (provs_vec.len() - 1) as u32) as usize;
                self.model.provinces[provs_vec[idx] as usize].owner_id = p as u32;
                self.model.provinces[provs_vec[idx] as usize].army_count = 1;
                provs_vec.remove(idx);
            }
        }
    }

    // prints to console if self.logging is set
    pub(super) fn log(&self, msg: String) {
        if self.logging {
            console_log!(msg)
        }
    }

    pub fn show_message(&mut self, _label: String) {
        // todo add view that just shows a message
        /*        self.ui_man.view_label.update(StateLabel{
        label_text: label, return_state:  next_state.clone()});
        self.set_ui_state(UiState::LABEL);*/
    }
    /*
       pub(super) fn set_ui_state(&mut self, state: UiState) { self.state_turn.attack_target =
       None; self.ui_man.select_view(&state); self.config_sig.unwrap().ui_state.set(state.clone());
       match state { UiState::SETUP => {self.info_display_div.set_default( "".to_string())}
       UiState::ARMY_PLACEMENT | UiState::ARMY_PLACEMENT_START => {
       self.info_display_div.set_default( "Click on your own provinces to place your
       armies".to_string())} UiState::TURN => { self.info_display_div.set_default( "Click on your
       own province to attack from there, or press end turn".to_string())} UiState::MOVE =>
       {todo!()} UiState::COMBAT => {self.info_display_div.set_default( "Select the number of
       armies you want to use and click attack/defend".to_string())} UiState::DICE_ROLL =>
       {self.info_display_div.set_default( "".to_string())} UiState::GAME_END =>
       {self.info_display_div.set_default( "Game Over".to_string())} UiState::CARD_SELECT =>
       {todo!()} UiState::LABEL => { self.info_display_div.set_default("".to_string()) } } }
    */


    // returns provid of the clicked xy coord if found
    pub fn lookup_coord(&self, clicked_coord: &Coord) -> Option<u32> {
        let mut found_at_idx: Vec<i32> = Vec::new();

        if !self.prov_lookup.target_is_valid(clicked_coord) {
            return None;
        }

        for i in 0..self.model.provinces.len() {
            if self
                .prov_lookup
                .compare_colors(&self.model.provinces[i].location, &clicked_coord)
            {
                found_at_idx.push(i as i32);
            }
        }

        if found_at_idx.len() == 0 {
            None
        } else if found_at_idx.len() == 1 {
            Some(found_at_idx[0] as u32)
        } else {
            let mut idx_shortest: i32 = -1;
            let mut shortest_dist = i32::MAX;
            for idx in found_at_idx {
                let dist = ProvLookupTable::dist_between_pnts(
                    &clicked_coord,
                    &self.model.provinces[idx as usize].location,
                );
                if dist < shortest_dist {
                    shortest_dist = dist;
                    idx_shortest = idx;
                }
            }
            Some(idx_shortest as u32)
        }
    }

    // returns true if the active player owns the provided provid
    pub(super) fn is_owned_by_active(&self, prov_id: &u32) -> bool {
        let owner = self.model.get_owner_from_prov_id(prov_id);
        if owner.is_some() {
            return owner.unwrap() == self.state_turn.active_player;
        }
        false
    }

    // recieves the config info from the sycamore ui
    pub fn set_player_config(&mut self, config: PlayerConfig) {
        gloo::console::log!(format!("player count = {}", config.player_count));
        gloo::console::log!(format!("{:?}", config));
        self.state_turn.in_setup = false;
        for i in 0..config.player_count {
            self.model.players.push(Player {
                id: i as u32,
                cards: vec![],
                color: config.player_colors.get_clone()[i as usize]
                    .as_str()
                    .to_string(),
                is_computer: config.player_is_ai[i as usize],
            })
        }

        self.assign_provs_random();
        self.army_placement_start_next(true);
        self.draw_board();
    }

    // generates the string that is shown when the mouse hovers on the canvas
    // currently only used the show the prov name
    pub fn get_prov_mouseover_string(&self, coord: &Coord) -> Option<String> {
        let prov_id = self.lookup_coord(coord);
        return if prov_id.is_some() {
            self.model.get_name_from_prov_id(&prov_id.unwrap())
        } else {
            None
        };
    }

    //creates the views at setup and mounts them to the dom
    pub fn create_views(&mut self, self_ref: Rc<RefCell<Game>>, mount_id: &str) {
        self.self_ref = Some(self_ref.clone());
        self.view_main = Some(create_view_main(self_ref.clone(), mount_id));
        self.views = Some(self.get_view_main().borrow().views.clone());
        bind!(self.get_view_main(), view_main);
        view_main.hide_all();
    }

    // getter for the main view.
    // this is not done in a macro because it's stored differently
    pub(super) fn get_view_main(&self) -> Rc<RefCell<ViewMain>> {
        let ret = self.view_main.as_ref();
        if ret.is_none() {
            panic!("can't access main_view, not set")
        }
        ret.unwrap().clone()
    }
}

pub struct ProvLookupTable {
    pub pixels: Vec<[u8; 3]>,
    pub width: u32,
    pub max_div: u32,
}

impl ProvLookupTable {
    fn get_flat_ind(&self, coord: &Coord) -> usize {
        return (coord.x + coord.y * self.width as i32) as usize;
    }

    pub fn target_is_valid(&self, coord: &Coord) -> bool {
        // refocusing and other actions can produce canvas events with locations outside the canvas
        let idx = self.get_flat_ind(coord);
        return idx < self.pixels.len();
    }

    fn get_coord(&self, coord: &Coord) -> [u8; 3] {
        let idx = self.get_flat_ind(coord);
        return self.pixels[idx];
    }

    fn compare_colors(&self, target: &Coord, compare: &Coord) -> bool {
        let mut color_div_acc = 0;
        let color_target = self.get_coord(target);
        let color_compare = self.get_coord(compare);
        for i in 0..color_target.len() {
            color_div_acc += (color_target[i] as i32 - color_compare[i] as i32).abs();
        }
        return color_div_acc < self.max_div as i32;
    }

    fn dist_between_pnts(pnt1: &Coord, pnt2: &Coord) -> i32 {
        sqrt((pnt1.x - pnt2.x).pow(2) as f64 + (pnt1.y - pnt2.y).pow(2) as f64) as i32
    }
}
