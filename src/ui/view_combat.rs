use std::cell::RefCell;
use std::rc::Rc;
//use gloo::console::log;
use web_sys::{Document, HtmlOptionElement};
use crate::model::Player;
use crate::ui::templates::template_combat_menu;
use crate::ui::ui_state_manager::{borrow_game_safe, get_random_id, StatefullView};
use crate::ui::wrap_elem::{chk_set_visbility, WrapBtn, WrapDiv, WrapHeading, WrapHtml, WrapSelect, HTML_Div, HTMLable};
use gloo::console::log as console_log;
use wasm_bindgen::JsCast;
use crate::game::Game;


#[derive(Clone, Default)]
pub struct StateCombat {
    pub attack_location: String,
    pub armies_attacking: u32,
    pub armies_defending: u32,
    pub id_attacker: u32,
    pub id_defender: u32,
    pub prov_id_attacker: u32,
    pub prov_id_defender: u32,
    pub attack_visible:bool,
    pub defend_visible:bool,
}


pub struct ViewCombat {
    state: StateCombat,
    template: WrapHtml,
    title: WrapDiv,
    location_text: WrapHeading,
    balance_text: WrapDiv,
    menu_defend: CombatArmySelect,
    menu_attack: CombatArmySelect,
    btn_retreat: WrapBtn,
}

pub struct CombatArmySelect {
    main: WrapDiv,
    select: WrapSelect,
    player_text: WrapDiv,
    btn_next: WrapBtn,
}


impl ViewCombat {
    pub fn get_armies_selected(&self )-> (u32, u32){
        return (
            self.menu_attack.select.get_value().parse().unwrap(),
            self.menu_defend.select.get_value().parse().unwrap()
        )
    }

    pub fn reset_player_visibilty(&mut self, players:&Vec<Player>){
        if !players[self.state.id_attacker as usize].is_computer{
            self.state.attack_visible = true;
        }
        if !players[self.state.id_defender as usize].is_computer{
            self.state.defend_visible = true;
        }
    }
}


impl StatefullView<StateCombat> for ViewCombat {
    fn create(doc: &Document) -> Self {
        let id_title = get_random_id();
        let id_location = get_random_id();
        let id_balance = get_random_id();
        let id_main = (get_random_id(), get_random_id());
        let id_select = (get_random_id(), get_random_id());
        let id_player_text = (get_random_id(), get_random_id());
        let id_btn = (get_random_id(), get_random_id());
        let id_retreat = get_random_id();

        let template = WrapHtml::new(doc, "combat".to_string(), template_combat_menu(
            &id_title, &id_location, &id_balance, &id_select, &id_player_text,
            &id_btn, &id_main, Some(&id_retreat)).as_str());
        template.mount();
        ViewCombat { state: Default::default(), title: WrapDiv::new_from_id(&id_title), location_text: WrapHeading::new_from_id(&id_location), balance_text: WrapDiv::new_from_id(&id_balance), menu_defend: CombatArmySelect {
                main: WrapDiv::new_from_id(&id_main.1),
                select: WrapSelect::new_from_id(&id_select.1),
                player_text: WrapDiv::new_from_id(&id_player_text.1),
                btn_next: WrapBtn::new_from_id(&id_btn.1),
            }, menu_attack: CombatArmySelect {
                main: WrapDiv::new_from_id(&id_main.0),
                select: WrapSelect::new_from_id(&id_select.0),
                player_text: WrapDiv::new_from_id(&id_player_text.0),
                btn_next: WrapBtn::new_from_id(&id_btn.0),
            }, btn_retreat:WrapBtn::new_from_id(&id_retreat),
            template, }
    }

    fn mount(&mut self) {

    }

    fn update(&mut self, state: StateCombat) {
        self.state = state;
        self.update_self();
    }

    fn update_self(&mut self) {
        self.location_text.set_text(&format!("Attack in {}", self.state.attack_location));
        self.balance_text.set_text(&format!("Defenders {}:{} Attackers",
                                           self.state.armies_defending, self.state.armies_attacking));

        let set_visibilty_child = |elem: &WrapSelect, idx: u32, visible: bool| {
            let child = elem.elem.children()
                .get_with_index(idx).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
            chk_set_visbility(&child.style(), visible);
        };

        let handle_combat_view = |view: &mut CombatArmySelect, player: &u32, armies: u32,
                                  is_attacker: bool| {
            view.player_text.set_text(&format!("Player {}", player + 1));
            if armies > 2 && is_attacker {
                //log!(format!("attacker and > 2 is attack {} armies {}",is_attacker, armies));
                set_visibilty_child(&view.select, 1, true);
                set_visibilty_child(&view.select, 2, true);
                return;
            } else if armies > 1 {
                //log!(format!("armies > 1 is attack {} armies {}",is_attacker, armies));
                if view.select.get_value() == "3"{
                    view.select.set_value("2")
                }
                set_visibilty_child(&view.select, 1, true);
                set_visibilty_child(&view.select, 2, false);
            } else {
                let val = view.select.get_value();
                if val == "3" || val == "2"{
                    view.select.set_value("1")
                }
                //log!(format!("1 army is attack {} armies {}",is_attacker, armies));
                set_visibilty_child(&view.select, 1, false);
                set_visibilty_child(&view.select, 2, false);
            }
        };
        handle_combat_view(&mut self.menu_attack, &self.state.id_attacker,
                           self.state.armies_attacking, true);
        handle_combat_view(&mut self.menu_defend, &self.state.id_defender,
                           self.state.armies_defending, false);

        self.menu_attack.main.set_visibilty(self.state.attack_visible);
        self.menu_defend.main.set_visibilty(self.state.defend_visible);
    }

    fn get(&self) -> StateCombat {
        self.state.clone()
    }


    fn hide(&mut self) {
        self.template.set_visibilty(false);
    }

    fn show(&mut self) {
        console_log!("set combat to visible");
        self.template.set_visibilty(true);
    }

    fn set_handlers(&mut self, game_ref: &Rc<RefCell<Game>>) {
        let ref_attack = game_ref.clone();
        self.menu_attack.btn_next.set_click_handler(Box::from(move |_| {
            borrow_game_safe(&ref_attack,"attack btn".to_string(),
                             |mut g| g.handle_ui_combat_roll(true))
        }));

        let ref_defend = game_ref.clone();
        self.menu_defend.btn_next.set_click_handler(Box::from(move |_| {
            borrow_game_safe(&ref_defend,"defend btn".to_string(),
                             |mut g| g.handle_ui_combat_roll(false))
        }));

        let ref_retreat = game_ref.clone();
        self.btn_retreat.set_click_handler(Box::from(move |_| {
            borrow_game_safe(&ref_retreat,"retreat btn".to_string(),
                             |mut g| g.handle_ui_retreat())
        }));
    }
}
