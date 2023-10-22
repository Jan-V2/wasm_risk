#![allow(dead_code)]
mod element_getters;
mod canvas;
mod model;
mod game;
mod utils;
mod data_include;
mod ui;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use crate::canvas::{get_map_lookup_data};
use crate::game::Game;
use sycamore;
use crate::element_getters::get_element_by_id;
use crate::ui::main::{UiMainProps};
use gloo::console::log as console_log;
use sycamore::prelude::create_signal;


#[wasm_bindgen(start)]
fn setup() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let mut prov_coords:Vec<[i32; 2]> = Vec::new();
    prov_coords.push([388, 204]);
    console_log!("starting");
    let game = Game::new(get_map_lookup_data(50));
    //update_prov_data();


    let refc_game = Rc::from(RefCell::from(game));
    canvas::ui_init_canvas(refc_game.clone());
    //html_elem_setup::setup_tree_builder_btns(refc_game.clone());

    let cloj = || {

        ui::main::UiSide(UiMainProps{
            game_ref: create_signal(refc_game)
        })
    };
    sycamore::render_to(cloj, &get_element_by_id("main"));

}






