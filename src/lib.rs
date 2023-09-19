mod element_getters;
mod canvas;
mod ui;

use wasm_bindgen::prelude::*;
use crate::canvas::{ui_init_canvas, ui_init_canvas_test_btn};
use crate::ui::ui_init_max_color_slider;

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
    ui_init_canvas();
    ui_init_max_color_slider();
    ui_init_canvas_test_btn(prov_coords[0], 100);

}






