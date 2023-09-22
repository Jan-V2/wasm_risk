mod element_getters;
mod canvas;
mod ui;
mod model;
mod game;

use wasm_bindgen::prelude::*;
use crate::canvas::{get_map_lookup_data, ui_init_canvas, ui_init_canvas_test_btn};
use crate::ui::ui_init_max_color_slider;
use reqwest;
use serde_json::json;
use crate::game::Game;
use crate::model::{Coord, get_map_data,  prov_array_from_json, prov_array_to_json, Province};

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

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();

    let mut prov_coords:Vec<[i32; 2]> = Vec::new();
    prov_coords.push([388, 204]);
    console_log!("test");
    let game = Game::new(get_map_lookup_data(50));

    ui_init_canvas(game);
    ui_init_max_color_slider();
    ui_init_canvas_test_btn();

    // todo make a slection box, that has all the enum in it, so i can assign continents to provs
}




