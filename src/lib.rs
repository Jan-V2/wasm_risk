mod element_getters;
mod canvas;
mod ui;
mod model;
mod game;
mod utils;
mod map_data;

use wasm_bindgen::prelude::*;
use crate::canvas::{get_map_lookup_data};
use crate::game::Game;
use crate::model::{new_prov_array_to_json, NewProvince, prov_array_from_json};

// todo figure out how to export this from a module
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

// todo add cards to provs
// todo fix gap in prov
// todo add names to provs
// todo add player colors to prov flags
// todo make flags flash when prov is selected?
// this would probably involve some bullshit with .setTimeout

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();

    let mut prov_coords:Vec<[i32; 2]> = Vec::new();
    prov_coords.push([388, 204]);
    console_log!("test");
    let mut game = Game::new(get_map_lookup_data(50));

    game.setup_ui();
    game.draw_board();

    // this has to be done from the outer scope, because a ref can't go into a closure.
    crate::canvas::ui_init_canvas(game);
}


fn update_prov_data(){
    let mut new_provs:Vec<NewProvince> = Vec::new();
    for prov in prov_array_from_json(){
        new_provs.push(NewProvince::from_prov(&prov))
    }
    new_prov_array_to_json(&new_provs);
}



