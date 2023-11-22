use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlOptionElement};
use crate::canvas::{DiceFaceTex, get_dice_tex};
use crate::element_getters::{get_document};
use crate::model::{CombatResult, Player};
use crate::ui::wrap_elem::{WrapBtn, WrapHtml, WrapDiv, WrapSelect, WrapHeading, WrapDiceCanvas, chk_set_visbility};
use crate::ui::templates::*;
use crate::ui::traits::{HTML_Div, HTMLable};
use gloo::console::log as console_log;
use crate::game::Game;
use crate::ui::main::UiState;


const ALPHABET_LEN: usize = 26;
const ASCII_LOWER: [char; ALPHABET_LEN] = [
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

#[derive(Clone, Default)]
pub struct StateArmyPlacement {
    pub armies: u32,
    pub active_player: u32,
}


pub struct ViewArmyPlacement {
    state: StateArmyPlacement,
    template: WrapHtml,
    count_label: WrapDiv,
    mounted: bool,
}


impl StatefullView<StateArmyPlacement> for ViewArmyPlacement {
    fn create(doc: &Document) -> Self {
        let count_id = get_random_id();
        let mut ret = ViewArmyPlacement {
            state: Default::default(),
            template: WrapHtml::new(doc, "army_placement".to_string(),
                                    template_army_placement(&count_id).as_str()),
            count_label: WrapDiv::new(doc,
                                      count_id, "lkmlk".to_string()),
            mounted: false,
        };
        ret.update_self();
        ret
    }

    fn mount(&mut self) {
        if self.mounted {
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
        self.count_label.set_text(format!("You still need to Place {} armies",
                                          self.state.armies));
    }

    fn get(&self) -> StateArmyPlacement {
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
        self.player_label.set_text(format!("Player {}", self.state.current_player + 1));
        self.army_count_label.set_text(format!("{} armies still available.",
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


#[derive(Clone, Default)]
pub struct StateTurn {
    pub active_player: u32,
    pub can_reinforce: bool,
}

pub struct ViewTurn {
    template: WrapHtml,
    label_player: WrapDiv,
    btn_next_turn: WrapBtn,
    state: StateTurn,
    mounted: bool,
}


impl StatefullView<StateTurn> for ViewTurn {
    fn create(doc: &Document) -> Self {
        let id_label = get_random_id();
        let id_btn_next_turn = get_random_id();
        let template = WrapHtml::new(doc, "turn_start".to_string(), template_turn_menu(
            &id_label, &id_btn_next_turn,
        ).as_str());
        template.mount();
        let mut ret = ViewTurn {
            template,
            label_player: WrapDiv::new_from_id(&id_label),
            btn_next_turn: WrapBtn::new_from_id(&id_btn_next_turn),
            state: StateTurn {
                active_player: 0,
                can_reinforce: true,
            },
            mounted: false,
        };

        ret.update_self();
        ret
    }

    fn mount(&mut self) {
        if self.mounted {
            panic!("component is already mounted")
        }
        self.mounted = true;
        self.update_self();
    }

    fn update(&mut self, state: StateTurn) {
        self.state = state;
    }

    fn update_self(&mut self) {
        self.label_player.set_text(format!("Player {}", self.state.active_player + 1));
    }

    fn get(&self) -> StateTurn {
        self.state.clone()
    }

    fn hide(&mut self) {
        self.template.set_visibilty(false);
    }

    fn show(&mut self) {
        self.template.set_visibilty(true);
    }

    fn set_handlers(&mut self, game_ref: &Rc<RefCell<Game>>) {
        let ref_next_turn = game_ref.clone();
        self.btn_next_turn.set_click_handler(Box::from(move |_| {
            borrow_game_safe(&ref_next_turn, "attack btn".to_string(),
                             |mut g| {g.handle_ui_end_turn()});
        }));
    }
}


#[derive(Clone, Default)]
pub struct StateTurnStart {
    pub armies: u32,
}

pub struct ViewTurnStart {
    template: WrapHtml,
    state: StateTurnStart,
    text_label: WrapDiv,
    reinforce_btn: WrapBtn,
}

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


pub struct CombatArmySelect {
    main: WrapDiv,
    select: WrapSelect,
    player_text: WrapDiv,
    btn_next: WrapBtn,
}

pub struct ViewCombat {
    state: StateCombat,
    template: WrapHtml,
    title: WrapDiv,
    location_text: WrapHeading,
    balance_text: WrapDiv,
    menu_defend: CombatArmySelect,
    menu_attack: CombatArmySelect,
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

    fn show_attack(&mut self, is_visible: bool) {
        self.state.attack_visible = is_visible;
        self.update_self();
    }

    fn show_defend(&mut self, is_visible: bool) {
        self.state.defend_visible = is_visible;
        self.update_self();
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

        let template = WrapHtml::new(doc, "combat".to_string(), template_combat_menu(
            &id_title, &id_location, &id_balance, &id_select, &id_player_text,
            &id_btn, &id_main,
        ).as_str());
        template.mount();
        ViewCombat {
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

        let set_visibilty_child = |elem: &WrapSelect, idx: u32, visible: bool| {
            let child = elem.elem.children()
                .get_with_index(idx).unwrap().dyn_into::<HtmlOptionElement>().unwrap();
            chk_set_visbility(&child.style(), visible);
        };

        let handle_combat_view = |view: &mut CombatArmySelect, player: &u32, armies: u32,
                                  is_attacker: bool| {
            view.player_text.set_text(format!("Player {}", player + 1));
            if armies > 2 && is_attacker {
                log!(format!("attacker and > 2 is attack {} armies {}",is_attacker, armies));
                set_visibilty_child(&view.select, 1, true);
                set_visibilty_child(&view.select, 2, true);
                return;
            } else if armies > 1 {
                log!(format!("armies > 1 is attack {} armies {}",is_attacker, armies));
                set_visibilty_child(&view.select, 1, true);
                set_visibilty_child(&view.select, 2, false);
            } else {
                log!(format!("1 army is attack {} armies {}",is_attacker, armies));
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
            borrow_game_safe(&ref_defend,"attack btn".to_string(),
                             |mut g| g.handle_ui_combat_roll(false))
        }));
    }
}

pub struct ViewDiceRoll {
    state: CombatResult,
    template: WrapHtml,
    next_btn: WrapBtn,
    canvas_top: WrapDiceCanvas,
    canvas_bot: WrapDiceCanvas,
    dice_face_texes: Rc<RefCell<Vec<DiceFaceTex>>>,
}

impl StatefullView<CombatResult> for ViewDiceRoll {
    fn create(doc: &Document) -> Self {
        let id_canvases = (get_random_id(), get_random_id());
        let id_next_btn = get_random_id();
        let template = WrapHtml::new(doc, "dice_roll".to_string(),
                                     template_dice_roll(&id_canvases, &id_next_btn).as_str());
        template.mount();
        ViewDiceRoll {
            state: CombatResult::default(),
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
        console_log!("updateing dice view");
        console_log!(format!("{:?}", self.state));

            self.canvas_top.clear_canvas();
            self.canvas_bot.clear_canvas();
            self.canvas_top.draw_dice_rolls(&self.state.dice_roll_attacker,
                                            self.dice_face_texes.clone());
            self.canvas_bot.draw_dice_rolls(&self.state.dice_roll_defender,
                                            self.dice_face_texes.clone())

    }

    fn get(&self) -> CombatResult {
        return self.state.clone();
    }

    fn hide(&mut self) {
        self.template.set_visibilty(false);
    }

    fn show(&mut self) {
        self.template.set_visibilty(true);
    }

    fn set_handlers(&mut self, game_ref: &Rc<RefCell<Game>>) {
        let game = game_ref.clone();
        self.next_btn.set_click_handler(Box::from(move |_|{
            borrow_game_safe(&game, "dice next btn".to_string(),
                |mut g| g.handle_ui_dice_next())
        }))
    }
}


#[derive(Clone, Default)]
pub struct StateGameEnd {
    pub active: bool,
    pub armies: u32,
}

pub struct ViewGameEnd {
    state: StateArmyPlacement,
    template: WrapHtml,
    count_label: WrapDiv,
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
            info_div: WrapDiv::new_from_id(&"info".to_string()),
        }
    }

    pub fn mount(&mut self) {
        self.turn_menu.mount();
        self.start_army_placement.mount();
        self.army_placement.mount();
        self.combat.mount();
        self.dice_rolls.mount();
        self.hide_all();
    }

    pub fn update_all(&mut self) {
        self.turn_menu.update_self();
        self.start_army_placement.update_self();
        self.army_placement.update_self();
        self.combat.update_self();
        self.dice_rolls.update_self();
    }

    pub fn select_view(&mut self, view: &UiState) {
        self.hide_all();
        match view {
            UiState::ARMY_PLACEMENT_START => { self.start_army_placement.show() }
            UiState::ARMY_PLACEMENT => { self.army_placement.show() }
            UiState::TURN => { self.turn_menu.show() }
            UiState::COMBAT => { self.combat.show() }
            UiState::DICE_ROLL => { self.dice_rolls.show() }
            UiState::GAME_END => { todo!() }
            UiState::CARD_SELECT => { todo!() }
            _ => {}
        }
        self.update_all()
    }

    pub fn hide_all(&mut self) {
        self.turn_menu.hide();
        self.start_army_placement.hide();
        self.army_placement.hide();
        self.combat.hide();
        self.dice_rolls.hide();
    }

    pub fn set_handlers(&mut self, game_ref: &Rc<RefCell<Game>>) {
        self.turn_menu.set_handlers(game_ref);
        self.start_army_placement.set_handlers(game_ref);
        self.army_placement.set_handlers(game_ref);
        self.combat.set_handlers(game_ref);
        self.dice_rolls.set_handlers(game_ref);
    }
}


fn get_random_id() -> String {
    let mut rand_arry = [0u8; 10];
    web_sys::window().unwrap().crypto().unwrap()
        .get_random_values_with_u8_array(&mut rand_arry).unwrap();
    let ret: String = rand_arry.iter().map(|num| {
        let get_idx = (num / 10) as usize;
        ASCII_LOWER[get_idx]
    }).collect();
    ret
}

fn borrow_game_safe(game_ref: &Rc<RefCell<Game>>, name:String, func: fn(RefMut<'_, Game>) ) {
    let borrow = game_ref.try_borrow_mut();
    if borrow.is_err() {
        console_log!(format!("could not borrow in {}", name));
    }else {
        func(borrow.unwrap())
    }
}

