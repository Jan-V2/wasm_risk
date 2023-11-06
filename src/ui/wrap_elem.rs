
use wasm_bindgen::{Clamped, JsCast, JsValue};
use web_sys::{CssStyleDeclaration, Document, HtmlButtonElement, HtmlCanvasElement, HtmlDivElement, HtmlElement, HtmlImageElement, HtmlLabelElement, HtmlOptionElement, HtmlSelectElement, MouseEvent, Node};
use crate::element_getters::{create_new_elem, get_element_by_id, attach_handler_to_btn};
use crate::ui::traits::*;

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

    fn set_text(&mut self, new_str: String) {
        self.text = new_str;
        self.elem.set_inner_text(self.text.as_str())
    }

    fn get_text(&self) -> String {
        self.text.clone()
    }
}

pub struct WrapSelect{
    elem:HtmlSelectElement,
    id:String,
}

impl WrapSelect {
    pub fn new_from_id(id:String)->WrapSelect{
        let ret = WrapSelect{
            elem: get_element_by_id(id.as_str()).dyn_into::<HtmlSelectElement>().unwrap(),
            id,
        };
        ret
    }
    
    
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
}


pub struct WrapBtn{
    elem:HtmlButtonElement,
    id:String,
    text:String
}

impl WrapBtn {
    fn new(document:&Document, id:String, text:String, click_handler:Box<dyn FnMut(MouseEvent)>) -> Self {
        let ret = WrapBtn{
            elem: create_new_elem(document, "button"),
            id,
            text,
        };
        attach_handler_to_btn(&ret.elem,"click", click_handler);
        ret
    }
}

pub struct WrapDiceCanvas{
    elem:HtmlCanvasElement,
    id:String
}


pub struct WrapHtml {
    template:String,
    elem:HtmlElement,
    id:String
}

impl WrapHtml {
    pub fn new(document:&Document, id:String, template:&str, )-> WrapHtml {
        let elem:HtmlElement = create_new_elem(document, "div");
        elem.set_inner_html(template);
        WrapHtml {
            template:template.to_string(),
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

fn chk_set_visbility(css:&CssStyleDeclaration, is_visible:bool){
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

}

impl HTMLable for WrapDiv {
    fn mount(&self) {
        chk_append_child(self.id.as_str(), &self.elem);
    }

    fn set_visibilty(&mut self, is_visible: bool) {
        chk_set_visbility(&self.elem.style(), is_visible)
    }
}

impl HTMLable for WrapBtn{
    fn mount(&self) {
        chk_append_child(self.id.as_str(), &self.elem);
    }

    fn set_visibilty(&mut self, is_visible: bool) {
        chk_set_visbility(&self.elem.style(), is_visible)
    }
}

impl HTMLable for WrapSelect{
    fn mount(&self) {
        chk_append_child(self.id.as_str(), &self.elem);
    }

    fn set_visibilty(&mut self, is_visible: bool) {
        chk_set_visbility(&self.elem.style(), is_visible)
    }

}
