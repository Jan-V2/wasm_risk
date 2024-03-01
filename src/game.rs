use crate::menu_stack::MenuStack;
use crate::model::{CombatState, Coord, Model, Player, Rules};
use crate::prov_lookup::ProvLookupTable;
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
use crate::{bind, bind_mut, create_getter};
use gloo::console::log as console_log;
use gloo_timers::callback::Timeout;
use js_sys::Math::sqrt;
use marble::traits::View;
use stack_stack::{stack, Stack};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
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
    pub views: Option<ViewsStruct>,
    pub menu_stack: MenuStack,
    pub combat_state: CombatState,
    pub self_ref: Option<Rc<RefCell<Game>>>,
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
            views: None,
            menu_stack: MenuStack::new(true),
            combat_state: CombatState::default(),
            self_ref: None,
        };
    }

    pub fn pop_menu(&mut self) {
        let stack_len = self.menu_stack.len();
        self.log(format!("popping, stack len is {:?}", stack_len));
        match stack_len {
            0 => {
                panic!("stack empty cannot pop");
            }
            1 => {
                // this means that the current players turn has ended
                //todo check if a player is knocked out and skip their turn
                self.log("at last item in menu_stack, moving on to next player".to_string());
                self.log(format!(
                    "initial_placement_state = {:?}",
                    self.state_turn.in_initial_placement_phase
                ));
                let _ = self.menu_stack.pop();
                if self.state_turn.in_initial_placement_phase {
                    self.start_placement_next_player(false);
                } else {
                    self.setup_next_turn();
                }
            }
            2_u32..=u32::MAX => {
                let _ = self.menu_stack.pop();
                self.activate_menu(self.menu_stack.get().unwrap());
            }
        }
    }

    pub fn pop_menu_async(&self, time_ms: u32) {
        let game_ref = self.self_ref.as_ref().unwrap().clone();
        let t = Timeout::new(time_ms, move || {
            game_ref.borrow_mut().pop_menu();
            console_log!("popped menu async")
        });
        t.forget();
    }

    pub fn activate_menu(&mut self, menu: ViewsEnum) {
        match menu {
            ViewsEnum::Turn => {
                bind_mut!(self.get_turn(), turn);
                turn.update();
            }
            ViewsEnum::ArmyPlacement => {
                bind_mut!(self.get_army_placement(), turn);
                turn.update();
            }
            ViewsEnum::Combat => {
                bind_mut!(self.get_combat(), turn);
                turn.update();
            }
            ViewsEnum::DiceRolls => {
                bind_mut!(self.get_dice_rolls(), turn);
                turn.update();
            }
            ViewsEnum::Message => {
                bind_mut!(self.get_message(), turn);
                turn.update();
            }
        }
        self.get_view_main().borrow().set_active(menu);
    }

    pub fn activate_menu_async(&self, time_ms: u32) {
        let game_ref = self.self_ref.as_ref().unwrap().clone();
        let t = Timeout::new(time_ms, move || {
            let menu = game_ref.borrow().menu_stack.get().unwrap();
            game_ref.borrow_mut().activate_menu(menu);
            console_log!("popped menu async")
        });
        t.forget();
    }
    /*
        pub(super) fn apply_combat_result_to_map(&mut self, state_combat: &StateCombat) {
            self.log("combat finished handler".to_string());
            let mut prov_attack = (*self.model.get_prov_from_id(
                &state_combat.prov_id_attacker).unwrap()).clone();
            let mut prov_defend  = (*self.model.get_prov_from_id(
                &state_combat.prov_id_defender).unwrap()).clone();

            if state_combat.armies_defending == 0 && state_combat.armies_attacking > 0{
                // attacker won
                self.log("attack succeeded".to_string());
                prov_attack.army_count = 1;
                prov_defend.army_count = state_combat.armies_attacking;
                prov_defend.owner_id = prov_attack.owner_id;
            }else {
                // attack ongoing or failed
                self.log("attack failed".to_string());
                prov_attack.army_count = 1 + state_combat.armies_attacking; prov_defend.army_count = state_combat.armies_defending;
            }
            self.log(format!("{:?}", prov_attack));
            self.model.set_prov(prov_attack);
            self.model.set_prov(prov_defend);
            self.draw_board();
        }
    */
    pub fn handle_canvas_noop(&mut self, state: ViewsEnum) {
        self.log(format!("in state: {:?} the canvas is not handled", state))
    }

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

    pub(super) fn set_armies_in_prov(&mut self, num_armies: u32, prov_id: &u32) {
        let prov = self
            .model
            .get_prov_from_id_mut(prov_id)
            .expect(format!("prov with id {} could not be found", prov_id).as_str());
        prov.army_count = num_armies;
    }

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
        pub(super) fn set_ui_state(&mut self, state: UiState) {
            self.state_turn.attack_target = None;
            self.ui_man.select_view(&state);
            self.config_sig.unwrap().ui_state.set(state.clone());
            match state {
                UiState::SETUP => {self.info_display_div.set_default(
                    "".to_string())}
                UiState::ARMY_PLACEMENT | UiState::ARMY_PLACEMENT_START => {
                self.info_display_div.set_default(
                    "Click on your own provinces to place your armies".to_string())}
                UiState::TURN => {
                    self.info_display_div.set_default(
                    "Click on your own province to attack from there, or press end turn".to_string())}
                UiState::MOVE => {todo!()}
                UiState::COMBAT => {self.info_display_div.set_default(
                    "Select the number of armies you want to use and click attack/defend".to_string())}
                UiState::DICE_ROLL => {self.info_display_div.set_default(
                    "".to_string())}
                UiState::GAME_END => {self.info_display_div.set_default(
                    "Game Over".to_string())}
                UiState::CARD_SELECT => {todo!()}
                UiState::LABEL => {
                    self.info_display_div.set_default("".to_string())
                }
            }
        }
    */

    pub fn push_and_activate_menu(&mut self, menu: ViewsEnum) {
        self.menu_stack.push(menu);
        self.activate_menu(self.menu_stack.get().unwrap());
    }

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

    pub(super) fn is_owned_by_active(&self, prov_id: &u32) -> bool {
        let owner = self.model.get_owner_from_prov_id(prov_id);
        if owner.is_some() {
            return owner.unwrap() == self.state_turn.active_player;
        }
        false
    }

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
        self.start_placement_next_player(true);
        self.draw_board();
    }

    pub fn get_prov_mouseover_string(&self, coord: &Coord) -> Option<String> {
        let prov_id = self.lookup_coord(coord);
        return if prov_id.is_some() {
            self.model.get_name_from_prov_id(&prov_id.unwrap())
        } else {
            None
        };
    }

    pub fn create_views(&mut self, self_ref: Rc<RefCell<Game>>, mount_id: &str) {
        self.self_ref = Some(self_ref.clone());
        self.view_main = Some(create_view_main(self_ref.clone(), mount_id));
        self.views = Some(self.get_view_main().borrow().views.clone());
        bind!(self.get_view_main(), view_main);
        view_main.hide_all();
    }

    pub(super) fn get_view_main(&self) -> Rc<RefCell<ViewMain>> {
        let ret = self.view_main.as_ref();
        if ret.is_none() {
            panic!("can't access main_view, not set")
        }
        ret.unwrap().clone()
    }
}
