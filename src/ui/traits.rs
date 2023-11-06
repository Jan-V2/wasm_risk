#![allow(non_camel_case_types)]

use web_sys::Document;

pub trait HTMLable{
    fn mount(&self);// -> result?
    fn set_visibilty(&mut self, is_visible:bool);

}

pub trait HTML_Div where Self:HTMLable {
    fn new(document:&Document, id:String, text:String)-> Self;
    fn set_text(&mut self, new_str:String);
    fn get_text(&self)->String;
}

pub trait HTML_Input<T> where Self:HTMLable {
    fn get(&self) -> T;
}

pub trait HTML_Showable where Self:HTMLable {
    fn set(&mut self, value:bool);
    fn toggle(&mut self);
}


