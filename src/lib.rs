mod element_getters;
mod canvas;
mod ui;
mod model;

use wasm_bindgen::prelude::*;
use crate::canvas::{ui_init_canvas, ui_init_canvas_test_btn};
use crate::ui::ui_init_max_color_slider;
use crate::model::Province;
use serde_json::json;


// todo figure out how to export this from a module
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();

    let mut prov_coords:Vec<[i32; 2]> = Vec::new();
    prov_coords.push([388, 204]);
    console_log!("test");
    let temp_coords = vec![];


    ui_init_canvas(50, temp_coords.clone());
    ui_init_max_color_slider();
    ui_init_canvas_test_btn();

    // todo make a slection box, that has all the enum in it, so i can assign continents to provs



/*    let mut temp_json_objs:Vec<serde_json::Value> = Vec::new();
    for coord in temp_coords{
        temp_json_objs.push(Province::from_i32_pair(coord[0], coord[1]).to_json())
    }
    let mut out_str = "[".to_string();
    out_str = format!("{} {}", out_str, temp_json_objs[0].to_string());
    for i in 1..temp_json_objs.len(){
        out_str = format!("{}, {}", out_str, temp_json_objs[i].to_string());
    }
    out_str = format!("{} ]", out_str);
    console_log!("{}", out_str);
*/}






