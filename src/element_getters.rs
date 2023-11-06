#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlButtonElement, HtmlCanvasElement, HtmlInputElement, HtmlLabelElement, MouseEvent};


pub fn get_html_label_by_id(id :&str) -> HtmlLabelElement{
    return get_element_by_id(id).dyn_into::<HtmlLabelElement>().map_err(|_| ())
        .unwrap();
}

pub fn get_html_input_by_id(id :&str) -> HtmlInputElement{
    return get_element_by_id(id).dyn_into::<HtmlInputElement>().map_err(|_| ())
        .unwrap();
}

pub fn get_button_by_id(id :&str) -> HtmlButtonElement{
    return get_element_by_id(id).dyn_into::<HtmlButtonElement>().map_err(|_| ())
        .unwrap();
}

pub fn get_document()->Document{
    web_sys::window().unwrap().document().unwrap()
}

pub fn get_element_by_id(id :&str) -> Element{
    let document = get_document();
    let res = document.get_element_by_id(id);
    if res.is_none(){
        panic!("could not find element with id {}", id);
    }
    res.unwrap()
}

pub fn set_info_field(string:String){
    let elem = get_element_by_id("text_out").dyn_into::<HtmlLabelElement>()
        .map_err(|_| ()).unwrap();
    elem.set_inner_text(&format!("{}", string));
}



pub fn get_canvas(id:&str) -> HtmlCanvasElement{
    let canvas = get_element_by_id(id);
    return canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
}


pub fn get_drawing_context(canvas :&HtmlCanvasElement) -> CanvasRenderingContext2d{
    return canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
}


pub fn create_new_elem<T>(document:&Document, elem_name: &str)->T
        where T:JsCast{
    document.create_element(elem_name).unwrap().dyn_into().unwrap()
}

pub fn attach_handler_to_btn(btn:&HtmlButtonElement, event_type:&str, clojure: Box<dyn FnMut(MouseEvent)>){
    let closure_start = Closure::<dyn FnMut(_)>::new(clojure);
    let res = btn.add_event_listener_with_callback(
        event_type, closure_start.as_ref().unchecked_ref());
    closure_start.forget();
    if res.is_err(){
        panic!("could not attach handler of type {} to btn with id: {}", event_type, btn.id())
    }
}