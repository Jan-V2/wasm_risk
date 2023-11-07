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
use crate::canvas::get_map_lookup_data;
use crate::game::Game;
use crate::element_getters::{attach_handler_to_btn, get_button_by_id, /*get_element_by_id*/};
use gloo::console::log as console_log;
use crate::ui::ui_state_manager::{Selected, StateCombat, StatefullView, UiStateManager};

#[wasm_bindgen(start)]
fn setup() {
    console_error_panic_hook::set_once();

    let mut prov_coords: Vec<[i32; 2]> = Vec::new();
    prov_coords.push([388, 204]);
    console_log!("starting");
    let game = Game::new(get_map_lookup_data(50));
    //update_prov_data();


    let refc_game = Rc::from(RefCell::from(game));
    canvas::ui_init_canvas(refc_game.clone());
    //html_elem_setup::setup_tree_builder_btns(refc_game.clone());

    let mut ui_state = UiStateManager::build(refc_game.clone());
    ui_state.mount();
    ui_state.combat.update(StateCombat{
        active: true,
        attack_location:"West Europe".to_string(),
        armies_attacking:5,
        armies_defending:10,
        id_attacker: Some(1),
        id_defender:Some(3),
    });
/*
    let handler = Box::from(move|_|{
       let selected = ui_state.selected.clone();

        match selected{
            Selected::Header => {
                gloo::console::log!("header");
                ui_state.selected = Selected::StartPlace;
                ui_state.header.state.active = true;
                ui_state.start_army_placement.state.active = false;
                ui_state.army_placement.state.active = false;
            }
            Selected::StartPlace => {
                gloo::console::log!("start place");
                ui_state.selected = Selected::Place;
                ui_state.header.state.active = false;
                ui_state.start_army_placement.state.active = true;
                ui_state.army_placement.state.active = false;

            }
            Selected::Place => {
                gloo::console::log!("place");
                ui_state.selected = Selected::Header;
                ui_state.header.state.active = false;
                ui_state.start_army_placement.state.active = false;
                ui_state.army_placement.state.active = true;
            }
        }

        ui_state.update_all();
    });
*/


    let handler = Box::from(move|_|{
        let selected = ui_state.selected.clone();
        ui_state.update_all();
    });

    let btn = get_button_by_id("tester");
    attach_handler_to_btn(&btn, "click", handler);



/*    let cloj = || {
        ui::main::UiSide(UiMainProps {
            game_ref: create_signal(refc_game)
        })
    };
    //sycamore::render_to(cloj, &get_element_by_id("main"));
*/

}
