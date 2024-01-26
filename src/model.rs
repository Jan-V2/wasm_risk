use std::fmt;
use std::fmt::Formatter;
use queues::{IsQueue, Queue};
use serde::{Deserialize, Serialize};
use crate::data_include::{ get_map_data, get_navtree_data};
use gloo::console::log as console_log;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::utils::structs::AttackDefendPair;

//todo refactor this file
pub struct Model{
    pub provinces:Vec<Province>,
    pub nav_tree:NavTree,
    pub players:Vec<Player>,
    pub rules:Rules,
    pub combat_engine:CombatEngine,
}

impl Model{
    pub fn new_from_json() -> Model{
        return Model{
            provinces:serde_json::from_str(&get_map_data()).unwrap(),
            nav_tree: serde_json::from_str(&get_navtree_data()).unwrap(),
            players: vec![],
            rules: Rules {},
            combat_engine: CombatEngine {},
        }
    }


    pub fn get_prov_from_id_mut(&mut self, prov_id:&u32) -> Option< &mut Province>{
        for prov in &mut self.provinces{
            if prov.id == *prov_id{
                return Some(prov);
            }
        }
        None
    }

    pub fn get_prov_from_id(&self, prov_id:&u32) -> Option< &Province>{
        for prov in &self.provinces{
            if prov.id == *prov_id{
                return Some(prov);
            }
        }
        None
    }

    pub fn set_prov(&mut self, new_prov:Province){
        for i in 0..self.provinces.len(){
            if self.provinces[i].id == new_prov.id{
                self.provinces[i] = new_prov;
                return;
            }
        }
        panic!("prov not found in prov array")
    }

    pub fn get_owner_from_prov_id(&self, prov_id:&u32)-> Option<u32>{
        let prov = self.get_prov_from_id(prov_id);
        if prov.is_some(){
            return Some(prov.unwrap().owner_id)
        }
        None
    }

    pub fn get_name_from_prov_id(&self, prov_id:&u32)  -> Option<String>{
        let prov = self.get_prov_from_id(&prov_id);
        if prov.is_some(){
            return Some(prov.unwrap().name.clone());
        }
        None
    }

    pub fn get_prov_count_owned_by_player(&self, player_id: u32)->u32{
        let mut ret = 0;
        for prov in &self.provinces{
            if prov.owner_id == player_id{
                ret +=1;
            }
        }
        return ret;
    }

    pub fn get_player_armies_reinforce(&self, player_id:u32)->u32{
        // the number of armies this player get's if they reinforce
        let player_prov_num =
            self.get_prov_count_owned_by_player(player_id);
        let mut reinforcing_army_count = player_prov_num / 3;
        if reinforcing_army_count < 3 {
            reinforcing_army_count = 3;
        }
        return reinforcing_army_count;
    }

    pub fn get_player_continent_armies(&self, player_id:&u32) -> u32 {
        let mut acc = 0;
        let mut found_continents:Vec<_> = Continent::iter().collect();
        for prov in &self.provinces{
            if &prov.owner_id != player_id{
                let found_idx_opt = found_continents.iter()
                    .position(|c| c == &prov.continent);
                if found_idx_opt.is_some(){
                    let _ = found_continents.remove(found_idx_opt.unwrap());
                }
            }
        }
        for c in found_continents{
            acc += c.get_armies();
        }
        return acc;
    }

    pub fn get_player_count(&self)->u32{
        self.players.len() as u32
    }
}

#[derive(Clone, Debug, Default)]
pub struct CombatState { //todo move somewhere sensible
    pub armies:AttackDefendPair<u32>,
    pub losses:AttackDefendPair<u32>,
    pub dice_rolls:AttackDefendPair<Vec<u32>>,
    pub num_dice_used:AttackDefendPair<u32>,
    pub prov_id:AttackDefendPair<u32>,
    pub combat_ongoing:bool,
}

impl CombatState {
    pub(crate) fn apply_losses(&mut self) {
        self.armies.attack -= self.losses.attack;
        self.armies.defend -= self.losses.defend;
    }

    pub(crate) fn get_remaining(&self)->AttackDefendPair<u32>{
        AttackDefendPair{
            attack: self.armies.attack - self.losses.attack,
            defend: self.armies.defend - self.losses.defend,
        }
    }

    pub(crate) fn attacker_has_won(&self)->bool{
        let armies = self.get_remaining();
        return if armies.attack > 1 && armies.defend == 0{
            true
        }else {
            false
        }
    }

    pub(crate) fn combat_can_continue(&self)->bool{
        todo!()
    }
}

pub struct CombatEngine{
}

