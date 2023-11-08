use std::rc::Rc;
use std::cell::RefCell;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlOptionElement};
use crate::canvas::{DiceFaceTex, get_dice_tex};
use crate::element_getters::get_document;
use crate::game::Game;
use crate::model::CombatResult;
use crate::ui::wrap_elem::{WrapBtn, WrapHtml, WrapDiv, WrapSelect, WrapHeading, WrapDiceCanvas, chk_set_visbility};
use crate::ui::templates::*;
use crate::ui::traits::{HTML_Div, HTMLable};

const ALPHABET_LEN:usize = 26;
const ASCII_LOWER: [char; ALPHABET_LEN] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];


pub trait StatefullView<T> {
    fn create(doc:&Document)->Self;
    fn mount(&mut self);
    fn update(&mut self, state: T);
    fn update_self(&mut self);
    fn get(&self) -> T;
}

#[derive(Clone, Default)]
pub struct StateArmyPlacement {
    pub active: bool,
    pub armies: u32,
}

pub struct ViewArmyPlacement{
    pub state:StateArmyPlacement,
    template:WrapHtml,
    count_label: WrapDiv,
    mounted:bool,
}

impl StatefullView<StateArmyPlacement> for ViewArmyPlacement{
    fn create(doc: &Document) -> Self {
        let count_id =get_random_id();
        let mut ret = ViewArmyPlacement{
            state: Default::default(),
            template: WrapHtml::new(doc, "army_placement".to_string(),
                                    template_army_placement(&count_id).as_str() ),
            count_label: WrapDiv::new(doc,
                                      count_id, "lkmlk".to_string()),
            mounted: false,
        };
        ret.update_self();
        ret
    }

    fn mount(&mut self) {
        if self.mounted{
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
        self.count_label.set_text(format!("You still need to Place {} armies", self.state.armies));
        self.template.set_visibilty(self.state.active);
    }

    fn get(&self) -> StateArmyPlacement {
        self.state.clone()
    }
}


#[derive(Clone, Default)]
pub struct StateStartArmyPlacement {
    pub active: bool,
    pub current_player: u32,
    pub num_players:u32,
    pub armies: [u32; 6],
}

pub struct ViewStartArmyPlacement {
    pub state: StateStartArmyPlacement,
    template: WrapHtml,
    player_label: WrapDiv,
    army_count_label: WrapDiv,
    mounted:bool,
}


impl StatefullView<StateStartArmyPlacement> for ViewStartArmyPlacement {
    fn create(doc:&Document)->Self{
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
        if self.mounted{
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
        self.player_label.set_text(format!("Player {}", self.state.current_player + 1));
        self.army_count_label.set_text(format!("{} armies still available.",
                                               self.state.armies[self.state.current_player as usize]));
        self.template.set_visibilty(self.state.active)
    }

    fn get(&self) -> StateStartArmyPlacement {
        self.state.clone()
    }
}


#[derive(Clone, Default)]
pub struct StateHeader {
    pub active: bool,
    pub text: String,
}

pub struct ViewHeader{
//    template:WrapHtml,
    pub state:StateHeader,
    text_label: WrapDiv,
    mounted:bool
}

impl StatefullView<StateHeader> for ViewHeader{
    fn create(doc: &Document) -> Self {
        let mut ret = ViewHeader{
            state: StateHeader{
                active: true,
                text: "Player 1".to_string(),
            },
            text_label: WrapDiv::new(doc, "header".to_string(), "".to_string()),
            mounted: false,
        };

        ret.update_self();
        ret
    }

    fn mount(&mut self) {
        if self.mounted{
            panic!("component is already mounted")
        }
        self.mounted = true;
        self.update_self();
        self.text_label.mount();
    }

    fn update(&mut self, state: StateHeader) {
        self.state = state;
    }

    fn update_self(&mut self) {
        self.text_label.set_text(self.state.text.clone());
        self.text_label.set_visibilty(self.state.active);
    }

