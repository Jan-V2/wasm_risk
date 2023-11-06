use std::fmt;
use std::fmt::Formatter;
use queues::{IsQueue, Queue};
use serde::{Deserialize, Serialize};
use crate::data_include::{ get_map_data, get_navtree_data};


pub struct Model{
    pub provinces:Vec<Province>,
    pub nav_tree:NavTree,
    pub players:Vec<Player>,
    pub rules:Rules,
}

impl Model{
    pub fn new_from_json() -> Model{
        return Model{
            provinces:serde_json::from_str(&get_map_data()).unwrap(),
            nav_tree: serde_json::from_str(&get_navtree_data()).unwrap(),
            players: vec![],
            rules: Rules {},
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

    pub fn get_prov_name_from_id(&self, prov_id:&u32) ->String{
        let prov = self.get_prov_from_id(prov_id);
        if prov.is_some(){
            return prov.unwrap().name.clone();
        }else{
            "prov not found".to_string()
        }
    }
}

pub struct CombatData {
    pub armies_attacker:u32,
    pub armies_defender:u32,
    pub losses_defender:u32,
    pub losses_attacker:u32,
    pub dice_attacker:Vec<u32>,
    pub dice_defender:Vec<u32>,
    pub combat_finished:bool,
}

pub struct CombatEngine{
}

impl CombatEngine{
    pub fn new()->Self{
        return CombatEngine{}
    }

    fn roll_dice(&self)->u32{
        (js_sys::Math::random() * 5f64).round() as u32 +1
    }

    pub fn next_round(&mut self, attacking_armies:u32, defending_armies:u32,
                      attack_armies_active:u32, defence_armies_active:u32 ) -> CombatData {
        if attack_armies_active > attacking_armies
            || defence_armies_active > defending_armies{
            panic!("Incorrect number of attackers/defenders attacking {} out of {} defending {} out of {}",
                   attack_armies_active, attacking_armies, defence_armies_active, defending_armies)
        }

        let swap_indexes = |dice:&mut Vec<u32>, idx:usize|{
            let temp = dice[idx];
            dice[idx] = dice[idx +1];
            dice[idx+1] = temp;
        };

        let sort_dice = |dice:&mut Vec<u32>|{
            let len = dice.len();
            if len > 1{
                if dice[0] > dice[1]{
                    swap_indexes(dice, 0)
                }
            }
            if len > 2{
                if dice[1] > dice[2]{
                    swap_indexes(dice, 1)
                }
            }
        };

        let mut attacking_dice:Vec<u32> = vec![];
        let mut defending_dice:Vec<u32> = vec![];
        for _ in 0..attack_armies_active {
            attacking_dice.push(self.roll_dice())
        }
        sort_dice(&mut attacking_dice);
        for _ in 0..defending_dice.len(){
            attacking_dice.push(self.roll_dice())
        }
        sort_dice(&mut defending_dice);

        let mut ret = CombatData {
            armies_attacker: attacking_armies,
            armies_defender: defending_armies,
            losses_defender: 0,
            losses_attacker: 0,
            dice_attacker: attacking_dice,
            dice_defender: defending_dice,
            combat_finished: false,
        };

        for i in 0..ret.dice_defender.len(){
            if ret.dice_attacker[i] > ret.dice_defender[i]{
                ret.losses_defender += 1;
            }else {
                ret.losses_attacker += 1;
            }
        }
        ret.armies_attacker -= ret.losses_attacker;
        ret.armies_defender -= ret.losses_defender;
        ret
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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



#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TerritoryCardType {
    Infantry,
    Artillery,
    Cavalry
}

#[derive(Debug, Serialize, Deserialize)]
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

type ProvId = u32;

#[derive(Debug, Serialize, Deserialize)]
pub struct NavTree{
    nav_nodes:Vec<NavNode>,
    pub adding_id_set:bool,
    adding_to:u32,
    currently_selected:ProvId, // make this a &province rather than id?
    pub selection_active:bool,
}

#[allow(dead_code)]
impl NavTree {
    pub fn new()->NavTree{
        NavTree{
            nav_nodes: vec![],
            adding_id_set: false,
            adding_to: 0,
            currently_selected: 0,
            selection_active: false,
        }
    }

    pub fn select_prov(&mut self, prov_id:u32){
        self.currently_selected = prov_id;
        self.selection_active = true
    }

    pub fn deselect(&mut self){
        self.selection_active = false
    }

    fn validate_nav(&self, to:u32)->bool{
        if !self.selection_active{
            gloo::console::log!("can't navigate, no selection");
            false;
        }
        if self.currently_selected == to{
            gloo::console::log!("can't navigate, to and from are the same");
            false;
        }
        true
    }

    pub fn navigate_adjacent(&self, to:u32) ->Option<bool>{
        if !self.validate_nav(to){
            return None
        }
        return Some(self.get_node_from_id(&self.currently_selected).unwrap().connections.contains(&to));
    }

  //  #[allow(unused_variables)]
    pub fn navigate_move(&self, to:u32, provs:&Vec<Province>) -> Option<bool>{
        if !self.validate_nav(to){
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
        let target_owner = get_prov_with_id(&self.currently_selected).unwrap().owner_id;
        let mut visit_q:Queue<&Province> = Queue::new();
        let _ = visit_q.add(get_prov_with_id(&self.currently_selected).unwrap());

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




