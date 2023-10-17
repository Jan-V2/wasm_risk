#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
use js_sys::Math::random;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;
use crate::element_getters::get_button_by_id;
use crate::game::Game;


// todo error checking with option
// because of floor it makes upper extremely unlikely
// todo add proper rounding
pub fn rand_int(lower:u32, upper:u32)->u32{
    let diff = upper - lower ;
    let mut res = lower +  (random() *  diff as f64) as u32;
    if res == upper{
        res = res - 1;
    }
    return res
}

pub fn rand_int_inclusive(lower:u32, upper:u32)->u32{
    let diff = upper - lower;
    return lower +  (random() *  diff as f64) as u32;
}




pub fn setup_tree_builder_btns(game: Rc<RefCell<Game>>){
    // sets up buttons that are used for building the nav tree

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