    fn get(&self) -> StateHeader {
        self.state.clone()
    }
}


#[derive(Clone, Default)]
pub struct StateTurnStart {
    pub active: bool,
    pub armies: u32,
}

pub struct ViewTurnStart{
    template:WrapHtml,
    state:StateTurnStart,
    text_label: WrapDiv,
    reinforce_btn:WrapBtn,
}

#[derive(Clone, Default)]
pub struct StateCombat {
    pub active: bool,
    pub attack_location:String,
    pub armies_attacking: u32,
    pub armies_defending:u32,
    pub id_attacker:Option<u32>,
    pub id_defender:Option<u32>,
}

pub struct CombatArmySelect{
    main:WrapDiv,
    select:WrapSelect,
    player_text:WrapDiv,
    btn_next:WrapBtn,
}

pub struct ViewCombat{
    state:StateCombat,
    template:WrapHtml,
    title: WrapDiv,
    location_text: WrapHeading,
    balance_text: WrapDiv,
    menu_defend:CombatArmySelect,
    menu_attack:CombatArmySelect,
}

impl StatefullView<StateCombat> for ViewCombat{
    fn create(doc: &Document) -> Self {
        let id_title = get_random_id();
        let id_location = get_random_id();
        let id_balance = get_random_id();
        let id_main = (get_random_id(), get_random_id());
        let id_select = (get_random_id(), get_random_id());
        let id_player_text = (get_random_id(), get_random_id());
        let id_btn = (get_random_id(), get_random_id());

        let template = WrapHtml::new(doc, "combat".to_string(), template_combat_menu(
            &id_title, &id_location, &id_balance, &id_select, &id_player_text,
            &id_btn, &id_main
        ).as_str());
        template.mount();
        ViewCombat{
            state: Default::default(),
            title: WrapDiv::new_from_id(&id_title),
            location_text: WrapHeading::new_from_id(&id_location),
            balance_text: WrapDiv::new_from_id(&id_balance),
            menu_defend: CombatArmySelect {
                main: WrapDiv::new_from_id(&id_main.1),
                select: WrapSelect::new_from_id(&id_select.1),
                player_text: WrapDiv::new_from_id(&id_player_text.1),
                btn_next: WrapBtn::new_from_id(&id_btn.1),
            },
            menu_attack: CombatArmySelect {
                main: WrapDiv::new_from_id(&id_main.0),
                select: WrapSelect::new_from_id(&id_select.0),
                player_text: WrapDiv::new_from_id(&id_player_text.0),
                btn_next: WrapBtn::new_from_id(&id_btn.0),
            },
            template,
        }
    }

    fn mount(&mut self) {
        self.template.set_visibilty(false);
        //todo set btn next
    }

    fn update(&mut self, state: StateCombat) {
        self.state = state;
        self.update_self();
    }

    fn update_self(&mut self) {
        self.location_text.set_text(format!("Attack in {}", self.state.attack_location));
        self.balance_text.set_text(format!("Defenders {}:{} Attackers",
                                           self.state.armies_defending, self.state.armies_attacking));

        let set_visibilty_child = |elem :&WrapSelect, idx:u32, visible:bool|{
            let child = elem.elem.children()
                .get_with_index(idx).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
            chk_set_visbility(&child.style(), visible);
        };

        let handle_combat_view = |view:&mut CombatArmySelect, player:&Option<u32>, armies:u32,
                                  is_attacker:bool|{
            if player.is_some(){
                view.player_text.set_text(format!("Player {}", player.as_ref().unwrap()));
                if armies > 2 && is_attacker{
                    log!(format!("attacker and > 2 is attack {} armies {}",is_attacker, armies));
                    set_visibilty_child(&view.select, 1, true );
                    set_visibilty_child(&view.select, 2, true );
                    return;
                }else if  armies > 1 {
                    log!(format!("armies > 1 is attack {} armies {}",is_attacker, armies));
                    set_visibilty_child(&view.select, 1, true );
                    set_visibilty_child(&view.select, 2, false );
                } else {
                    log!(format!("1 army is attack {} armies {}",is_attacker, armies));
                    set_visibilty_child(&view.select, 1, false );
                    set_visibilty_child(&view.select, 2, false);
                }

            }else {
                view.main.set_visibilty(false)
            }
        };
        handle_combat_view(&mut self.menu_attack, &self.state.id_attacker,
                           self.state.armies_attacking, true);
        handle_combat_view(&mut self.menu_defend, &self.state.id_defender,
                           self.state.armies_defending, false);
    }

