use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Document;
use crate::game::Game;
use crate::ui::templates::template_army_placement;
use crate::ui::ui_state_manager::{get_random_id, StatefullView};
use crate::ui::wrap_elem::{WrapDiv, WrapHtml, HTML_Div, HTMLable};

#[derive(Clone, Default)]
pub struct StateArmyPlacement {
    pub armies: u32,
    pub active_player: u32,
    pub end_turn_placement:bool
}


pub struct ViewArmyPlacement {
    state: StateArmyPlacement,
    template: WrapHtml,
    count_label: WrapDiv,
    mounted: bool,
}


impl StatefullView<StateArmyPlacement> for ViewArmyPlacement {
    fn create(doc: &Document) -> Self {
        let count_id = get_random_id();
        let mut ret = ViewArmyPlacement {
            state: Default::default(),
            template: WrapHtml::new(doc, "army_placement".to_string(),
                                    template_army_placement(&count_id).as_str()),
            count_label: WrapDiv::new(doc,
                                      count_id, "lkmlk".to_string()),
            mounted: false,
        };
        ret.update_self();
        ret
    }

    fn mount(&mut self) {
        if self.mounted {
            panic!("component is already mounted")
        }
        self.mounted = true;
        self.update_self();
        self.template.mount();
        self.count_label.mount();
    }

    fn update(&mut self, state: StateArmyPlacement) {
        self.state = state;
        self.update_self()
    }

    fn update_self(&mut self) {
        self.count_label.set_text(&format!("You still need to Place {} armies",
                                          self.state.armies));
    }

    fn get(&self) -> StateArmyPlacement {
        self.state.clone()
    }

    fn hide(&mut self) {
        self.template.set_visibilty(false);
    }

    fn show(&mut self) {
        self.template.set_visibilty(true);
    }

    fn set_handlers(&mut self, _: &Rc<RefCell<Game>>) {}
}

