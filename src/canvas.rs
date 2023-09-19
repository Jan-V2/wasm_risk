use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData, MouseEvent};
use crate::element_getters::*;
extern crate queues;
use queues::*;

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

pub fn ui_init_canvas(){
    // inits canvas click handeler
    let canvas = get_canvas();
    let context = get_drawing_context(&canvas);

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
        let array_idx = (_event.y() *  canvas2.width() as i32 + _event.offset_x()) * 4;
        console_log!("coord xy: {}, {} color = {}, {}, {}", _event.x(), _event.y(), img_data[array_idx as usize],
            img_data[array_idx as usize + 1], img_data[array_idx as usize + 2] );
    });
    let _ = canvas.add_event_listener_with_callback("click", px_color_clos.as_ref().unchecked_ref());
    px_color_clos.forget();
}


pub fn ui_init_canvas_test_btn(start_point:[i32; 2], max_color_div:i32){
    // inits click for test button
    let button = get_button_by_id("nuke_btn");
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

        fn get_coord (coord: [i32; 2], width: i32, img_data:Clamped<Vec<u8>>) -> [u8; 3] {
            let idx = (coord[0] + coord[1] * width) * 4;
            return [img_data[idx as usize], img_data[(idx+1)as usize], img_data[(idx+2) as usize]];
        };

        fn compare_colors(target:[i32; 2], compare:[i32; 2], max_div:i32) -> bool{
            let color_div_acc = 0;
            let color_target = get_coord(target);
            let
        }

        let base_color = get_coord(start_point, canvas.width() as i32);
        let mut prov_vec:Vec<[i32; 2]> = Vec::new();
        let mut searched:Vec<[i32; 2]> = Vec::new();
        let mut search_q: Queue<[i32; 2]> = Queue::new();
        _  = search_q.add(start_point.clone());

        let mut loop_color;
        let mut loop_color_div_acc;

        while search_q.size() > 0{
            let coord = search_q.remove().unwrap();
            if !searched.contains(&coord){
                searched.push(coord);
                loop_color_div_acc = 0;
                loop_color = get_coord(coord, canvas.width() as i32);
                for i in 0..loop_color.len(){
                    loop_color_div_acc += (loop_color[i] as i32 - base_color[i] as i32).abs();
                }
                if loop_color_div_acc < max_color_div{
                    prov_vec.push(coord);
                    for c in get_nearby_coords(coord){
                        if !searched.contains(&c){
                            _ = search_q.add(c);
                        }
                    }

                }
            }
        }
        drop(search_q);
        drop(searched);
        console_log!("found {} pixels", prov_vec.len());

        console_log!("looking for edges");
        let mut edge_provs:Vec<[i32; 2]> = Vec::new();
        for coord in &prov_vec{
            if !(prov_vec.contains(&[coord[0] + 1, coord[1]]) && prov_vec.contains(&[coord[0], coord[1] + 1]) &&
                prov_vec.contains(&[coord[0] - 1, coord[1]]) && prov_vec.contains(&[coord[0], coord[1] - 1])){
                edge_provs.push(*coord);
            }
        }
        drop(prov_vec);

        console_log!("sorting edges");
        let mut count = 0;
        let mut edge_provs_sorted:Vec<[i32; 2]> = Vec::new();
        let edge_len = edge_provs.len();
        edge_provs_sorted.push(edge_provs.remove(0));


        while count < edge_len{
            let head_coord = edge_provs_sorted[edge_provs_sorted.len()-1];

            let mut check_coord = |coord| -> bool {
                if edge_provs.contains(coord){
                    let idx = edge_provs.iter().position(|c| {
                        return c[0] == coord[0] && c[1] == coord[1];
                    }).unwrap();
                    edge_provs_sorted.push(edge_provs.remove( idx));
                    return true;
                }
                return false;
            };

            let mut nearby_vec:Vec<[i32; 2]> = Vec::new();
            nearby_vec.push([head_coord[0] + 1, head_coord[1]]);
            nearby_vec.push([head_coord[0] - 1, head_coord[1]]);
            nearby_vec.push([head_coord[0], head_coord[1] + 1]);
            nearby_vec.push([head_coord[0], head_coord[1] - 1]);


            let mut found_neighbour = false;
            for i in 0..nearby_vec.len(){
                if check_coord(&nearby_vec[i]){
                    found_neighbour = true;
                    break;
                }

            }

            if !found_neighbour {
                console_log!("could not make continues ring out of edge");
                break;
            }

            count += 1;

        }
        console_log!("count is {}", count);

        /*
        todo put edges in order
        todo generate type of edge per square
        todo generate list of lines
        todo implement distance from line to point
        todo simplify polygon
        todo write collision logic
        */


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
    closure_btn.forget();
}


pub fn draw_board(canvas:&HtmlCanvasElement, context: &CanvasRenderingContext2d){

    context.rect(0f64, 0f64, canvas.width() as f64, canvas.height() as f64);
    context.set_fill_style(&JsValue::from_str("LightCyan"));
    context.fill();
    let image = get_element_by_id("board_2").dyn_into::<HtmlImageElement>()
        .map_err(|_| ()).unwrap();
    let _ = context.draw_image_with_html_image_element_and_dw_and_dh(&image, 0f64, 0f64, canvas.width() as f64, canvas.height() as f64);
}


pub fn color_counter(img:&ImageData, max_color_div:i32, min_px_per_color:u32, verbose_logging:bool ) -> i32{
    let mut colors:Vec<[u8; 3]> = Vec::new();
    let mut px_per_color:Vec<u32> = Vec::new();
    let img_data = img.data();

    // 43 prov
    // 11 double
    // excpecting 32 + 2 black + empty colors

    // ignores the a in rgba, cuz don't need it
    for i  in 0..img_data.len()/4{
        let idx = i * 4;
        let px_color = [img_data[idx], img_data[idx+1], img_data[idx+2]];
        if colors.len() == 0{
            colors.push(px_color);
            px_per_color.push(1);
        }else {
            let mut in_colors = false;
            for c in 0..colors.len() {
                let color = colors[c];
                let mut color_div = 0;
                for j in 0..color.len(){
                    color_div += (color[j] as i32 - px_color[j] as i32).abs();
                }
                if color_div < max_color_div {
                    in_colors = true;
                    px_per_color[c] += 1;
                    break;
                }
            }
            if !in_colors{
                colors.push(px_color);
                px_per_color.push(1);
            }
        }
        if verbose_logging{
            if i as u32 * 4  % img.width() == 0{ // every line
                if i as u32 / img.width() % 10 == 0{
                    console_log!("at line {}", i as u32 / img.width())
                }
            }
        }
    }
    if verbose_logging{
        console_log!("printing colors...");
    }
    let color_len =  colors.len();
    let mut important_color_count = 0;
    for c in 0..colors.len(){
        if px_per_color[c] > min_px_per_color{
            if verbose_logging{
                console_log!("{} {} {} px: {}", colors[c][0], colors[c][1], colors[c][2], px_per_color[c]);
            }
            important_color_count += 1;
        }
    }
    console_log!("found {} colors total", color_len);
    console_log!("found {} important colors", important_color_count);
    return important_color_count;
}

