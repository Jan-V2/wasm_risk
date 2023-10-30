#![allow(non_camel_case_types)]

use web_sys::{Document, HtmlElement, HtmlLabelElement};
use crate::element_getters::{get_html_label_by_id, get_element_by_id, get_html_input_by_id,
                             get_button_by_id, create_new_label, create_new_html};


pub trait HTMLable{
    fn mount(&self);// -> result?
}

pub trait HTML_Label where Self:HTMLable {
    fn new(document:&Document, id:String, text:String)-> Self;
    fn set(&mut self, new_str:String);
    fn get_text(&self)->String;
}

pub trait HTML_Input<T> where Self:HTMLable {
    fn get(&self) -> T;
}

pub trait HTML_Showable where Self:HTMLable {
    fn set(&mut self, value:bool);
    fn toggle(&mut self);
}

pub struct TemplHtml{
    template:String,
    elem:HtmlElement,
    id:String
}

impl TemplHtml {
    pub fn new(document:&Document, id:String, template:String, )->TemplHtml{
        let elem = create_new_html(document);
        elem.set_inner_html(template.as_str());
        TemplHtml{
            template,
            elem,
            id,
        }
    }
}

impl HTMLable for TemplHtml{
    fn mount(&self) {
        let res =get_element_by_id(self.id.as_str()).append_child(&self.elem);
        if res.is_err(){
            panic!("could not mount label to id {}", self.id)
        }
    }
}

pub struct TemplLabel{
    elem:HtmlLabelElement,
    id:String,
    text:String,
    pub count:u32,
}

impl HTMLable for TemplLabel {

    fn mount(&self) {
        let res =get_element_by_id(self.id.as_str()).append_child(&self.elem);
        if res.is_err(){
            panic!("could not mount label to id {}", self.id)
        }
    }
}

impl HTML_Label for TemplLabel{
    fn new(document:&Document, id:String, text: String) -> Self {
        let ret = TemplLabel{
            elem: create_new_label(document),
            id,
            text,
            count: 0,
        };
        ret.elem.set_inner_text(ret.text.as_str());
        ret
    }

    fn set(&mut self, new_str: String) {
        self.text = new_str;
        self.elem.set_inner_text(self.text.as_str())
    }

    fn get_text(&self) -> String {
        self.text.clone()
    }
}

