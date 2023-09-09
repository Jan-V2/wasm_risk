use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Element, HtmlButtonElement, HtmlCanvasElement, HtmlImageElement, HtmlLabelElement, ImageData, MouseEvent};


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
    let canvas = get_canvas();
    let context = get_drawing_context(&canvas);

    let mut state = true;

    process_state(&state, &context, &canvas );

    let closure = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        state = !state;
        let canvas2 = get_canvas();
        let context2 = get_drawing_context(&canvas2);
        process_state(&state, &context2, &canvas2);
    });
    let _ = canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
    closure.forget();

    let closure2 = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
        let label = get_html_label_by_id("xy_coord_label");
        label.set_inner_text(&format!("canvas coord x:{} y:{}", event.offset_x(), event.offset_y()));
    });
    let _ = canvas.add_event_listener_with_callback("mousemove", closure2.as_ref().unchecked_ref());
    closure2.forget();


    let button = get_button_by_id("nuke_btn");
    let closure_btn = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        let canvas2 = get_canvas();
        let context2 = get_drawing_context(&canvas2);
        let img = context2.get_image_data(0f64, 0f64
                                          , canvas2.width() as f64, canvas2.height() as f64).unwrap();
        for div in 1..5{
            for px_count in 1 ..5{
                let pixel_min = px_count * 50;
                let div_max = div * 25;
                console_log!("div min {}, px min {}", div_max, pixel_min);
                _ = color_counter(&img, div_max, pixel_min, false);
            }
        }

    });
    let _ = button.add_event_listener_with_callback("click", closure_btn.as_ref().unchecked_ref());
    closure_btn.forget();
}

fn color_counter(img:&ImageData, max_color_div:i32, min_px_per_color:u32, verbose_logging:bool ) -> i32{
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


fn get_html_label_by_id(id :&str) -> HtmlLabelElement{
    return get_element_by_id(id).dyn_into::<HtmlLabelElement>().map_err(|_| ())
        .unwrap();
}

fn get_button_by_id(id :&str) -> HtmlButtonElement{
    return get_element_by_id(id).dyn_into::<HtmlButtonElement>().map_err(|_| ())
        .unwrap();
}

fn get_element_by_id(id :&str) -> Element{
    let document = web_sys::window().unwrap().document().unwrap();
    return document.get_element_by_id(id).unwrap();
}


fn get_canvas() -> HtmlCanvasElement{
    let canvas = get_element_by_id("canvas");
    return canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
}


fn get_drawing_context(canvas :&HtmlCanvasElement) -> CanvasRenderingContext2d{
    return canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
}

fn process_state(state :&bool, context :&CanvasRenderingContext2d, canvas :&HtmlCanvasElement){
    console_log!("state is {}", state);
    if *state{
        context.rect(0f64, 0f64, canvas.width() as f64, canvas.height() as f64);
        context.set_fill_style(&JsValue::from_str("LightCyan"));
        context.fill();
        let image = get_element_by_id("board_2").dyn_into::<HtmlImageElement>()
            .map_err(|_| ()).unwrap();
        let _ = context.draw_image_with_html_image_element_and_dw_and_dh(&image, 0f64, 0f64, canvas.width() as f64, canvas.height() as f64);
        //context.rect(0f64, 0f64, canvas.width() as f64, canvas.height() as f64);
        //context.set_fill_style(&JsValue::from_str("red"));
        //context.fill();
    }else {
        context.rect(0f64, 0f64, canvas.width() as f64, canvas.height() as f64);
        context.set_fill_style(&JsValue::from_str("LightCyan"));
        context.fill();
    }
}


