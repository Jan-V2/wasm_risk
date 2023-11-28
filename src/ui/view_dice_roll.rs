use std::cell::RefCell;
use std::rc::Rc;
use crate::model::CombatResult;
use crate::ui::ui_state_manager::{borrow_game_safe, get_random_id, StatefullView};
use gloo::console::log as console_log;
use web_sys::Document;
use crate::canvas::{DiceFaceTex, get_dice_tex};
use crate::game::Game;
use crate::ui::templates::template_dice_roll;
use crate::ui::wrap_elem::{WrapBtn, WrapDiceCanvas, WrapHtml, HTML_Div, HTMLable};


pub struct ViewDiceRoll {
    state: CombatResult,
    template: WrapHtml,
    next_btn: WrapBtn,
    canvas_top: WrapDiceCanvas,
    canvas_bot: WrapDiceCanvas,
    dice_face_texes: Rc<RefCell<Vec<DiceFaceTex>>>,
}

impl StatefullView<CombatResult> for ViewDiceRoll {
    fn create(doc: &Document) -> Self {
        let id_canvases = (get_random_id(), get_random_id());
        let id_next_btn = get_random_id();
        let template = WrapHtml::new(doc, "dice_roll".to_string(),
                                     template_dice_roll(&id_canvases, &id_next_btn).as_str());
        template.mount();
        ViewDiceRoll {
            state: CombatResult::default(),
            template,
            next_btn: WrapBtn::new_from_id(&id_next_btn),
            canvas_top: WrapDiceCanvas::new_from_id(&id_canvases.0),
            canvas_bot: WrapDiceCanvas::new_from_id(&id_canvases.1),
            dice_face_texes: get_dice_tex(),
        }
    }

    fn mount(&mut self) {
        self.template.set_visibilty(false);
    }

    fn update(&mut self, state: CombatResult) {
        self.state = state;
        self.update_self()
    }

    fn update_self(&mut self) {
        console_log!("updateing dice view");
        console_log!(format!("{:?}", self.state));

        self.canvas_top.clear_canvas();
        self.canvas_bot.clear_canvas();
        self.canvas_top.draw_dice_rolls(&self.state.dice_roll_attacker,
                                        self.dice_face_texes.clone());
        self.canvas_bot.draw_dice_rolls(&self.state.dice_roll_defender,
                                        self.dice_face_texes.clone())

    }

    fn get(&self) -> CombatResult {
        return self.state.clone();
    }

    fn hide(&mut self) {
        self.template.set_visibilty(false);
    }

    fn show(&mut self) {
        self.template.set_visibilty(true);
    }

    fn set_handlers(&mut self, game_ref: &Rc<RefCell<Game>>) {
        let game = game_ref.clone();
        self.next_btn.set_click_handler(Box::from(move |_|{
            borrow_game_safe(&game, "dice next btn".to_string(),
                             |mut g| g.handle_ui_dice_next())
        }))
    }
}

