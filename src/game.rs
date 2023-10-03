use js_sys::Math::{sqrt};
use crate::element_getters::put_text_in_out_field;
use crate::model::{Coord, Model, Player};
use crate::ui_player_setup::PlayerConfig;
use crate::utils::rand_int;



enum GameState {
    Start,
    Setup,
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
    current_state:GameState,
    model:Model,
    prov_lookup:ProvLookupTable,
    flag_scale:f64
}


impl Game {
    pub fn new(prov_lookup:ProvLookupTable) -> Game {
        return Game {
            current_state: GameState::Start,
            model:Model::new_from_json(),
            prov_lookup,
            flag_scale: 0.5,
        }
    }


    fn assign_provs_random(&mut self){
        let player_count = self.model.players.len() as i32;
        for i in 0..self.model.provinces.len(){
            let idx = rand_int(0, player_count as u32 + 1) as i32;
            if idx < player_count{
                self.model.provinces[i].owner_id = idx as u32
            }
        }
    }

    pub fn draw_board(&self){
        if self.model.players.len() > 0 {
            crate::canvas::redraw_board_state(&self.model, self.flag_scale, true);
        }else {
            crate::canvas::redraw_board_state(&self.model, self.flag_scale, false);
        }
    }

    pub fn set_player_config(&mut self, config:PlayerConfig){
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
        if prov_id.is_some(){
            if !self.model.nav_tree.adding_id_set{
                self.model.nav_tree.add_node(prov_id.unwrap());
            }else{
                self.model.nav_tree.add_connection(prov_id.unwrap());
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

}
