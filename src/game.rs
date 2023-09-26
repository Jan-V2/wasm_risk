use js_sys::Math::{sqrt};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::element_getters::put_text_in_out_field;
use crate::model::{Coord, Model};
use crate::utils::rand_int;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

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

    pub fn setup_ui(&self){
        crate::ui::ui_init_max_color_slider();
        crate::canvas::ui_init_canvas_test_btn();
    }

    pub fn draw_board(&mut self){
        let player_count = 4;
        self.model.test_add_players(player_count);
        for i in 0..self.model.provinces.len(){
            let idx = rand_int(0, player_count as u32 + 1) as i32;
            if idx < player_count{
                self.model.provinces[i].owner_id = idx as u32
            }
        }
        crate::canvas::redraw_board_state(&self.model, self.flag_scale);
    }



    pub fn handle_canvas_click(&self, clicked_coord :Coord){

        let mut found_at_idx:Vec<i32> = Vec::new();

        for i in 0..self.model.provinces.len(){
            if self.prov_lookup.compare_colors(&self.model.provinces[i].location, &clicked_coord) {
                found_at_idx.push(i as i32);
            }
        }

        if found_at_idx.len() == 0{
            let str_out = "found nothing".to_string();
            put_text_in_out_field(str_out.clone());
            //console_log!(str_out)

        }else if found_at_idx.len() == 1 {
            let prov = &self.model.provinces[found_at_idx[0] as usize];
            let str_out = format!("found {} on continent {}", prov.name, prov.continent);
            put_text_in_out_field(str_out.clone());
            //console_log!("{}", str_out);

        }else {
            let idxes_found = found_at_idx.len();
            let mut idx_shortest:i32 = -1;
            let mut shortest_dist = i32::MAX;
            for idx in found_at_idx{
                let dist = ProvLookupTable::dist_between_pnts(&clicked_coord,
                                                              &self.model.provinces[idx as usize].location);
                if dist < shortest_dist{
                    shortest_dist = dist;
                    idx_shortest = idx;
                }
            }
            let prov = &self.model.provinces[idx_shortest as usize];
            let str_out = format!("found {} on continent {}", prov.name, prov.continent);
            put_text_in_out_field(str_out.clone());
            //console_log!("{}", str_out);
        }
    }
}