impl CombatEngine{
    fn roll_dice(&self)->u32{
        (js_sys::Math::random() * 5f64).round() as u32 +1
    }

    pub fn next_round(&mut self, mut combat_data: CombatState) -> CombatState {
        assert!(
            combat_data.combat_ongoing == false &&
            combat_data.losses.defend == 0 &&
            combat_data.losses.attack == 0
        );

        let swap_indexes = |dice:&mut Vec<u32>, idx:usize|{
            let temp = dice[idx];
            dice[idx] = dice[idx +1];
            dice[idx+1] = temp;
        };

        let sort_dice = |dice:&mut Vec<u32>|{
            let len = dice.len();
            for _ in 0..2{
                if len > 2{
                    if dice[1] < dice[2]{
                        swap_indexes(dice, 1)
                    }
                }
                if len > 1{
                    if dice[0] < dice[1]{
                        swap_indexes(dice, 0)
                    }
                }
            }
        };

        let mut attacking_dice:Vec<u32> = vec![];
        let mut defending_dice:Vec<u32> = vec![];
        for _ in 0..combat_data.num_dice_used.attack {
            attacking_dice.push(self.roll_dice())
        }
        sort_dice(&mut attacking_dice);
        for _ in 0..combat_data.num_dice_used.defend{
            defending_dice.push(self.roll_dice())
        }
        sort_dice(&mut defending_dice);
        
        combat_data.dice_rolls.attack = attacking_dice;
        combat_data.dice_rolls.defend = defending_dice;

        console_log!(format!("{:?}", combat_data));
        let dice_min = if combat_data.num_dice_used.attack > combat_data.num_dice_used.defend {
            combat_data.num_dice_used.defend as usize
        }else {
            combat_data.num_dice_used.attack as usize
        };

        for i in 0..dice_min{
            if combat_data.dice_rolls.attack[i] > combat_data.dice_rolls.defend[i]{
                combat_data.losses.defend += 1;
            }else {
                combat_data.losses.attack += 1;
            }
        }

        let attacking_signed = combat_data.armies.attack as i32 -
            combat_data.losses.attack as i32 ;
        if attacking_signed > -1{
            combat_data.armies.attack = attacking_signed as u32
        }else {
            combat_data.armies.attack = 0;
        }
        let defending_signed = combat_data.armies.defend as i32 -
            combat_data.losses.defend as i32;
        if defending_signed > -1{
            combat_data.armies.defend= defending_signed as u32
        }else {
            combat_data.armies.defend = 0;
        }

        if combat_data.armies.attack == 0 || combat_data.armies.defend == 0 {
            combat_data.combat_ongoing = true;
        }
        combat_data    
    }
}

pub struct Rules{}

