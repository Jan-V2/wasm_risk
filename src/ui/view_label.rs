use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{Document};
use crate::game::Game;
use crate::ui::main::UiState;
use crate::ui::templates::template_label;
use crate::ui::ui_state_manager::{borrow_game_safe, get_random_id, StatefullView};
use crate::ui::wrap_elem::{HTML_Div, HTMLable, WrapBtn, WrapDiv, WrapHtml};


//todo make this a temp viewz
#[derive(Clone, Default)]
pub struct StateLabel {
    pub label_text:String,
    pub return_state:UiState
}

pub struct ViewLabel{
    template:WrapHtml,
    label_div : WrapDiv,
    next_btn:WrapBtn,
    state:StateLabel,
}

impl StatefullView<StateLabel> for ViewLabel{
    fn create(doc: &Document) -> Self {
        let id_label = get_random_id();
        let id_btn = get_random_id();
        let template = WrapHtml::new(doc, "label".to_string(), template_label(
            &id_label, &id_btn
        ).as_str());
        template.mount();
        ViewLabel{
            template,
            label_div: WrapDiv::new_from_id(&id_label),
            next_btn: WrapBtn::new_from_id(&id_btn),
            state: Default::default(),
        }
    }

    fn mount(&mut self) {
        self.template.set_visibilty(false);
    }

    fn update(&mut self, state: StateLabel) {
        self.state = state;
        self.update_self()
    }

    fn update_self(&mut self) {
        self.label_div.set_text(&self.state.label_text);
    }

    fn get(&self) -> StateLabel {
        return self.state.clone();
    }

    fn hide(&mut self) {
        self.template.set_visibilty(false);
    }

    fn show(&mut self) {
        self.template.set_visibilty(true);
    }

    fn set_handlers(&mut self, game_ref: &Rc<RefCell<Game>>) {
        let ref_next = game_ref.clone();
        self.next_btn.set_click_handler(Box::from(move |_| {
            borrow_game_safe(&ref_next,"label nexe btn".to_string(),
                             |mut g| g.handle_label_next())
        }));

    }
}