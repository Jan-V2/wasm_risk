use std::cell::RefCell;
use std::rc::Rc;
use marble::impl_visibility;
use marble::wrap::{*};
use marble::traits::{*};
use crate::build_constructor;
use crate::game::Game;
use gloo::console::log as console_log;
use crate::views::info::ViewInfo;

//todo player label?

pub struct ViewArmyPlacement{
    head: WrpDiv,
    count_label: WrpDiv,
    pub armies: u32,
    pub default_msg:String,
}

impl View for ViewArmyPlacement{
    fn update(&mut self) {
        self.count_label.inline_txt(&format!("You still need to Place {} armies",
                                           self.armies));
    }
}


impl DefaultMsg for ViewInfo{
    fn default_msg(&self) -> String {
        return "Click on the map, to place your armies".to_string();
    }
}


impl ViewArmyPlacement{
    pub fn reset(&mut self, armies:u32){
        self.armies = armies;
        self.update();
    }
}

impl_visibility!(ViewArmyPlacement);

fn create_view_army_placement(_: Rc<RefCell<Game>>, mount_id:&str) -> ViewArmyPlacement{
    console_log!("creating army placement view");

    let mut count_label = Div();

    let head = Div().child(
        count_label.get_clone()
    ).mount(mount_id);

    return ViewArmyPlacement{
        head,
        count_label,
        armies: 0,
        default_msg: "Please click on a province, to place an army.".to_string(),
    };
}

//todo this macro can only be used if you don't have to set handlers
// also it sucks to use i should get rid of it
build_constructor!(create_army_placement, create_view_army_placement, ViewArmyPlacement);
