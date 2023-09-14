use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageData, MouseEvent};
use crate::element_getters::*;


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
        console_log!("color = {}, {}, {}", img_data[array_idx as usize], img_data[array_idx as usize + 1]
            , img_data[array_idx as usize + 2] );

    });
    let _ = canvas.add_event_listener_with_callback("click", px_color_clos.as_ref().unchecked_ref());
    px_color_clos.forget();
}


pub fn ui_init_canvas_test_btn(){
    let button = get_button_by_id("nuke_btn");
    let closure_btn = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        console_log!("removing pixels");

        let canvas2 = get_canvas();
        let context2 = get_drawing_context(&canvas2);
        let img = context2.get_image_data(0f64, 0f64
                                          , canvas2.width() as f64, canvas2.height() as f64).unwrap();
        let mut img_data = img.data();

        let max_color_value = get_html_input_by_id("range").value().parse::<i32>().unwrap();
        let mut px_removed  = 0;
        for px_x in 0..canvas2.width(){
            for px_y in 0..canvas2.height(){
                let idx = (px_y * canvas2.width() as u32 + px_x) * 4;
                let mut remove = false;
                for i in 0..3{
                    if img_data[idx as usize + i] > max_color_value as u8{
                        remove = true;
                        break;
                    }
                }
                if remove{
                    px_removed += 1;
                    for i in 0..3 {
                        img_data[idx as usize + i] = 255;
                    }
                }
            }
        }
        let img_data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(img_data.as_slice()),
                                                                   canvas2.width(), canvas2.height()).unwrap();
        _ = context2.put_image_data(&img_data,0.0, 0.0 );
        console_log!("removed {px_removed} pixels?");
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

/*
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
}*/

