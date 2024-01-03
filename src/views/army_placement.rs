use std::cell::RefCell;
use std::rc::Rc;
use marble::impl_visibility;
use marble::wrap::{Div,  WrpDiv, };
use marble::traits::{Visibilty, View, Inline};
use crate::build_constructor;
use crate::game::Game;

pub struct ViewArmyPlacement{
    head: WrpDiv,
    count_label: WrpDiv,
    pub armies: u32,
    pub end_turn_placement:bool
}

impl View for ViewArmyPlacement{
    fn update(&mut self) {
        self.count_label.inline_txt(&format!("You still need to Place {} armies",
                                           self.armies));
    }
}

impl_visibility!(ViewArmyPlacement);

fn create_view_army_placement(_: Rc<RefCell<Game>>, mount_id:&str) -> ViewArmyPlacement{
    let mut count_label = Div();

    let head = Div().child(
        count_label.get_clone()
    ).mount(mount_id);

    return ViewArmyPlacement{
        head,
        count_label,
        armies: 0,
        end_turn_placement: false,
    };
}

//todo this macro can only be used if you don't have to set handlers
// also it sucks to use i should get rid of it
build_constructor!(create_army_placement, create_view_army_placement, ViewArmyPlacement);
