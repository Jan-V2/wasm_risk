use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Document;
use crate::game::Game;
use crate::ui::templates::template_turn_menu;
use crate::ui::traits::{HTML_Div, HTMLable};
use crate::ui::ui_state_manager::{borrow_game_safe, get_random_id, StatefullView};
use crate::ui::wrap_elem::{WrapBtn, WrapDiv, WrapHtml};

#[derive(Clone, Default)]
pub struct StateTurn {
    pub active_player: u32,
    pub can_reinforce: bool,
}

pub struct ViewTurn {
    template: WrapHtml,
    label_player: WrapDiv,
    btn_next_turn: WrapBtn,
    state: StateTurn,
    mounted: bool,
}


impl StatefullView<StateTurn> for ViewTurn {
    fn create(doc: &Document) -> Self {
        let id_label = get_random_id();
        let id_btn_next_turn = get_random_id();
        let template = WrapHtml::new(doc, "turn_start".to_string(), template_turn_menu(
            &id_label, &id_btn_next_turn,
        ).as_str());
        template.mount();
        let mut ret = ViewTurn { template, label_player: WrapDiv::new_from_id(&id_label), btn_next_turn: WrapBtn::new_from_id(&id_btn_next_turn), state: StateTurn {
                active_player: 0,
                can_reinforce: true,
            }, mounted: false, };

        ret.update_self();
        ret
    }

    fn mount(&mut self) {
        if self.mounted {
            panic!("component is already mounted")
        }
        self.mounted = true;
        self.update_self();
    }

    fn update(&mut self, state: StateTurn) {
        self.state = state;
    }

    fn update_self(&mut self) {
        self.label_player.set_text(format!("Player {}", self.state.active_player + 1));
    }

    fn get(&self) -> StateTurn {
        self.state.clone()
    }

    fn hide(&mut self) {
        self.template.set_visibilty(false);
    }

    fn show(&mut self) {
        self.template.set_visibilty(true);
    }

    fn set_handlers(&mut self, game_ref: &Rc<RefCell<Game>>) {
        let ref_next_turn = game_ref.clone();
        self.btn_next_turn.set_click_handler(Box::from(move |_| {
            borrow_game_safe(&ref_next_turn, "attack btn".to_string(),
                             |mut g| {g.handle_ui_end_turn()});
        }));
    }
}

