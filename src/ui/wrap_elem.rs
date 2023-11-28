use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{JsCast};
use web_sys::{CssStyleDeclaration, Document, HtmlButtonElement, HtmlCanvasElement, HtmlDivElement, HtmlElement, HtmlHeadingElement, HtmlOptionElement, HtmlSelectElement, MouseEvent, Node};
use crate::canvas::{clear_canvas, DiceFaceTex, draw_dice};
use crate::element_getters::{create_new_elem, get_element_by_id, attach_handler_to_btn, get_T_from_id, get_drawing_context};
use crate::model::Coord;
use crate::ui::traits::*;

pub struct WrapHeading{
    elem:HtmlHeadingElement,
    id:String,
    text:String
}


impl HTML_Div for WrapHeading {
    fn new(document:&Document, id:String, text: String) -> Self {
        let ret = WrapHeading {
            elem: create_new_elem(document, "h3"),
            id,
            text,
        };
        ret.elem.set_inner_text(ret.text.as_str());
        ret
    }

    fn set_text(&mut self, new_str: &String) {
        self.text = new_str.clone();
        self.elem.set_inner_text(self.text.as_str())
    }


    fn get_text(&self) -> String {
        self.text.clone()
    }
}


pub struct WrapDiv {
    elem:HtmlDivElement,
    id:String,
    text:String,
}

impl HTML_Div for WrapDiv {
    fn new(document:&Document, id:String, text: String) -> Self {
        let ret = WrapDiv {
            elem: create_new_elem(document, "div"),
            id,
            text,
        };
        ret.elem.set_inner_text(ret.text.as_str());
        ret
    }

    fn set_text(&mut self, new_str: &String) {
        self.text = new_str.clone();
        self.elem.set_inner_text(self.text.as_str())
    }

    fn get_text(&self) -> String {
        self.text.clone()
    }
}

pub struct WrapSelect{
    pub elem:HtmlSelectElement,
    id:String,
}

impl WrapSelect {
    pub fn new(document:&Document, id:String, options:&Vec<(String, String)>) -> WrapSelect{
        let ret = WrapSelect{
            elem: create_new_elem(document, "select"),
            id,
        };
        for option_vals in options{
            let option:HtmlOptionElement = create_new_elem(document, "option");
            option.set_text(option_vals.0.as_str());
            option.set_value(option_vals.1.as_str());
            let _ = ret.elem.append_child(&option);
        }
        ret
    }

    pub fn get_value(&self)->String{
        self.elem.value()
    }

    pub fn set_value(&self, value:&str){
        self.elem.set_value(value)
    }
}


pub struct WrapBtn{
    elem:HtmlButtonElement,
    id:String,
    text:String
}

impl WrapBtn {
    fn new(document:&Document, id:String, text:String) -> Self {
        let ret = WrapBtn{
            elem: create_new_elem(document, "button"),
            id,
            text,
        };
        ret
    }

    pub fn set_click_handler(&mut self, clojure: Box<dyn FnMut(MouseEvent)>){
        attach_handler_to_btn(&self.elem, "click", clojure)
    }

    pub fn set_text(&mut self, text: &str){
        self.elem.set_inner_text(text)
    }
}

pub struct WrapDiceCanvas{
    elem:HtmlCanvasElement,
    id:String
}

impl WrapDiceCanvas {
    pub fn new(document:&Document, id:String, template:&str, )-> WrapDiceCanvas {
        let elem:HtmlCanvasElement = create_new_elem(document, "canvas");
        elem.set_inner_html(template);
        WrapDiceCanvas {
            elem,
            id,
        }
    }

    pub fn draw_dice_rolls(&self, dice_rolls:&Vec<u32>, dice_tex:Rc<RefCell<Vec<DiceFaceTex>>>){

        for i in 0..dice_rolls.len(){
            let roll = dice_rolls[i];
            if roll > 6 || roll == 0{
                panic!("invalid dice roll. number {}", roll)
            }
            let size = self.elem.height();
            draw_dice(get_drawing_context(&self.elem),
                      &dice_tex.as_ref().borrow()[(roll-1) as usize],
            Coord{ y: 0, x: i as i32 * size as i32 }, size )
        }
    }

