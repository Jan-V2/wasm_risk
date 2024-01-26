use std::cell::RefCell;
use std::rc::Rc;
use marble::wrap::{*};
use marble::traits::{*};
use crate::views::turn::{create_view_turn, ViewTurn};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::game::Game;
use crate::views::army_placement::{create_army_placement, ViewArmyPlacement};
use crate::views::combat::{create_view_combat, ViewCombat};
use gloo::console::log as console_log;
use crate::views::dice_roll::{create_view_dice_roll, ViewDiceRoll};
use crate::views::message::{create_view_mesage, ViewMessage};

#[derive(Default, EnumIter, Debug, PartialEq, Clone)]
pub enum ViewsEnum {
    #[default]
    Turn,
    ArmyPlacement,
    Combat,
    DiceRolls,
    Message,
}

#[derive(Clone)]
pub struct ViewsStruct {
    pub turn: Rc<RefCell<ViewTurn>>,
    pub army_placement: Rc<RefCell<ViewArmyPlacement>>,
    pub combat: Rc<RefCell<ViewCombat>>,
    pub dice_rolls: Rc<RefCell<ViewDiceRoll>>,
    pub message: Rc<RefCell<ViewMessage>>,
}


pub struct ViewMain {
    head: WrpDiv,
    pub views: ViewsStruct,
    pub selected_view: ViewsEnum,
}

pub fn create_view_main(game: Rc<RefCell<Game>>, mount_id: &str) -> Rc<RefCell<ViewMain>> {
    console_log!("creating main view");

    let mut mount_turn = Div();
    let mut mount_army_placement = Div();
    let mut mount_combat = Div();
    let mut mount_dice_rolls = Div();
    let mut mount_message = Div();


    let head = Div().children(vec![
        mount_turn.get_clone(),
        mount_army_placement.get_clone(),
        mount_combat.get_clone(),
        mount_dice_rolls.get_clone(),
        mount_message.get_clone(),
    ]).mount(mount_id);

    return Rc::new(RefCell::new(ViewMain {
        head,
        views: ViewsStruct {
            turn: create_view_turn(game.clone(), &mount_turn.get_id()),
            army_placement: create_army_placement(game.clone(), &mount_army_placement.get_id()),
            combat: create_view_combat(game.clone(), &mount_combat.get_id()),
            dice_rolls: create_view_dice_roll(game.clone(), &mount_dice_rolls.get_id()),
            message: create_view_mesage(game.clone(), &mount_message.get_id()),
        },
        selected_view: Default::default(),
    }));
}

impl View for ViewMain {
    //todo find way to iterate trough views
    fn update(&mut self) {
        todo!()
    }
}

impl ViewMain {
    pub fn set_active(&self, view: ViewsEnum) {
        // view can be borrowed safely, because they should not be mut borrowed when this is called.
        // the game struct is mutably borrowed but not the views
        for _view in ViewsEnum::iter() {
            if _view == view {
                match _view {
                    ViewsEnum::Turn => { self.views.turn.borrow().show() }
                    ViewsEnum::ArmyPlacement => { self.views.army_placement.borrow().show() }
                    ViewsEnum::Combat => { self.views.combat.borrow().show() }
                    ViewsEnum::DiceRolls => { self.views.dice_rolls.borrow().show() }
                    ViewsEnum::Message => { self.views.message.borrow().show() }
                }
            } else {
                match _view {
                    ViewsEnum::Turn => { self.views.turn.borrow().hide() }
                    ViewsEnum::ArmyPlacement => { self.views.army_placement.borrow().hide() }
                    ViewsEnum::Combat => { self.views.combat.borrow().hide() }
                    ViewsEnum::DiceRolls => { self.views.dice_rolls.borrow().hide() }
                    ViewsEnum::Message => { self.views.message.borrow().hide() }
                }
            }
        }
    }

    pub fn hide_all(&self) {
        for _view in ViewsEnum::iter() {
            match _view {
                ViewsEnum::Turn => { self.views.turn.borrow().hide() }
                ViewsEnum::ArmyPlacement => { self.views.army_placement.borrow().hide() }
                ViewsEnum::Combat => { self.views.combat.borrow().hide() }
                ViewsEnum::DiceRolls => { self.views.dice_rolls.borrow().hide() }
                ViewsEnum::Message => { self.views.message.borrow().hide() }
            }
        }
    }
}


