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
use crate::element_getters::{ get_element_by_id};
use gloo::console::log as console_log;
use sycamore::prelude::create_signal;
use crate::ui::main::UiMainProps;
use crate::ui::ui_state_manager::{ UiStateManager};

pub struct ComBus{
    game_ref:Option<Rc<RefCell<Game>>>,
    ui_ref:Option<Rc<RefCell<UiStateManager>>>
}

impl ComBus {
    pub fn new()->ComBus{
        ComBus{
            game_ref: None,
            ui_ref: None,
        }
    }

    pub fn add_game(&mut self, game:Rc<RefCell<Game>>){
        self.game_ref = Some(game)
    }

    pub fn add_ui_manager(&mut self, ui_manager:Rc<RefCell<UiStateManager>>){
        self.ui_ref = Some(ui_manager)
    }

    pub fn get_ui(&self)->Rc<RefCell<UiStateManager>>{
        self.ui_ref.as_ref().unwrap().clone()
    }

    pub fn get_game(&self)->Rc<RefCell<Game>>{
        self.game_ref.as_ref().unwrap().clone()
    }
}

#[wasm_bindgen(start)]
fn setup() {
    console_error_panic_hook::set_once();

    console_log!("starting");

    let game = Game::new(get_map_lookup_data(50), true);
    let refc_game = Rc::from(RefCell::from(game));

    let mut ui_state = UiStateManager::build();
    ui_state.mount();
    let refc_ui = Rc::from(RefCell::from(ui_state));


    let mut com_bus = ComBus::new();
    com_bus.add_game(refc_game.clone());
    com_bus.add_ui_manager(refc_ui.clone());
    let combus_rc = Rc::from(com_bus);

    refc_game.borrow_mut().add_combus(combus_rc.clone());
    refc_ui.borrow_mut().add_combus(combus_rc);


    canvas::ui_init_canvas(refc_game.clone());




/*
    let handler = Box::from(move|_|{
//        let selected = ui_state.selected.clone();
        //ui_state.update_all();
        ui_state.dice_rolls.update(CombatResult{
            armies_attacker:10,
            armies_defender:5,
            losses_defender:1,
            losses_attacker:1,
            dice_roll_attacker:vec![6,3,1],
            dice_roll_defender :vec![5,5],
            combat_finished:false
        });
    });

    let btn = get_button_by_id("tester");
    attach_handler_to_btn(&btn, "click", handler);

*/

    let cloj = || {
        ui::main::UiSide(UiMainProps {
            game_ref: create_signal(refc_game)
        })
    };
    sycamore::render_to(cloj, &get_element_by_id("main"));


}