    pub fn clear_canvas(&self){
        clear_canvas(&self.elem, &get_drawing_context(
            &self.elem), "LightCyan")
    }
}


pub struct WrapHtml {
    elem:HtmlElement,
    id:String
}

impl WrapHtml {
    pub fn new(document:&Document, id:String, template:&str, )-> WrapHtml {
        let elem:HtmlElement = create_new_elem(document, "div");
        elem.set_inner_html(template);
        WrapHtml {
            elem,
            id,
        }
    }
}


fn chk_append_child(id:&str, node:&Node){
    let res =get_element_by_id(id).append_child(node);
    if res.is_err(){
        panic!("could not mount to id {}", id)
    }
}
fn chk_set_css_property(css:&CssStyleDeclaration, property:&str, value:&str ){
    let res = css.set_property(property, value);
    if res.is_err(){
        panic!("unable to set css property {} to {}", property, value)
    }
}

pub fn chk_set_visbility(css:&CssStyleDeclaration, is_visible:bool){
    if is_visible{
        chk_set_css_property(css, "display", "block");
    }else {
        chk_set_css_property(css, "display", "none");
    }

}

impl HTMLable for WrapHtml {
    fn mount(&self) {
        chk_append_child(self.id.as_str(), &self.elem);
    }


    fn set_visibilty(&mut self, is_visible:bool){
        chk_set_visbility(&self.elem.style(), is_visible)
    }

    fn new_from_id(id: &String) -> Self {
        WrapHtml{
            elem: get_T_from_id(id.as_str()),
            id:id.clone(),
        }
    }
}

impl HTMLable for WrapDiv {
    fn mount(&self) {
        chk_append_child(self.id.as_str(), &self.elem);
    }

    fn set_visibilty(&mut self, is_visible: bool) {
        chk_set_visbility(&self.elem.style(), is_visible)
    }

    fn new_from_id(id: &String) -> Self {
        let mut ret = WrapDiv{
            elem: get_T_from_id(id.as_str()),
            id:id.clone(),
            text: "".to_string(),
        };
        ret.text = ret.elem.text_content().unwrap();
        ret
    }
}

impl HTMLable for WrapBtn{
    fn mount(&self) {
        chk_append_child(self.id.as_str(), &self.elem);
    }

    fn set_visibilty(&mut self, is_visible: bool) {
        chk_set_visbility(&self.elem.style(), is_visible)
    }

    fn new_from_id(id: &String) -> Self {
        let mut ret = WrapBtn{
            elem: get_T_from_id(id.as_str()),
            id:id.clone(),
            text: "".to_string(),
        };
        ret.text = ret.elem.text_content().unwrap();
        ret
    }
}

impl HTMLable for WrapSelect{
    fn mount(&self) {
        chk_append_child(self.id.as_str(), &self.elem);
    }

    fn set_visibilty(&mut self, is_visible: bool) {
        chk_set_visbility(&self.elem.style(), is_visible)
    }

    fn new_from_id(id:&String)->Self{
        WrapSelect{
            elem: get_element_by_id(id.as_str()).dyn_into::<HtmlSelectElement>().unwrap(),
            id:id.clone(),
        }
    }
}

impl HTMLable for WrapHeading {
    fn mount(&self) {
        chk_append_child(self.id.as_str(), &self.elem);
    }

    fn set_visibilty(&mut self, is_visible: bool) {
        chk_set_visbility(&self.elem.style(), is_visible)
    }

    fn new_from_id(id: &String) -> Self {
        let mut ret = WrapHeading{
            elem: get_T_from_id(id.as_str()),
            id:id.clone(),
            text: "".to_string(),
        };
        ret.text = ret.elem.text_content().unwrap();
        ret
    }
}

impl HTMLable for WrapDiceCanvas{
    fn mount(&self) {
        chk_append_child(self.id.as_str(), &self.elem);
    }

    fn set_visibilty(&mut self, is_visible: bool) {
        chk_set_visbility(&self.elem.style(), is_visible)
    }

    fn new_from_id(id: &String) -> Self {
        WrapDiceCanvas {
            elem: get_T_from_id(id.as_str()),
            id:id.clone(),
        }
    }

}