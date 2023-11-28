use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Document;
use crate::game::Game;
use crate::ui::templates::template_start_army_placement;
use crate::ui::ui_state_manager::{get_random_id, StatefullView};
use crate::ui::wrap_elem::{WrapDiv, WrapHtml, HTML_Div, HTMLable};


#[derive(Clone, Default, Debug)]
pub struct StateStartArmyPlacement {
    pub current_player: u32,
    pub num_players: u32,
    pub armies: [u32; 6],
}

pub struct ViewStartArmyPlacement {
    state: StateStartArmyPlacement,
    template: WrapHtml,
    player_label: WrapDiv,
    army_count_label: WrapDiv,
    mounted: bool,
}


impl StatefullView<StateStartArmyPlacement> for ViewStartArmyPlacement {
    fn create(doc: &Document) -> Self {
        let id_player = get_random_id();
        let id_count = get_random_id();
        ViewStartArmyPlacement {
            state: StateStartArmyPlacement::default(),
            template: WrapHtml::new(&doc, "start_army_placement".to_string(),
                                    template_start_army_placement(&id_player, &id_count).as_str()),
            player_label: WrapDiv::new(&doc,
                                       id_player, "unset".to_string()),
            army_count_label: WrapDiv::new(&doc,
                                           id_count, "unset".to_string()),
            mounted: false,
        }
    }

    fn mount(&mut self) {
        if self.mounted {
            panic!("component is already mounted");
        }
        self.mounted = true;
        self.update_self();
        self.template.mount();
        self.army_count_label.mount();
        self.player_label.mount();
    }

    fn update(&mut self, state: StateStartArmyPlacement) {
        self.state = state;
        self.update_self()
    }

    fn update_self(&mut self) {
        self.player_label.set_text(&format!("Player {}", self.state.current_player + 1));
        self.army_count_label.set_text(&format!("{} armies still available.",
                                               self.state.armies[self.state.current_player as usize]));
    }

    fn get(&self) -> StateStartArmyPlacement {
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

