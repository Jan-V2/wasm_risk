
use wasm_bindgen::JsValue;
use web_sys::{CssStyleDeclaration, Document, HtmlButtonElement, HtmlElement, HtmlLabelElement, MouseEvent, Node};
use crate::element_getters::{create_new_elem, get_element_by_id, attach_handler_to_btn};
use crate::ui::traits::*;

pub struct WrapLabel {
    elem:HtmlLabelElement,
    id:String,
    text:String,
}

impl HTML_Label for WrapLabel {
    fn new(document:&Document, id:String, text: String) -> Self {
        let ret = WrapLabel {
            elem: create_new_elem(document, "label"),
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

fn chk_set_visbility(css:&CssStyleDeclaration, is_visible:bool){
    let res:Result<(), JsValue>;
    if is_visible{
        res =css.set_property("display", "block");
    }else {
        res =css.set_property("display", "none");
    }
    if res.is_err(){
        panic!("unable to set template visibility to {}", is_visible)
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

impl HTMLable for WrapLabel {
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