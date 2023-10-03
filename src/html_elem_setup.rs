use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent};
use crate::element_getters::{get_button_by_id, get_html_input_by_id, get_html_label_by_id};
use crate::game::Game;


pub fn ui_init_max_color_slider(){
    let range = get_html_input_by_id("range");
    let closure_btn = Closure::<dyn FnMut(_)>::new(move |_event:  MouseEvent| {
        let slider_val = get_html_input_by_id("range").value();
        get_html_label_by_id("range_label").set_inner_text(&format!("{slider_val}"))
    });

    let _ = range.add_event_listener_with_callback("input", closure_btn.as_ref().unchecked_ref());
    closure_btn.forget();
}

pub fn setup_tree_builder_btns(game: Rc<RefCell<Game>>){
    let game_ref_btn2 = game.clone();
    let game_ref_btn3 = game.clone();


    let start=   get_button_by_id("nav_start");
    let end=   get_button_by_id("nav_end");
    let dump = get_button_by_id("nav_dump");

    let closure_start = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        game_ref_btn3.as_ref().borrow_mut().nav_tree_check();
    });
    let _ = start.add_event_listener_with_callback("click", closure_start.as_ref().unchecked_ref());
    closure_start.forget();


    let closure_end = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        game.as_ref().borrow_mut().nav_tree_end_add();
    });
    let _ = end.add_event_listener_with_callback("click", closure_end.as_ref().unchecked_ref());
    closure_end.forget();

    let closure_dump = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        game_ref_btn2.as_ref().borrow().nav_tree_dump();
    });
    let _ = dump.add_event_listener_with_callback("click", closure_dump.as_ref().unchecked_ref());
    closure_dump.forget();
}

