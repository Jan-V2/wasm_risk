use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Element, HtmlCanvasElement, HtmlImageElement, HtmlLabelElement, MouseEvent};

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
        let label = get_element_by_id("xy_coord_label").dyn_into::<HtmlLabelElement>().map_err(|_| ())
            .unwrap();
        label.set_inner_text(&format!("canvas coord x:{} y:{}", event.offset_x(), event.offset_y()));
    });
    let _ = canvas.add_event_listener_with_callback("mousemove", closure2.as_ref().unchecked_ref());
    closure2.forget()
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
        test_pattern(&context);
    }else {
        context.rect(0f64, 0f64, canvas.width() as f64, canvas.height() as f64);
        context.set_fill_style(&JsValue::from_str("LightCyan"));
        context.fill();
        test_pattern(&context);
    }
}


fn test_pattern(context: &CanvasRenderingContext2d){
    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}