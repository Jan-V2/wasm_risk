use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;
use crate::element_getters::{get_html_input_by_id, get_html_label_by_id};



pub fn ui_init_max_color_slider(){
    let range = get_html_input_by_id("range");
    let closure_btn = Closure::<dyn FnMut(_)>::new(move |_event:  MouseEvent| {
        let slider_val = get_html_input_by_id("range").value();
        get_html_label_by_id("range_label").set_inner_text(&format!("{slider_val}"))
    });

    let _ = range.add_event_listener_with_callback("input", closure_btn.as_ref().unchecked_ref());
    closure_btn.forget();
}



