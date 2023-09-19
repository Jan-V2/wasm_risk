use js_sys::Math::sqrt;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, MouseEvent};
use crate::element_getters::*;
extern crate queues;

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

pub fn ui_init_canvas(max_color_div:i32, prov_points:Vec<[i32; 2]>){
    // inits canvas click handeler
    let canvas = get_canvas();
    let context = get_drawing_context(&canvas);
    let mut coord_array:Vec<[i32; 2]> = Vec::new();
    let canvas_width = canvas.width() as i32;
    let canvas_height = canvas.height() as i32;
    draw_board(&canvas, &context);

    let closure2 = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
        let label = get_html_label_by_id("xy_coord_label");
        label.set_inner_text(&format!("canvas coord x:{} y:{}", event.offset_x(), event.offset_y()));

    });
    let _ = canvas.add_event_listener_with_callback("mousemove", closure2.as_ref().unchecked_ref());
    closure2.forget();

    let px_color_clos = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        let canvas2 = get_canvas();
        let context2 = get_drawing_context(&canvas2);
        let img_data = context2.get_image_data(0f64, 0f64
                                               , canvas2.width() as f64, canvas2.height() as f64).unwrap().data();
        let _canvas = get_canvas();
        let clicked_coord = [ _event.x() - _canvas.offset_left(), _event.y() - _canvas.offset_top()];
        let array_idx = (clicked_coord[1] *  canvas2.width() as i32 + clicked_coord[0]) * 4;


        console_log!("coord xy: {}, {} color = {}, {}, {}", clicked_coord[0], clicked_coord[1], img_data[array_idx as usize],
            img_data[array_idx as usize + 1], img_data[array_idx as usize + 2] );

        coord_array.push(clicked_coord);
        let mut str_out:String = "[".to_string();
        for i in 0..coord_array.len(){
            str_out = format!("{}[{}, {}],", str_out, coord_array[i][0], coord_array[i][1]);
        }
        console_log!("{}]", str_out);


        fn get_coord (coord: [i32; 2], width: i32, img_data:&Clamped<Vec<u8>>) -> [u8; 3] {
            let idx = (coord[0] + coord[1] * width) * 4;
            return [img_data[idx as usize], img_data[(idx+1)as usize], img_data[(idx+2) as usize]];
        }

        fn compare_colors(target:[i32; 2], compare:[i32; 2], max_div:i32, width:i32,
                          img_data:&Clamped<Vec<u8>>) -> bool{
            let mut color_div_acc = 0;
            let color_target = get_coord(target, width, img_data);
            let color_compare = get_coord(compare, width, img_data);
            for i in 0..color_target.len(){
                color_div_acc += (color_target[i] as i32 - color_compare[i] as i32).abs();
            }
            return color_div_acc < max_div;
        }

        fn dist_between_pnts(pnt1:&[i32; 2], pnt2:&[i32; 2]) -> i32{
            sqrt((pnt1[0] - pnt2[0]).pow(2) as f64 +
                (pnt1[1] - pnt2[1]).pow(2) as f64) as i32
        }

        let mut found_at_idx:Vec<i32> = Vec::new();

        for i in 0..prov_points.len(){
            if compare_colors(prov_points[i], clicked_coord, max_color_div,
                              canvas_width, &img_data) {
                found_at_idx.push(i as i32);
            }
        }

        if found_at_idx.len() == 0{
            console_log!("could not find color in in the array")

        }else if found_at_idx.len() == 1 {
            console_log!("found color at idx {} 1 idx found", found_at_idx[0]);
        }else {
            let idxes_found = found_at_idx.len();
            let mut idx_shortest:i32 = -1;
            let mut shortest_dist = i32::MAX;
            for idx in found_at_idx{
                let dist = dist_between_pnts(&clicked_coord, &prov_points[idx as usize]);
                if dist < shortest_dist{
                    shortest_dist = dist;
                    idx_shortest = idx;
                }
            }
            console_log!("found color at idx {} {} idxes found", idx_shortest, idxes_found);

        }

    });
    let _ = canvas.add_event_listener_with_callback("click", px_color_clos.as_ref().unchecked_ref());
    px_color_clos.forget();
}


pub fn ui_init_canvas_test_btn(){
    // inits click for test button

    /*let button = get_button_by_id("nuke_btn");
    let closure_btn = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        console_log!("removing pixels");

        let canvas = get_canvas();
        let context = get_drawing_context(&canvas);
        let img = context.get_image_data(0f64, 0f64
                                         , canvas.width() as f64, canvas.height() as f64).unwrap();
        let img_data = img.data();


        fn get_nearby_coords(point:[i32; 2]) -> Vec<[i32; 2]> {
            let mut ret:Vec<[i32; 2]> = Vec::new();
            ret.push([point[0] + 1, point[1]]);
            ret.push([point[0] - 1, point[1]]);
            ret.push([point[0], point[1] + 1]);
            ret.push([point[0], point[1] - 1]);
            return ret;
        }

        console_log!("painting result");
        context.rect(0f64, 0f64, canvas.width() as f64, canvas.height() as f64);
        context.set_fill_style(&JsValue::from_str("LightCyan"));
        context.fill();

        let img = context.get_image_data(0f64, 0f64
                                             , canvas.width() as f64, canvas.height() as f64).unwrap();
        let mut img_data = img.data();


/*        for coord in prov_vec{
            let idx = ((coord[1] * canvas.width() as i32 + coord[0]) * 4) as usize;
            img_data[idx] = base_color[0];
            img_data[idx + 1] = base_color[1];
            img_data[idx + 2] = base_color[2];
        }
*/

/*        for coord in edge_provs{
            let idx = ((coord[1] * canvas.width() as i32 + coord[0]) * 4) as usize;
            img_data[idx] = 0;
            img_data[idx + 1] = 0;
            img_data[idx + 2] = 0;
        }*/

        let img_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(img_data.as_slice()),
                                                                   canvas.width(), canvas.height()).unwrap();
        _ = context.put_image_data(&img_data, 0.0, 0.0 );
        console_log!("removed pixels");
    });
    let _ = button.add_event_listener_with_callback("click", closure_btn.as_ref().unchecked_ref());
    closure_btn.forget();*/
}


pub fn draw_board(canvas:&HtmlCanvasElement, context: &CanvasRenderingContext2d){

    context.rect(0f64, 0f64, canvas.width() as f64, canvas.height() as f64);
    context.set_fill_style(&JsValue::from_str("LightCyan"));
    context.fill();
    let image = get_element_by_id("board_2").dyn_into::<HtmlImageElement>()
        .map_err(|_| ()).unwrap();
    let _ = context.draw_image_with_html_image_element_and_dw_and_dh(&image, 0f64, 0f64, canvas.width() as f64, canvas.height() as f64);
}

