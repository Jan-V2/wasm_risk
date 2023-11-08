use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, ImageBitmap, ImageData, MouseEvent};
use crate::element_getters::*;
use crate::game::{Game, ProvLookupTable};
use crate::model::{Coord, Model, Province};

extern crate queues;


pub fn get_map_lookup_data(max_div: u32) -> ProvLookupTable {
    let canvas = get_canvas("canvas");
    let context = get_drawing_context(&canvas);
    draw_board_raw(&canvas, &context);

    let img_data = context.get_image_data(0f64, 0f64
                                          , canvas.width() as f64, canvas.height() as f64).unwrap().data();
    let mut ret: Vec<[u8; 3]> = Vec::new();
    for i in (0..img_data.len()).step_by(4) {
        ret.push([img_data[i], img_data[i + 1], img_data[i + 2]]);
    }
    return ProvLookupTable {
        pixels: ret,
        width: canvas.width(),
        max_div,
    };
}

pub fn ui_init_canvas(game_model: Rc<RefCell<Game>>) {
    // inits canvas click handeler
    let canvas = get_canvas("canvas");
    let game_model_clone = game_model.clone();

    let canvas_xy_mouseover_handeler = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
        let label = get_html_label_by_id("xy_coord_label");
        label.set_inner_text(&format!("canvas coord x:{} y:{}", event.offset_x(), event.offset_y()));

        let mut text:Option<String> = None;
        let game_borrow = game_model_clone.as_ref().try_borrow();
        if game_borrow.is_ok(){
            let game_unwrap = game_borrow.unwrap();
            text = game_unwrap.get_prov_mouseover_string(&Coord{
                x: event.offset_x(),
                y: event.offset_y(),
            });
            game_unwrap.draw_board();
        }



        if text.is_some(){
            let canvas = get_canvas("canvas");
            let ct = get_drawing_context(&canvas);

            let point_size = 13f64;
            let font_str = format!("bold {}px serif", (point_size ) as i32);
            ct.set_fill_style(&JsValue::from_str("black"));
            ct.set_font(&font_str);
            ct.set_text_align("right");

            let _ = ct.fill_text(text.unwrap().as_str(), event.offset_x() as f64, event.offset_y() as f64);
        }


    });
    let _ = canvas.add_event_listener_with_callback("mousemove", canvas_xy_mouseover_handeler.as_ref().unchecked_ref());
    canvas_xy_mouseover_handeler.forget();

    let canvas_click_handler = Closure::<dyn FnMut(_)>::new(move |_event: MouseEvent| {
        let _canvas = get_canvas("canvas");
        let clicked_coord = [_event.x() - _canvas.offset_left(), _event.y() - _canvas.offset_top()];

        let ret_coord = Coord {
            x: clicked_coord[0],
            y: clicked_coord[1],
        };

        game_model.as_ref().borrow_mut().handle_canvas_click(ret_coord);
    });
    let _ = canvas.add_event_listener_with_callback("click", canvas_click_handler.as_ref().unchecked_ref());
    canvas_click_handler.forget();
}


pub fn redraw_board_state(model: &Model, scale: f64, draw_flags:bool){
    draw_board();
    if draw_flags{
        draw_all_flags(&model, scale);
        draw_all_army_count(&model.provinces, scale);
    }
}


fn draw_all_flags(model: &Model, scale: f64) {
    let canvas = get_canvas("canvas");
    let ct = get_drawing_context(&canvas);

    for prov in &model.provinces {
        if prov.owner_id != 100{
            draw_flag_raw(&prov, &model.players[prov.owner_id as usize].color, scale, &ct);
        }else {
            draw_flag_raw(&prov, &"blue".to_string(), scale, &ct);
        }
    }
}


fn draw_flag_raw(prov: &Province, color:&String, scale: f64, ct: &CanvasRenderingContext2d) {
    let bot_w = 30f64 * scale;
    let staff_height = 35f64 * scale;
    let flag_height = 20f64 * scale;
    let flag_width = 20f64 * scale;
    let half_bot: f64 = bot_w / 2f64;

    let origin = [prov.location.x as f64, prov.location.y as f64];
    ct.set_line_width(2f64);
    ct.set_fill_style(&JsValue::from_str(color));
    ct.begin_path();
    ct.move_to(origin[0] - half_bot, origin[1]);
    ct.line_to(origin[0] + half_bot, origin[1]);
    ct.move_to(origin[0], origin[1]);
    let top_staff = [origin[0], origin[1] - staff_height];
    ct.line_to(top_staff[0], top_staff[1]);
    ct.line_to(top_staff[0] + flag_width, top_staff[1] + flag_height / 2f64);
    ct.line_to(top_staff[0], top_staff[1] + flag_height);
    ct.stroke();
    ct.fill();
}


