use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{Document};
use crate::element_getters::get_document;
use crate::game::Game;
use crate::ui::wrap_elem::{WrapBtn, WrapHtml, WrapDiv, WrapSelect, WrapHeading};
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
                main: WrapDiv::new_from_id(&id_main.0),
                select: WrapSelect::new_from_id(&id_select.0),
                player_text: WrapDiv::new_from_id(&id_player_text.0),
                btn_next: WrapBtn::new_from_id(&id_btn.0),
            },
            menu_attack: CombatArmySelect {
                main: WrapDiv::new_from_id(&id_main.1),
                select: WrapSelect::new_from_id(&id_select.1),
                player_text: WrapDiv::new_from_id(&id_player_text.1),
                btn_next: WrapBtn::new_from_id(&id_btn.1),
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
        let handle_combat_view = |view:&mut CombatArmySelect, player:&Option<u32>|{
            if player.is_some(){
                view.player_text.set_text(format!("Player {}", player.as_ref().unwrap()))
                //todo limit options in select
            }else {
                view.main.set_visibilty(false)
            }
        };
        handle_combat_view(&mut self.menu_attack, &self.state.id_attacker);
        handle_combat_view(&mut self.menu_defend, &self.state.id_defender);
    }

    fn get(&self) -> StateCombat {
        self.state.clone()
    }
}

#[derive(Clone, Default)]
pub struct StateCombatEnd {
    pub active: bool,
    pub armies: u32,
}

pub struct ViewCombatEnd{
    state:StateArmyPlacement,
    template:WrapHtml,
    count_label: WrapDiv,
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
    Combat_end,
}

pub struct UiStateManager {
    game_ref: Rc<RefCell<Game>>,
    pub header:ViewHeader,
    pub start_army_placement: ViewStartArmyPlacement,
    pub army_placement:ViewArmyPlacement,
    pub selected:Selected,
    pub combat:ViewCombat
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
        }
    }

    pub fn mount(&mut self) {
        self.header.mount();
        self.start_army_placement.mount();
        self.army_placement.mount();
       // self.combat.mount();
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
