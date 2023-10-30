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
use crate::element_getters::{attach_handler_to_btn, get_button_by_id, get_document, get_element_by_id};
use crate::ui::main::{UiMainProps};
use gloo::console::log as console_log;
use sycamore::prelude::create_signal;
use crate::ui::army_placement::{HTML_Label, TemplLabel,HTMLable};
use crate::ui::ui_state_manager::UiStateManager;


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
    //html_elem_setup::setup_tree_builder_btns(refc_game.clone());

    let cloj = || {

        ui::main::UiSide(UiMainProps{
            game_ref: create_signal(refc_game)
        })
    };
    sycamore::render_to(cloj, &get_element_by_id("main"));

    let test = UiStateManager::build();
    test.mount();

/*    let  label = TemplLabel::new(&get_document(), "templ_test".to_string(), "start".to_string());
    label.mount("templ_test".to_string());
    let rfc = Rc::from(RefCell::from(label));

    create_test_btn(rfc);
*/
}

fn create_test_btn(rfc:Rc<RefCell<TemplLabel>>){
    let btn = get_button_by_id("tester");

    let handler = Box::from(move|_|{
        rfc.borrow_mut().count += 1;
        let new_str = format!("current count is {}", rfc.borrow().count);
        rfc.borrow_mut().set(new_str)
    });
    attach_handler_to_btn(&btn, "click", handler)
}

