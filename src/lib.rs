#![allow(dead_code)]
#![allow(unused_imports)]


mod element_getters;
mod event_controller;
mod canvas;
mod model;
mod game;
mod utils;
mod data_include;
mod syca;
mod game_event_handlers;
mod views;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use crate::canvas::get_map_lookup_data;
use crate::game::Game;
use crate::element_getters::{ get_element_by_id};
use gloo::console::log as console_log;
use sycamore::prelude::create_signal;
use crate::syca::main::UiMainProps;

#[wasm_bindgen(start)]
fn setup() {
    console_error_panic_hook::set_once();

    console_log!("starting");

    let game = Game::new(get_map_lookup_data(50), true);
    let refc_game = Rc::from(RefCell::from(game));
    let ref2 = refc_game.clone();
    refc_game.borrow_mut().create_views(ref2, "test2");
    canvas::ui_init_canvas(refc_game.clone());

    let cloj = || {
        syca::main::UiSide(UiMainProps {
            game_ref: create_signal(refc_game)
        })
    };
    sycamore::render_to(cloj, &get_element_by_id("main"));

}
