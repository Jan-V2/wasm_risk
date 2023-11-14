use std::rc::Rc;
use js_sys::Math::sqrt;
use crate::element_getters::set_info_field;
use crate::model::{Coord, Model, Player, Rules};
use crate::ui::player_setup::PlayerConfig;
use crate::utils::funcs::rand_int;
use gloo::console::log as console_log;
use crate::ComBus;
use crate::ui::structs::{UiInfo};
use crate::ui::main::UiState;
use crate::ui::ui_state_manager::{UpdateableState};


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
    com_bus:Option<Rc<ComBus>>,
    logging:bool,
}


impl Game {
    pub fn new(prov_lookup: ProvLookupTable, use_logging:bool) -> Game {
        return Game {
            model: Model::new_from_json(),
            prov_lookup,
            flag_scale: 0.5,
            ui_info: None,
            com_bus:None,
            logging: use_logging,
        };
    }

    pub fn add_combus(&mut self, com_bus:Rc<ComBus>){
        self.com_bus = Some(com_bus)
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
        let player_count = self.model.players.len();
        let prov_total = self.model.provinces.len() as u32;

        let mut provs_available:Vec<u32> = vec![];
        let provs_per_player = prov_total / player_count as u32;
        let mut remainder = prov_total % player_count as u32;

        for player_id in 0..player_count{
            provs_available.push(provs_per_player);
            if remainder > 0{
                provs_available[player_id] +=1;
                remainder -= 1;
            }
        }

        let mut provs_vec:Vec<_> = (0..prov_total).collect();

        for p in 0..player_count{
            for _ in 0..provs_available[p]{
                let idx = rand_int(0, (provs_vec.len() -1) as u32) as usize;
                self.model.provinces[provs_vec[idx] as usize].owner_id = p as u32;
                self.model.provinces[provs_vec[idx] as usize].army_count = 1;
                provs_vec.remove(idx);
            }
        }


    }

    pub fn set_player_config(&mut self, config: PlayerConfig) {
        gloo::console::log!(format!("player count = {}", config.player_count));
        gloo::console::log!(format!("{:?}", config));
        for i in 0..config.player_count {
            self.model.players.push(Player {
                id: i as u32,
                cards: vec![],
                color: config.player_colors.get_clone()[i as usize].as_str().to_string(),
                is_computer: config.player_is_ai[i as usize],
            })
        }
        self.log(format!("{:?}", self.model.players));
        let armies_per_player = Rules::armies_per_players_start(config.player_count as u32).unwrap();
        self.assign_provs_random();


        let provs = &self.model.provinces;
        let mut count_id = vec![0u32; config.player_count as usize];
        for prov in provs{
            count_id[prov.owner_id as usize] += 1;
        }
        self.log(format!("provinces per player {:?}", count_id));

        self.com_bus.as_ref().unwrap().get_ui().borrow_mut().start_army_placement.state.update(|state|{
            state.active = true;
            state.num_players = config.player_count as u32;
            for i in 0..state.num_players as usize{
                state.armies[i] = armies_per_player - self.model.players[i].get_owned_provs(provs).len() as u32;
                self.log(format!("found {} armies for player {}", state.armies[i], i));

            }
        });
        self.com_bus.as_ref().unwrap().get_ui().borrow_mut().update_all();
        self.ui_info.unwrap().ui_state.set(UiState::ARMY_PLACEMENT_START);

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

    fn is_owned_by_active(&self, prov_id:&u32)->bool{
        let owner = self.model.get_owner_from_prov_id(prov_id);
        if owner.is_some(){
            return owner.unwrap() == self.ui_info.unwrap().active_player.get();
        }
        false
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

    fn set_ui_state(&mut self, state:UiState){
        self.ui_info.unwrap().ui_state.set(state)
    }

    fn get_ui_state(&mut self)->UiState{
        self.ui_info.unwrap().ui_state.get()
    }

    fn set_active_player(&mut self, active_player:u32){
        self.ui_info.unwrap().active_player.set(active_player)
    }

    fn get_active_player(&mut self)->u32{
        self.ui_info.unwrap().active_player.get()
    }

    fn handle_turn(&mut self, prov_id:u32){
        self.log("turn".to_string())
    }

    fn handle_army_placement(&mut self, prov_id: u32, placement_start: bool) {
        if !self.is_owned_by_active(&prov_id){
            self.log("placement: prov not owned by active player".to_string());
            return;
        }

        console_log!(format!("running placement id {}, start {}", prov_id, placement_start));
        let armies_available: u32 = if placement_start {
            let tmp = self.com_bus.as_ref().unwrap().get_ui().borrow().start_army_placement.state.clone();
            tmp.armies[tmp.current_player as usize]
        } else {
            self.com_bus.as_ref().unwrap().get_ui().borrow().army_placement.state.armies
        };


        if armies_available > 0 {
            self.change_armies_in_prov(1, &prov_id);
            if placement_start {

                self.com_bus.as_ref().unwrap().get_ui().borrow_mut().
                    start_army_placement.state.update(|state |{
                    state.armies[state.current_player as usize] -=1;
                    if state.armies[state.current_player as usize] == 0{
                        if state.current_player + 1 < state.num_players{
                            state.current_player += 1;
                            self.ui_info.unwrap().active_player.set(state.current_player);
                        }else {
                            state.active = false;
                            self.ui_info.unwrap().active_player.set(0);
                            self.ui_info.unwrap().ui_state.set(UiState::TURN);
                        }
                    }
                });
            } else {
                self.com_bus.as_ref().unwrap().get_ui().borrow_mut().
                    army_placement.state.update(|state |{
                    state.armies -= 1;
                    if state.armies == 0{
                        state.active = false;
                        //todo next state

                    }
                });
            }
        } else {
            panic!("in placement state, with 0 armies to Place {:?}",
                       self.com_bus.as_ref().unwrap().get_ui().borrow().start_army_placement.state);

        }
        self.com_bus.as_ref().unwrap().get_ui().borrow_mut().update_all();
        self.draw_board();
    }

    fn next_state(&mut self){
        match self.ui_info_ref().ui_state.get() {
            UiState::SETUP => {}
            UiState::ARMY_PLACEMENT_START => {}
            UiState::ARMY_PLACEMENT => {}
            UiState::TURN_START => {}
            UiState::TURN => {}
            UiState::COMBAT => {}
            UiState::GAME_END => {}
            UiState::CARD_SELECT => {}
        }
    }

    pub fn nav_tree_end_add(&mut self) {
        self.model.nav_tree.end_add();
    }

    pub fn nav_tree_dump(&self) {
        gloo::console::log!("dumping nav tree");
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


    fn log(&self, msg:String){
        if self.logging{
            console_log!(msg)
        }
    }

    pub fn set_ui_info(&mut self, info: UiInfo) {
        self.ui_info = Some(info);
    }
}
