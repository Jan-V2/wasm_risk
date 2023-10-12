use js_sys::Math::{sqrt};
use sycamore::prelude::{create_rc_signal, RcSignal};
use crate::element_getters::put_text_in_out_field;
use crate::model::{Coord, Model, Player, Rules};
use crate::ui_player_setup::PlayerConfig;
use crate::utils::rand_int;
use gloo::console::log as console_log;
use crate::ui_main::UiInfo;


enum GameState {
    Setup,
    ArmyPlacementStart,
    Turn,
    ArmyPlacement,

}

pub struct ProvLookupTable {
    pub pixels:Vec<[u8; 3]>,
    pub width:u32,
    pub max_div:u32
}

impl ProvLookupTable{
    fn get_coord (&self, coord: &Coord) -> [u8; 3] {
        let idx = (coord.x + coord.y * self.width as i32) as usize;
        return self.pixels[idx];
    }

    fn compare_colors(&self, target: &Coord, compare:&Coord, ) -> bool{
        let mut color_div_acc = 0;
        let color_target = self.get_coord(target);
        let color_compare = self.get_coord(compare);
        for i in 0..color_target.len(){
            color_div_acc += (color_target[i] as i32 - color_compare[i] as i32).abs();
        }
        return color_div_acc < self.max_div as i32;
    }

    fn dist_between_pnts(pnt1:&Coord, pnt2:&Coord) -> i32{
        sqrt((pnt1.x - pnt2.x).pow(2) as f64 +
            (pnt1.y - pnt2.y).pow(2) as f64) as i32
    }
}

pub struct Game {
    state:GameState,
    pub model:Model,
    prov_lookup:ProvLookupTable,
    flag_scale:f64,
    army_placement_sig:Option<RcSignal<u32>>,
    army_placement_done: RcSignal<bool>,
    ui_info:RcSignal<UiInfo>,
}


impl Game {
    pub fn new(prov_lookup:ProvLookupTable) -> Game {
        return Game {
            state: GameState::Setup,
            model:Model::new_from_json(),
            prov_lookup,
            flag_scale: 0.5,
            army_placement_sig: Option::None,
            army_placement_done: Default::default(),
            ui_info: create_rc_signal(UiInfo::new()),
        }
    }

    pub fn draw_board(&self){
        if self.model.players.len() > 0 {
            crate::canvas::redraw_board_state(&self.model, self.flag_scale, true);
        }else {
            crate::canvas::redraw_board_state(&self.model, self.flag_scale, false);
        }
    }

    fn assign_provs_random(&mut self){
        gloo::console::log!(format!("players len = {}", self.model.players.len()));
        let player_count = self.model.players.len() as i32;
        for i in 0..self.model.provinces.len(){
            let idx = rand_int(0, player_count as u32 ) as i32;
            if idx < player_count{
                self.model.provinces[i].owner_id = idx as u32
            }
        }
    }

    pub fn set_player_config(&mut self, config:PlayerConfig){
        gloo::console::log!(format!("player count = {}", config.player_count));
        for i in 0..config.player_count{
            self.model.players.push(Player{
                id: i as u32,
                cards: vec![],
                color: config.player_colors[i as usize].clone(),
                is_computer: config.player_is_ai[i as usize],
            })
        }
        self.assign_provs_random();
        self.draw_board();
        self.state = GameState::ArmyPlacementStart;
    }

    #[allow(unused_variables)]
    pub fn lookup_prov_id(&self, prov_id:u32){
        todo!()
    }

    pub fn get_prov_location_string(&self, coord:&Coord) ->Option<String>{
        let prov_id = self.lookup_coord(coord);
        if prov_id.is_some(){
            let prov = &self.model.provinces[prov_id.unwrap() as usize];
            return Some( format!("found {} on continent {}", prov.name, prov.continent));
        }else {
            return None
        }
    }

    pub fn get_prov_mouseover_string(&self, coord:&Coord) -> Option<String>{
        let prov_id = self.lookup_coord(coord);
        if prov_id.is_some(){
            let prov = &self.model.provinces[prov_id.unwrap() as usize];
            return Some( prov.name.clone());
        }else {
            return None
        }
    }

    pub fn lookup_coord(&self, clicked_coord:&Coord)-> Option<u32>{
        let mut found_at_idx:Vec<i32> = Vec::new();

        for i in 0..self.model.provinces.len(){
            if self.prov_lookup.compare_colors(&self.model.provinces[i].location, &clicked_coord) {
                found_at_idx.push(i as i32);
            }
        }

        if found_at_idx.len() == 0{
            None
        }else if found_at_idx.len() == 1 {
            Some(found_at_idx[0] as u32)
        }else {
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

    pub fn handle_canvas_click(&mut self, clicked_coord :Coord){
        let prov_str = self.get_prov_location_string(&clicked_coord);
        if prov_str.is_some(){
            put_text_in_out_field(prov_str.unwrap());
        }else{
            put_text_in_out_field("".to_owned());
        }

        let prov_id = self.lookup_coord(&clicked_coord);
        let mut tmp_ui_info = *self.ui_info.get();
        if prov_id.is_some() && tmp_ui_info.army_placement{
            let current_armies_available = tmp_ui_info.army_count;
            if current_armies_available> 0{
                let id = prov_id.unwrap();
                self.change_armies_in_prov(1, &id);
                tmp_ui_info.army_count = tmp_ui_info.army_count -1;
                tmp_ui_info.updated = true;
                self.ui_info.set(tmp_ui_info);
                console_log!("placed an army in ", self.model.get_prov_name_from_id(&id));
                self.draw_board();
            }else {
                tmp_ui_info.updated = true;
                tmp_ui_info.is_done = true;
                self.ui_info.set(tmp_ui_info);
                console_log!("no armies to place");
            }
        }
    }

    pub fn nav_tree_end_add(&mut self){
        self.model.nav_tree.end_add();
    }

    pub fn nav_tree_dump(&self){
        gloo::console::log!("dumping");
        gloo::console::log!(serde_json::to_string(&self.model.nav_tree).unwrap());
    }

    pub fn nav_tree_check(&self){
        self.model.nav_tree.verify_self();
    }

    pub fn set_army_placement_sig(&mut self, num_army_sig:RcSignal<u32>, army_placement_done:RcSignal<bool>){
        self.army_placement_sig = Some(num_army_sig);
        self.army_placement_done = army_placement_done;

    }

    pub fn change_armies_in_prov(&mut self, num_armies:i32 ,prov_id:&u32){
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

    pub fn get_free_armies_available_start(&self, player_id:u32) -> u32{
        let armies_total = Rules::armies_per_players_start(self.model.players.len() as u32).unwrap();
        let owned = self.model.players[player_id as usize].get_owned_provs(&self.model.provinces).len() as u32;
        return armies_total - owned;
    }

    pub fn get_ui_info_clone(&self) -> RcSignal<UiInfo>{
        return self.ui_info.clone();
    }

}