fn draw_all_army_count(provs: &Vec<Province>, scale: f64) {
    // this is the only method, because redrawing the army count requires screen clear.
    // text js fiddle https://jsfiddle.net/xcp370k1/

    let canvas = get_canvas("canvas");
    let ct = get_drawing_context(&canvas);

    let padding = 3f64;
    let point_size = 22f64;
    let font_str = format!("{}px serif", (point_size * scale) as i32);
    ct.set_fill_style(&JsValue::from_str("black"));
    ct.set_font(&font_str);
    ct.set_text_align("right");

    let draw_count = |prov: &Province| {
        let origin = [prov.location.x as f64, prov.location.y as f64];
        let _ = ct.fill_text(prov.army_count.to_string().as_str(), origin[0] - padding, origin[1] - padding);
    };

    for prov in provs {
        draw_count(&prov);
    }
}


pub fn draw_board() {
    let canvas = get_canvas("canvas");
    draw_board_raw(&canvas, &get_drawing_context(&canvas));
}

fn draw_board_raw(canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d) {
    context.rect(0f64, 0f64, canvas.width() as f64, canvas.height() as f64);
    context.set_fill_style(&JsValue::from_str("LightCyan"));
    context.fill();
    let image = get_element_by_id("board_2").dyn_into::<HtmlImageElement>()
        .map_err(|_| ()).unwrap();

    let _ = context.draw_image_with_html_image_element_and_dw_and_dh(&image, 0f64, 0f64,
                                                                     canvas.width() as f64, canvas.height() as f64);
}

pub struct DiceFaceTex{
    width:u32,
    height:u32,
    img_data:Clamped<Vec<u8>>,
    face_number:u32,
    bitmap:Option<ImageBitmap>,
}

pub fn draw_dice( context:CanvasRenderingContext2d, dice:&DiceFaceTex, location:Coord, size:u32 ){
    let _ = context.draw_image_with_image_bitmap_and_dw_and_dh(dice.bitmap.as_ref().unwrap(),
                                                                location.x as f64, location.y as f64,
                                                               size as f64, size as f64);
}


pub fn get_dice_tex() -> Rc<RefCell<Vec<DiceFaceTex>>>{
    let canvas = get_canvas("canvas");
    let context = get_drawing_context(&canvas);

    let image = get_element_by_id("dice").dyn_into::<HtmlImageElement>()
        .map_err(|_| ()).unwrap();

    let _ = context.draw_image_with_html_image_element_and_dw_and_dh(&image, 0f64, 0f64,
                                                                     image.width() as f64, image.height() as f64);
    let img_data = context.get_image_data(0f64, 0f64
                                          , image.width() as f64, image.height() as f64).unwrap().data();
    draw_board();
    let mut ret:Vec<DiceFaceTex> = vec![];
    let texes_along_width = 3u32;
    let texes_along_height = 2u32;
    let face_height = image.height() / texes_along_height;
    let face_width = image.width() / texes_along_width;

    for face_row in 0..texes_along_width {
        for face_col in 0..texes_along_height {
            let mut new_face = DiceFaceTex{
                width: face_width,
                height: face_height,
                img_data: Clamped(Vec::new()),
                face_number: (face_row + face_col * texes_along_width) + 1,
                bitmap: None,
            };

            let start_y = face_height * face_col;

            for idx_y in start_y..start_y + face_height{
                let start_idx = (idx_y * image.width() + face_row * face_width) * 4;

                new_face.img_data.append(&mut img_data[
                    start_idx as usize..(start_idx + face_width *4) as usize
                    ].to_vec());
            }
            ret.push(new_face);
        }
    }

    draw_board();

    let rc2:Rc<RefCell<Vec<DiceFaceTex>>> = Rc::from(RefCell::from(ret));
    let rc = rc2.clone();
    spawn_local(async move {
        for dice in &mut *rc.borrow_mut(){
            let test = ImageData::new_with_u8_clamped_array(
                Clamped(dice.img_data.0.as_slice()), dice.width).unwrap();
            let tex_future = JsFuture::from(
                web_sys::window().unwrap().create_image_bitmap_with_image_data(&test).unwrap());
            let tex = tex_future.await.unwrap().dyn_into::<ImageBitmap>().unwrap();
            dice.bitmap = Some(tex);
        }
    });

    rc2
}