impl Rules {
    pub fn armies_per_players_start(num_players:u32)->Option<u32> {
        match num_players {
            0 | 1 => None,
            2 => Some(40),
            3 => Some(35),
            4 => Some(30),
            5 => Some(25),
            6 => Some(20),
            _ => None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coord{
    pub x:i32,
    pub y:i32
}


impl fmt::Display for Coord{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return write!(f, "{{ x:{}, y:{} }}", self.x, self.y);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, EnumIter, PartialEq, Eq)]
pub enum Continent {
    Africa,
    SAmerica,
    NAmerica,
    Europe,
    Asia,
    Australia
}

impl fmt::Display for Continent{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self{
            Continent::Africa => write!(f, "Africa"),
            Continent::SAmerica => write!(f, "South america"),
            Continent::Europe => write!(f, "Europe"),
            Continent::NAmerica => write!(f, "North america"),
            Continent::Asia => write!(f, "Asia"),
            Continent::Australia => write!(f, "Australia"),
        }
    }
}

impl Continent{//todo move this to rules?
    pub fn get_armies(&self)->u32{
        match self {
            Continent::Africa => {3}
            Continent::SAmerica => {2}
            Continent::NAmerica => {5}
            Continent::Europe => {5}
            Continent::Asia => {7}
            Continent::Australia => {2}
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TerritoryCardType {
    Infantry,
    Artillery,
    Cavalry
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Province{
    pub name:String,
    pub id:u32,
    pub army_count:u32,
    pub owner_id:u32,
    pub location:Coord,
    pub continent:Continent,
    pub card_type: TerritoryCardType,
}

impl Province{
    #[allow(dead_code)]
    pub fn from_i32_pair(x:i32, y:i32) -> Province{
        return Province{
            name: "none".to_string(),
            id:0,
            army_count: 0,
            owner_id: 100,
            location:Coord{ x, y, },
            continent:Continent::Africa,
            card_type: TerritoryCardType::Infantry,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavNode{
    id:u32,
    connections:Vec<u32>,
}

impl NavNode {
    #[allow(dead_code)]
    pub fn new(id:u32)->NavNode{
        NavNode{
            id,
            connections: vec![],
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NavTree{
    nav_nodes:Vec<NavNode>,
    pub adding_id_set:bool,
    adding_to:u32
}

#[allow(dead_code)]
impl NavTree {
    pub fn new()->NavTree {
        NavTree {
            nav_nodes: vec![],
            adding_id_set: false,
            adding_to: 0,
        }
    }

    fn validate_nav(&self, to:&u32, from:&u32)->bool{
        if from == to{
            gloo::console::log!("can't navigate, to and from are the same");
            false;
        }
        true
    }

    pub fn navigate_adjacent(&self,  to:u32, from:u32) ->Option<bool>{
        if !self.validate_nav(&to, &from){
            return None
        }
        return Some(self.get_node_from_id(
            &from).unwrap().connections.contains(&to));
    }

    pub fn navigate_move(&self, to:u32, from:u32, provs:&Vec<Province>) -> Option<bool>{
        if !self.validate_nav(&to, &from){
            return None
        }
        let mut visited:Vec<u32> = Vec::new();
        let get_prov_with_id = |id:&u32|{
            for prov in provs{
                if prov.id == *id{
                    return Some(prov);
                }
            }
            None
        };
        let target_owner = get_prov_with_id(&from).unwrap().owner_id;
        let mut visit_q:Queue<&Province> = Queue::new();
        let _ = visit_q.add(get_prov_with_id(&from).unwrap());

        while visit_q.size() > 0{
            let curr_prov_id = visit_q.remove().unwrap().id;
            visited.push(curr_prov_id);
            let adjacent = self.get_node_from_id(&curr_prov_id).unwrap();
            let next: Vec<_> = adjacent.connections.iter().filter(|prov_id|{
                !visited.contains(prov_id)
            }).filter(|prov_id|{
                get_prov_with_id(prov_id).unwrap().owner_id == target_owner
            }).collect();
            for id in next{
                if *id == to{
                    return Some(true);
                }else{
                    let _ = visit_q.add(get_prov_with_id(&id).unwrap());
                }
            }
        }
        Some(false)
    }

    pub fn add_node(&mut self, id:u32){
        if self.adding_id_set{
            gloo::console::log!(format!("already adding to id {}", self.adding_to ));
            return;
        }
        for node in &self.nav_nodes{
            if node.id == id{
                gloo::console::log!(" can't add node is already in list");
                return;
            }
        }
        self.nav_nodes.push(NavNode::new(id));
        self.adding_id_set = true;
        self.adding_to = id;
        gloo::console::log!("added new node")
    }

    pub fn add_connection(&mut self, dest:u32){
        if self.adding_to == dest {
            gloo::console::log!("add failed to and from are the same");
            return;
        }
        if !(self.adding_id_set) {
            gloo::console::log!("adding id not set");
            return;
        }
        for node in  &mut self.nav_nodes{
            if node.id == self.adding_to{
                if !node.connections.contains(&dest){
                    node.connections.push(dest);
                    gloo::console::log!(format!("added path from {} to {}", self.adding_to, dest));
                    return;
                }else {
                    gloo::console::log!("dest is already in node");
                }
            }
        }
        gloo::console::log!(format!("add failed could not find id {} in nav tree", self.adding_to))
    }

    pub fn verify_self(&self){
        for node in &self.nav_nodes{
            for id in &node.connections{
                if !self.get_node_from_id(id).unwrap().connections.contains(&node.id){
                    gloo::console::log!(format!("node {} is missing connections", node.id));
                    return;
                }
            }
        }
        gloo::console::log!("success")
    }

    pub fn end_add(&mut self){
        self.adding_id_set = false;
        gloo::console::log!(format!("finished adding to {}", self.adding_to))
    }

    fn get_node_from_id(&self, id:&u32)->Option<&NavNode>{
        for node in &self.nav_nodes{
            if node.id == *id{
                return Some(node);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Player{
    pub id:u32,
    pub cards:Vec<TerritoryCardType>,
    pub color:String,
    pub is_computer:bool,
}

impl Player {
    #[allow(dead_code)]
    fn new(id:u32, color:String, is_computer:bool) -> Player{
        return Player{
            id,
            cards: vec![],
            color,
            is_computer,
        }
    }

    pub fn get_owned_provs<'a>(&self, provs: &'a Vec<Province>) -> Vec<&'a Province>{
        let mut ret:Vec<&Province> = Vec::new();
        for prov in provs{
            if prov.owner_id == self.id{
                ret.push(prov)
            }
        }
        ret
    }
}




