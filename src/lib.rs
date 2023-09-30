mod element_getters;
mod canvas;
mod ui;
mod model;
mod game;
mod utils;
mod data_include;
mod syca;

use wasm_bindgen::prelude::*;
use crate::canvas::{get_map_lookup_data};
use crate::game::Game;
use crate::model::{new_prov_array_to_json, NewProvince, prov_array_from_json};

use sycamore;
use web_sys::Node;
use crate::element_getters::get_element_by_id;

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

// todo ui for game setup
// todo show prov name on mouse over
// todo printout div
// todo province selecting
// todo make navigation tree editor
// todo create nav tree
// todo make flag flash when prov is selected?
// this would probably involve some bullshit with .setTimeout
// todo battle ui (on the right of the screen probably)
// todo 3d dice
// todo basic ai
// todo mission cards



fn test_scoping(test:String){
    console_log!("{test}");
}

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

    let test = get_element_by_id("main");
    sycamore::render_to(syca::TestApp, &test);


}


fn update_prov_data(){
    let mut new_provs:Vec<NewProvince> = Vec::new();
    for prov in prov_array_from_json(){
        new_provs.push(NewProvince::from_prov(&prov))
    }
    new_prov_array_to_json(&new_provs);
}



