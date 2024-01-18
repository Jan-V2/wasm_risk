use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use web_sys::{Document};
use crate::element_getters::{get_document};
use crate::ui::wrap_elem::{ WrapDiv, HTMLable};
use crate::game::Game;
use crate::ui::main::UiState;
use gloo::console::log as console_log;


pub(super) const ALPHABET_LEN: usize = 26;
pub(super) const ASCII_LOWER: [char; ALPHABET_LEN] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];


pub trait StatefullView<T> {
    fn create(doc: &Document) -> Self;
    fn mount(&mut self);
    fn update(&mut self, state: T);
    fn update_self(&mut self);
    fn get(&self) -> T;
    fn hide(&mut self);
    fn show(&mut self);
    fn set_handlers(&mut self, game_ref: &Rc<RefCell<Game>>);
}

#[derive(Clone)]
pub enum SelectedView {
    TurnMenu,
    StartPlace,
    Place,
    Combat,
    DiceRolling,
}

pub struct UiStateManager {
    pub selected: SelectedView,

    pub info_div: WrapDiv,
}

impl UiStateManager {
    pub fn build() -> UiStateManager {
        UiStateManager {
            selected: SelectedView::TurnMenu,
            info_div: WrapDiv::new_from_id(&"info".to_string()),
        }
    }

    pub fn mount(&mut self) {
        self.hide_all();
    }

    pub fn update_all(&mut self) {
    }

    pub fn select_view(&mut self, view: &UiState) {
        self.hide_all();
        console_log!(format!("selecting {:?}", view));
        self.update_all()
    }

    pub fn hide_all(&mut self) {
    }

}


pub(super) fn get_random_id() -> String {
    let mut rand_arry = [0u8; 10];
    web_sys::window().unwrap().crypto().unwrap()
        .get_random_values_with_u8_array(&mut rand_arry).unwrap();
    let ret: String = rand_arry.iter().map(|num| {
        let get_idx = (num / 10) as usize;
        ASCII_LOWER[get_idx]
    }).collect();
    ret
}

pub(super) fn borrow_game_safe(game_ref: &Rc<RefCell<Game>>, name:String, func: fn(RefMut<'_, Game>) ) {
    let borrow = game_ref.try_borrow_mut();
    if borrow.is_err() {
        console_log!(format!("could not borrow in {}", name));
    }else {
        func(borrow.unwrap())
    }
}

