#![allow(dead_code)]

mod element_getters;
mod canvas;
mod html_elem_setup;
mod model;
mod game;
mod utils;
mod data_include;
mod ui_main;
mod ui_player_setup;


use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use crate::canvas::{get_map_lookup_data};
use crate::game::Game;
use crate::model::{new_prov_array_to_json, NewProvince, prov_array_from_json};
use sycamore;
use sycamore::prelude::Scope;
use crate::element_getters::get_element_by_id;
use crate::ui_main::{ UiMainProps};

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


fn test_scoping(test:String){
    console_log!("{test}");
}

#[wasm_bindgen(start)]
fn setup() {
    console_error_panic_hook::set_once();

    let mut prov_coords:Vec<[i32; 2]> = Vec::new();
    prov_coords.push([388, 204]);
    console_log!("starting");
    let game = Game::new(get_map_lookup_data(50));
    //update_prov_data();



    let refc_game = Rc::from(RefCell::from(game));
    canvas::ui_init_canvas(refc_game.clone());
    html_elem_setup::setup_tree_builder_btns(refc_game.clone());

    let cloj = |cx:Scope| {
        ui_main::UiSide(cx, UiMainProps{
            game_ref: refc_game
        })
    };
    sycamore::render_to(cloj, &get_element_by_id("main"));

}


fn update_prov_data(){
    let mut new_provs:Vec<NewProvince> = Vec::new();
    for prov in prov_array_from_json(){
        new_provs.push(NewProvince::from_prov(&prov))
    }
    new_prov_array_to_json(&new_provs);
}



