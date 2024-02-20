use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use marble::{impl_visibility, impl_visibility_no_update};
use marble::wrap::{*};
use marble::traits::{*};
use crate::game::Game;
use web_sys::{HtmlOptionElement};
use gloo::console::log as console_log;
use crate::utils::structs::AttackDefendPair;

pub struct CombatArmySelect {
    head: WrpDiv,
    select: WrpSelect,
    player_text: WrpDiv,
    btn_next: WrpBtn,
}

impl CombatArmySelect {
    fn new() -> CombatArmySelect {
        CombatArmySelect {
            head: Div(),
            select: Select(),
            player_text: Div(),
            btn_next: Button(),
        }
    }

    fn get_html(&mut self, retreat_btn: Option<WrpBtn>) -> WrpDiv {
        let is_attacker = if retreat_btn.is_some() { true } else {false};
        let max_options = if is_attacker { 3u32 } else { 2u32 };
        let btn_text = if is_attacker { "Attack" } else { "Defend" };

        self.select.inline_style("float: left;white-space: break-spaces;");
        for i in 0..max_options {
            let val = (i + 1).to_string();
            self.select.inline_child(OptionElem().value(&val).txt(&val));
        }

        let mut button_div = Div();
        self.head.inline_style("margin-bottom: 30px;");
        self.head.inline_children(vec![
            self.player_text.get_clone().style("margin-bottom: 10px"),
            Div().child(
                Label().style("float: left;white-space: break-spaces;").attr("for", &self.select.get_id())
            ).child(
                self.select.get_clone()
            ).child(
                Div().style("white-space: break-spaces;").txt("armies")
            ),
            button_div.get_clone().style("white-space: break-spaces;").child(
                self.btn_next.get_clone().txt(btn_text)
            )
        ]);

        if retreat_btn.is_some(){
            button_div.inline_child(retreat_btn.unwrap().txt("retreat"))
        }

        return self.head.get_clone();
    }

    pub fn get_armies_selected(&self)->u32{
        let val_str = self.select.get_value();
        val_str.parse().unwrap()
    }
}

impl_visibility_no_update!(CombatArmySelect);

pub struct ViewCombat {
    head: WrpDiv,
    game_ref:Rc<RefCell<Game>>,
    title: WrpH3,
    balance_text: WrpDiv,
    retreat_btn:WrpBtn,
    pub submenus: AttackDefendPair<CombatArmySelect>,

    pub attack_location: String,// todo is this needed?
    pub armies: AttackDefendPair<u32>,
    pub player_ids: AttackDefendPair<u32>,
    pub prov_ids: AttackDefendPair<u32>,
    pub is_visible: AttackDefendPair<bool>,
}

impl View for ViewCombat{
    fn update(&mut self) {
        self.title.inline_txt(&format!("Attack in {}", self.attack_location));
        self.balance_text.inline_txt(&format!("Defenders {}:{} Attackers",
                                            self.armies.defend, self.armies.attack));
        let update_side = |submenu:&CombatArmySelect, is_attacker:bool,
        armies:u32, player_id:u32, is_visible:bool|{
            //todo maybe this should happen inside the nested view
            if is_visible{
                submenu.show();
            }else {
                submenu.hide();
                return;
            }
            submenu.player_text.inline_txt(&format!("Player {}", player_id +1));

            let available_armies:u32;
            if armies > 2 && is_attacker{
                available_armies = 3;
            } else if armies > 1{
                available_armies = 2;
            }else{
                available_armies = 1;
            }

            let children  = submenu.select.node.child_nodes();
            for i in 0..children.length(){
                let node= children.get(i).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
                if node.text_content().unwrap().parse::<u32>().unwrap() <= available_armies {
                    chk_set_visbility(&node.style(), true);
                }else {
                    chk_set_visbility(&node.style(), false);
                }
                // todo intellegently select correct legal value
                // aka select lower, when over the limit
            }
        };
        update_side(&self.submenus.attack, true, self.armies.attack,
            self.player_ids.attack, self.is_visible.attack);
        update_side(&self.submenus.defend, true, self.armies.defend,
                    self.player_ids.defend, self.is_visible.defend)
    }
}


impl_visibility!(ViewCombat);

pub fn create_view_combat(game: Rc<RefCell<Game>>, mount_id: &str) -> Rc<RefCell<ViewCombat>> {
    console_log!("creating combat view");
    let mut title = H3();
    let mut balance_text = Div();
    let mut retreat_btn = Button();
    let mut submenus = AttackDefendPair {
        attack: CombatArmySelect::new(),
        defend: CombatArmySelect::new(),
    };

    let head = Div().child(
        title.get_clone()
    ).children(vec![
        balance_text.get_clone().style("margin-bottom: 15px;"),
        submenus.attack.get_html(Some(retreat_btn.get_clone())),
        submenus.defend.get_html(None),
    ]).mount(mount_id);

    let view_combat = ViewCombat{
        head,
        game_ref: game,
        title,
        balance_text,
        retreat_btn,
        submenus,
        attack_location: "".to_string(),
        armies: AttackDefendPair::default(),
        player_ids: AttackDefendPair::default(),
        prov_ids: AttackDefendPair::default(),
        is_visible: AttackDefendPair::default(),
    };
    let rc_view = Rc::new(RefCell::new(view_combat));

    rc_view.borrow_mut().submenus.attack.btn_next.set_state_handler(rc_view.clone(),
        #[allow(unused_mut)]
        |mut s:RefMut<ViewCombat>|{
            s.game_ref.borrow_mut().handle_ui_combat_roll(true);
        }, "attack btn"
    );

    rc_view.borrow_mut().submenus.defend.btn_next.set_state_handler(rc_view.clone(),
        #[allow(unused_mut)]
        |mut s:RefMut<ViewCombat>|{
            s.game_ref.borrow_mut().handle_ui_combat_roll(false);
        }, "defend btn"
    );

    rc_view.borrow_mut().retreat_btn.set_state_handler(rc_view.clone(),
        #[allow(unused_mut)]
       |mut s:RefMut<ViewCombat>|{
           s.game_ref.borrow_mut().handle_ui_retreat();
       }, "retreat btn"
    );


    return rc_view;
}