    fn get(&self) -> StateCombat {
        self.state.clone()
    }
}

pub struct ViewDiceRoll {
    state:CombatResult,
    template:WrapHtml,
    next_btn:WrapBtn,
    canvas_top:WrapDiceCanvas,
    canvas_bot:WrapDiceCanvas,
    dice_face_texes:Rc<RefCell<Vec<DiceFaceTex>>>,
}

impl StatefullView<CombatResult> for ViewDiceRoll{
    fn create(doc: &Document) -> Self {
        let id_canvases = (get_random_id(), get_random_id());
        let id_next_btn = get_random_id();
        let template = WrapHtml::new(doc, "dice_roll".to_string(),
                                    template_dice_roll(&id_canvases, &id_next_btn).as_str());
        template.mount();
        ViewDiceRoll{
            state: Default::default(),
            template,
            next_btn: WrapBtn::new_from_id(&id_next_btn),
            canvas_top: WrapDiceCanvas::new_from_id(&id_canvases.0),
            canvas_bot: WrapDiceCanvas::new_from_id(&id_canvases.1),
            dice_face_texes: get_dice_tex(),
        }
    }

    fn mount(&mut self) {
        self.template.set_visibilty(false);
    }

    fn update(&mut self, state: CombatResult) {
        self.state = state;
        self.update_self()
    }

    fn update_self(&mut self) {
        if self.state.combat_finished{
            self.template.set_visibilty(false);
        }else {
            self.template.set_visibilty(true);
            self.canvas_top.draw_dice_rolls(&self.state.dice_roll_attacker,
                                            self.dice_face_texes.clone());
            self.canvas_bot.draw_dice_rolls(&self.state.dice_roll_defender,
                                            self.dice_face_texes.clone())
        }

    }

    fn get(&self) -> CombatResult {
        return self.state.clone()
    }
}


#[derive(Clone, Default )]
pub struct StateGameEnd {
    pub active: bool,
    pub armies: u32,
}

pub struct ViewGameEnd{
    state:StateArmyPlacement,
    template:WrapHtml,
    count_label: WrapDiv,
}

#[derive(Clone )]
pub enum Selected{
    Header,
    StartPlace,
    Place,
    Combat,
    DiceRolling,
}

pub struct UiStateManager {
    game_ref: Rc<RefCell<Game>>,
    pub header:ViewHeader,
    pub start_army_placement: ViewStartArmyPlacement,
    pub army_placement:ViewArmyPlacement,
    pub selected:Selected,
    pub combat:ViewCombat,
    pub dice_rolls:ViewDiceRoll,
    pub info_div:WrapDiv,
}

impl UiStateManager {
    pub fn build(game_ref: Rc<RefCell<Game>>) -> UiStateManager {
        let doc = get_document();
        UiStateManager {
            game_ref,
            header: ViewHeader::create(&doc),
            start_army_placement: ViewStartArmyPlacement::create(&doc),
            army_placement: ViewArmyPlacement::create(&doc),
            selected: Selected::Header,
            combat: ViewCombat::create(&doc),
            dice_rolls: ViewDiceRoll::create(&doc),
            info_div: WrapDiv::new_from_id(&"info".to_string()),
        }
    }

    pub fn mount(&mut self) {
        self.header.mount();
        self.start_army_placement.mount();
        self.army_placement.mount();
        //self.combat.mount();
        self.dice_rolls.mount();
    }

    pub fn update_all(&mut self){
        self.header.update_self();
        self.start_army_placement.update_self();
        self.army_placement.update_self();
    }

}


fn get_random_id() -> String {
    let mut rand_arry = [0u8; 10];
    web_sys::window().unwrap().crypto().unwrap()
        .get_random_values_with_u8_array(&mut rand_arry).unwrap();
    let ret:String= rand_arry.iter().map(|num|{
        let get_idx = (num / 10) as usize;
        ASCII_LOWER[get_idx]
    }).collect();
    ret
}
