use std::cell::RefCell;
use std::rc::Rc;
use marble::wrap::{Div,  WrpDiv, WrpBtn, Button};
use marble::traits::{Visibilty, View, Inline, };
use crate::views::turn::{create_view_turn, ViewTurn};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::game::Game;
use crate::views::army_placement::{create_army_placement, ViewArmyPlacement};

#[derive(Default, EnumIter, Debug, PartialEq)]
enum ViewsEnum {
    #[default]
    Turn,
    ArmyPlacement,
}

#[derive(Clone)]
pub struct ViewsStruct{
    turn:Rc<RefCell<ViewTurn>>,
    army_placement:Rc<RefCell<ViewArmyPlacement>>,
}


struct ViewMain {
    head:WrpDiv,
    views:ViewsStruct,
    pub selected_view :ViewsEnum,
}

fn create_main_view(game: Rc<RefCell<Game>>, mount_id:&str) -> Rc<RefCell<ViewMain>>{
    let mut mount_turn = Div();
    let mut mount_army_placement = Div();

    let head = Div().children(vec![
        mount_turn.get_clone(),
        mount_army_placement.get_clone(),
    ]).mount(mount_id);

    return Rc::new(RefCell::new(ViewMain{
        head,
        views: ViewsStruct {
            turn: create_view_turn(game.clone(), &mount_army_placement.get_id()),
            army_placement: create_army_placement(game.clone(), &mount_army_placement.get_id())
        },
        selected_view: Default::default(),
    }))
}

impl View for ViewMain {
    //todo find way to iterate trough views
    fn update(&mut self) {
        todo!()
    }
}

impl ViewMain {
    pub fn set_active(&self, view:ViewsEnum ){
        // view can be borrowed safely, because they should not be mut borrowed when this is called.
        // the game struct is mutably borrowed but not the views
        for v in ViewsEnum::iter(){
            if v == view{
                match v {
                    ViewsEnum::Turn => {self.views.turn.borrow().show()}
                    ViewsEnum::ArmyPlacement => {self.views.army_placement.borrow().show()}
                }
            }else {
                match v {
                    ViewsEnum::Turn => {self.views.turn.borrow().hide()}
                    ViewsEnum::ArmyPlacement => {self.views.army_placement.borrow().hide()}
                }
            }
        }
    }

    pub fn hide_all(&self){
        todo!()
    }
}


