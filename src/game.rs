use js_sys::Math::sqrt;
use crate::element_getters::set_info_field;
use crate::model::{Coord, Model, Player, Rules};
use crate::ui::player_setup::PlayerConfig;
use crate::utils::funcs::rand_int;
use gloo::console::log as console_log;
use crate::ui::structs::{UiInfo, UiUpdatable};
use crate::ui::main::UiState;


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
        sqrt((pnt1.x - pnt2.x).pow(2) as f64 +
            (pnt1.y - pnt2.y).pow(2) as f64) as i32
    }
}

pub struct Game {
    pub model: Model,
    prov_lookup: ProvLookupTable,
    flag_scale: f64,
    ui_info: Option<UiInfo>,
}





impl Game {
    pub fn new(prov_lookup: ProvLookupTable) -> Game {
        return Game {
            model: Model::new_from_json(),
            prov_lookup,
            flag_scale: 0.5,
            ui_info: None,
        };
    }

    fn ui_info_ref(&self) -> &UiInfo {
        self.ui_info.as_ref().unwrap()
    }

    pub fn draw_board(&self) {
        if self.model.players.len() > 0 {
            crate::canvas::redraw_board_state(&self.model, self.flag_scale, true);
        } else {
            crate::canvas::redraw_board_state(&self.model, self.flag_scale, false);
        }
    }

    fn assign_provs_random(&mut self) {
        // todo make this not a random number per player
        gloo::console::log!(format!("players len = {}", self.model.players.len()));
        let player_count = self.model.players.len() as i32;
        for i in 0..self.model.provinces.len() {
            let idx = rand_int(0, player_count as u32) as i32;
            if idx < player_count {
                self.model.provinces[i].owner_id = idx as u32;
                self.model.provinces[i].army_count = 1;
            }
        }
    }

    pub fn set_player_config(&mut self, config: PlayerConfig) {
        gloo::console::log!(format!("player count = {}", config.player_count));
        for i in 0..config.player_count {
            self.model.players.push(Player {
                id: i as u32,
                cards: vec![],
                color: config.player_colors.get_clone()[i as usize].as_str().clone().to_string(),
                is_computer: config.player_is_ai[i as usize],
            })
        }
        let armies_per_player = Rules::armies_per_players_start(config.player_count as u32).unwrap();
        self.assign_provs_random();

        let provs = &self.model.provinces;
        self.ui_info_ref().start_placement.set(self.ui_info_ref().start_placement.get().update(|tmp| {
            tmp.num_players = config.player_count as u32;
            for i in 0..tmp.num_players as usize {
                let armies = armies_per_player - self.model.players[i].get_owned_provs(provs).len() as u32;
                console_log!(format!("found {} armies for player {}", armies, i));
                tmp.armies_per_player[i] = armies;
            }
        }));

        self.draw_board();
    }


    pub fn get_prov_location_string(&self, coord: &Coord) -> Option<String> {
        let prov_id = self.lookup_coord(coord);
        if prov_id.is_some() {
            let prov = &self.model.provinces[prov_id.unwrap() as usize];
            return Some(format!("found {} on continent {}", prov.name, prov.continent));
        } else {
            return None;
        }
    }

    pub fn get_prov_mouseover_string(&self, coord: &Coord) -> Option<String> {
        let prov_id = self.lookup_coord(coord);
        if prov_id.is_some() {
            let prov = &self.model.provinces[prov_id.unwrap() as usize];
            return Some(prov.name.clone());
        } else {
            return None;
        }
    }

    pub fn lookup_coord(&self, clicked_coord: &Coord) -> Option<u32> {
        let mut found_at_idx: Vec<i32> = Vec::new();

        if !self.prov_lookup.target_is_valid(clicked_coord) {
            return None;
        }

        for i in 0..self.model.provinces.len() {
            if self.prov_lookup.compare_colors(&self.model.provinces[i].location, &clicked_coord) {
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
                let dist = ProvLookupTable::dist_between_pnts(&clicked_coord,
                                                              &self.model.provinces[idx as usize].location);
                if dist < shortest_dist {
                    shortest_dist = dist;
                    idx_shortest = idx;
                }
            }
            Some(idx_shortest as u32)
        }
    }

    pub fn handle_canvas_click(&mut self, clicked_coord: Coord) {
        let prov_str = self.get_prov_location_string(&clicked_coord);
        if prov_str.is_some() {
            set_info_field(prov_str.unwrap());
        } else {
            set_info_field("".to_owned());
        }

        let prov_id_opt = self.lookup_coord(&clicked_coord);
        if prov_id_opt.is_some() {
            let prov_id = prov_id_opt.unwrap();
            match self.ui_info_ref().ui_state.get() {
                UiState::SETUP => {}
                UiState::ARMY_PLACEMENT_START => self.handle_army_placement(prov_id, true),
                UiState::ARMY_PLACEMENT => self.handle_army_placement(prov_id, false),
                UiState::TURN_START => {}
                UiState::TURN => {}
                UiState::COMBAT => {}
                UiState::GAME_END => {}
                UiState::CARD_SELECT => {}
            }
        }
    }

    fn handle_army_placement(&mut self, prov_id: u32, placement_start: bool) {
        console_log!(format!("running placement id {}, start {}", prov_id, placement_start));
        let armies_available: u32 = if placement_start {
            let tmp = self.ui_info_ref().start_placement.get();
            tmp.armies_per_player[tmp.current_player as usize]
        } else {
            let tmp = self.ui_info_ref().placement.get();
            tmp.army_count
        };


        if armies_available > 0 {
            self.change_armies_in_prov(1, &prov_id);
            if placement_start {
                self.ui_info_ref().start_placement.set(self.ui_info_ref().start_placement.get().update(|tmp| {
                    tmp.armies_per_player[tmp.current_player as usize] = armies_available - 1;
                    if armies_available == 1 {
                        tmp.is_done = true;
                    }
                }))
            } else {
                self.ui_info_ref().placement.set(self.ui_info_ref().placement.get().update(|tmp| {
                    tmp.army_count = armies_available - 1;
                    if armies_available == 1 {
                        tmp.is_done = true;
                    }
                }))
            }
        } else {
            /*if placement_start {
                panic!("in placement state, with 0 armies to place {:?}", self.ui_info_ref().start_placement.get());
            } else {
                panic!("in placement state, with 0 armies to place {:?}", self.ui_info_ref().placement.get());
            };*/
        }
        self.draw_board();
    }

    pub fn nav_tree_end_add(&mut self) {
        self.model.nav_tree.end_add();
    }

    pub fn nav_tree_dump(&self) {
        gloo::console::log!("dumping");
        gloo::console::log!(serde_json::to_string(&self.model.nav_tree).unwrap());
    }

    pub fn nav_tree_check(&self) {
        self.model.nav_tree.verify_self();
    }


    pub fn change_armies_in_prov(&mut self, num_armies: i32, prov_id: &u32) {
        let prov = self.model.get_prov_from_id_mut(prov_id)
            .expect(format!("prov with id {} could not be found", prov_id).as_str());
        let old_army_count = prov.army_count;

        let new_army_count = old_army_count as i32 + num_armies;
        if new_army_count > -1 {
            prov.army_count = new_army_count as u32;
        } else {
            prov.army_count = 0;
        }
    }


    pub fn set_ui_info(&mut self, info: UiInfo) {
        self.ui_info = Some(info);
    }
}
