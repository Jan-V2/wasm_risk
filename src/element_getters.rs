#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Element, HtmlButtonElement, HtmlCanvasElement, HtmlInputElement, HtmlLabelElement};


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

pub fn get_element_by_id(id :&str) -> Element{
    let document = web_sys::window().unwrap().document().unwrap();
    return document.get_element_by_id(id).unwrap();
}


pub fn put_text_in_out_field(string:String){
    let elem = get_element_by_id("text_out").dyn_into::<HtmlLabelElement>()
        .map_err(|_| ()).unwrap();
    elem.set_inner_text(&format!("{}", string));
}


pub fn get_canvas() -> HtmlCanvasElement{
    let canvas = get_element_by_id("canvas");
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