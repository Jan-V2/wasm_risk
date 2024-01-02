use crate::model::{Coord, Model, Player, Rules};
use crate::ui::main::{UiInfo, UiState};
use crate::ui::player_setup::PlayerConfig;
use crate::ui::ui_state_manager::{
     StatefullView, UiStateManager,
};



use crate::utils::funcs::rand_int;

use gloo::console::log as console_log;
use js_sys::Math::sqrt;
use std::cell::RefCell;
use std::rc::Rc;
use crate::ui::view_combat::StateCombat;
use crate::ui::view_info::ViewInfo;
use crate::ui::view_label::StateLabel;

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

#[derive(Default)]
pub struct GameTurnState {
    pub(super) attack_target: Option<u32>,
}

pub struct Game {
    pub model: Model,
    pub prov_lookup: ProvLookupTable,
    pub flag_scale: f64,
    pub config_sig: Option<UiInfo>,
    pub logging: bool,
    pub state_turn: GameTurnState,
    pub ui_man: UiStateManager,
    pub info_display_div: ViewInfo
}

impl Game {
    pub fn new(prov_lookup: ProvLookupTable, use_logging: bool) -> Game {
        let mut ui_state_man = UiStateManager::build();
        ui_state_man.mount();
        return Game {
            model: Model::new_from_json(),
            prov_lookup,
            flag_scale: 0.5,
            config_sig: None,
            logging: use_logging,
            state_turn: Default::default(),
            ui_man: ui_state_man,
            info_display_div: ViewInfo::create(
                &"text_out".to_string(), "setup".to_string()),
        };
    }

    pub(super) fn apply_combat_result_to_map(&mut self, state_combat: &StateCombat) {
        self.log("combat finished handler".to_string());
        let mut prov_attack = (*self.model.get_prov_from_id(&state_combat.prov_id_attacker).unwrap()).clone();
        let mut prov_defend  = (*self.model.get_prov_from_id(&state_combat.prov_id_defender).unwrap()).clone();

        if state_combat.armies_defending == 0 && state_combat.armies_attacking > 0{
            // attacker won
            self.log("attack succeeded".to_string());
            prov_attack.army_count = 1;
            prov_defend.army_count = state_combat.armies_attacking;
            prov_defend.owner_id = prov_attack.owner_id;
        }else {
            // attack ongoing or failed
            self.log("attack failed".to_string());
            prov_attack.army_count = 1 + state_combat.armies_attacking;
            prov_defend.army_count = state_combat.armies_defending;
        }
        self.log(format!("{:?}", prov_attack));
        self.model.set_prov(prov_attack);
        self.model.set_prov(prov_defend);
        self.draw_board();
    }

    pub fn handle_canvas_noop(&mut self, state :UiState){
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

    pub(super) fn assign_provs_random(&mut self) {
        gloo::console::log!(format!("players len = {}", self.model.players.len()));
        let player_count = self.model.players.len();
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



    pub(super) fn ui_info_ref(&self) -> &UiInfo {
        self.config_sig.as_ref().unwrap()
    }

    pub fn set_config_sig(&mut self, info: UiInfo) {
        self.config_sig = Some(info);
    }

    pub fn show_label(&mut self, label:String, next_state:UiState){
        self.ui_man.view_label.update(StateLabel{
            label_text: label, return_state:  next_state.clone()});
        self.set_ui_state(UiState::LABEL);
    }

    pub(super) fn set_ui_state(&mut self, state: UiState) {
        self.state_turn.attack_target = None;
        self.ui_man.select_view(&state);
        self.config_sig.unwrap().ui_state.set(state.clone());
        match state {
            UiState::SETUP => {self.info_display_div.set_default(
                "".to_string())}
            UiState::ARMY_PLACEMENT | UiState::ARMY_PLACEMENT_START => {self.info_display_div.set_default(
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
            return owner.unwrap() == self.config_sig.unwrap().active_player.get();
        }
        false
    }

    pub fn set_active_player_signal(&mut self, active_player: u32) {
        self.config_sig.unwrap().active_player.set(active_player)
    }

    pub fn get_active_player(&mut self) -> u32 {
        self.config_sig.unwrap().active_player.get()
    }

    pub fn set_player_config(&mut self, config: PlayerConfig) {
        gloo::console::log!(format!("player count = {}", config.player_count));
        gloo::console::log!(format!("{:?}", config));
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
        self.log(format!("{:?}", self.model.players));
        let armies_per_player =
            Rules::armies_per_players_start(config.player_count as u32).unwrap();
        self.assign_provs_random();

        let provs = &self.model.provinces;
        let mut count_id = vec![0u32; config.player_count as usize];
        for prov in provs {
            count_id[prov.owner_id as usize] += 1;
        }
        self.log(format!("provinces per player {:?}", count_id));

        let mut state = self.ui_man.start_army_placement.get();
        state.num_players = config.player_count as u32;

        for i in 0..state.num_players as usize {
            state.armies[i] =
                armies_per_player - self.model.players[i].get_owned_provs(provs).len() as u32;
            self.log(format!("found {} armies for player {}", state.armies[i], i));
        }
        self.ui_man.start_army_placement.update(state);
        self.set_ui_state(UiState::ARMY_PLACEMENT_START);
        self.draw_board();
    }

    pub fn get_prov_mouseover_string(&self, coord: &Coord) -> Option<String> {
        let prov_id = self.lookup_coord(coord);
        if prov_id.is_some() {
            return self.model.get_name_from_prov_id(&prov_id.unwrap());
        } else {
            return None;
        }
    }

    pub fn set_self_ref(&mut self, self_ref: Rc<RefCell<Game>>) {
        self.ui_man.set_handlers(&self_ref)
    }

}
