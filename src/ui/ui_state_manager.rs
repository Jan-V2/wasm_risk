use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use web_sys::{Document};
use crate::element_getters::{get_document};
use crate::ui::wrap_elem::{ WrapDiv, HTMLable};
use crate::game::Game;
use crate::ui::main::UiState;
use crate::ui::view_army_place::ViewArmyPlacement;
use crate::ui::view_combat::ViewCombat;
use crate::ui::view_dice_roll::ViewDiceRoll;
use crate::ui::view_start_army_place::ViewStartArmyPlacement;
use gloo::console::log as console_log;
use crate::ui::view_label::ViewLabel;
use crate::ui::view_turn::ViewTurn;


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
    pub turn_menu: ViewTurn,
    pub start_army_placement: ViewStartArmyPlacement,
    pub army_placement: ViewArmyPlacement,
    pub selected: SelectedView,
    pub combat: ViewCombat,
    pub dice_rolls: ViewDiceRoll,
    pub view_label:ViewLabel,
    pub info_div: WrapDiv,
}

impl UiStateManager {
    pub fn build() -> UiStateManager {
        let doc = get_document();
        UiStateManager {
            turn_menu: ViewTurn::create(&doc),
            start_army_placement: ViewStartArmyPlacement::create(&doc),
            army_placement: ViewArmyPlacement::create(&doc),
            selected: SelectedView::TurnMenu,
            combat: ViewCombat::create(&doc),
            dice_rolls: ViewDiceRoll::create(&doc),
            view_label: ViewLabel::create(&doc),
            info_div: WrapDiv::new_from_id(&"info".to_string()),
        }
    }

    pub fn mount(&mut self) {
        self.turn_menu.mount();
        self.start_army_placement.mount();
        self.army_placement.mount();
        self.combat.mount();
        self.dice_rolls.mount();
        self.view_label.mount();
        self.hide_all();
    }

    pub fn update_all(&mut self) {
        self.turn_menu.update_self();
        self.start_army_placement.update_self();
        self.army_placement.update_self();
        self.combat.update_self();
        self.dice_rolls.update_self();
        self.view_label.mount();
    }

    pub fn select_view(&mut self, view: &UiState) {
        self.hide_all();
        console_log!(format!("selecting {:?}", view));
        match view {
            UiState::ARMY_PLACEMENT_START => { self.start_army_placement.show() }
            UiState::ARMY_PLACEMENT => { self.army_placement.show() }
            UiState::TURN => { self.turn_menu.show() }
            UiState::COMBAT => { self.combat.show() }
            UiState::DICE_ROLL => { self.dice_rolls.show() }
            UiState::GAME_END => { todo!() }
            UiState::CARD_SELECT => { todo!() }
            UiState::SETUP => {}// don't show a view
            UiState::MOVE => {todo!()}
            UiState::LABEL => {self.view_label.show();}
        }
        self.update_all()
    }

    pub fn hide_all(&mut self) {
        self.turn_menu.hide();
        self.start_army_placement.hide();
        self.army_placement.hide();
        self.combat.hide();
        self.dice_rolls.hide();
        self.view_label.hide();
    }

    pub fn set_handlers(&mut self, game_ref: &Rc<RefCell<Game>>) {
        self.turn_menu.set_handlers(game_ref);
        self.start_army_placement.set_handlers(game_ref);
        self.army_placement.set_handlers(game_ref);
        self.combat.set_handlers(game_ref);
        self.dice_rolls.set_handlers(game_ref);
        self.view_label.set_handlers(game_ref);
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

