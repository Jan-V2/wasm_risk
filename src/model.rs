use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{HtmlCanvasElement, MouseEvent};
use crate::data_include::{get_colors_array, get_map_data, get_navtree_data};


#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}



pub struct Model{
    pub provinces:Vec<Province>,
    pub nav_tree:NavTree,
    pub players:Vec<Player>,
}

impl Model{
    pub fn new_from_json() -> Model{
        return Model{
            provinces:serde_json::from_str(&get_map_data()).unwrap(),
            nav_tree: serde_json::from_str(&get_navtree_data()).unwrap(),
            players: vec![],
        }
    }

    pub fn test_add_players(&mut self, count:i32){
        let colors = get_colors_array();
        for i in 0..count{
            self.players.push(Player::new(i as u32, colors[i as usize].to_string(), false));
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coord{
    pub x:i32,
    pub y:i32
}


impl Coord {
    pub fn new(x:i32, y:i32) -> Coord{
        return Coord{
            x,
            y
        }
    }

    pub fn from_canvas_event( canvas:HtmlCanvasElement, event:MouseEvent) -> Coord{
        return Coord{
            x:event.x() - canvas.offset_left(),
            y:event.y() - canvas.offset_top()
        }
    }
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
    pub fn from_i32_pair(x:i32, y:i32) -> Province{
        return Province{
            name: "none".to_string(),
            id:0,
            army_count: 0,
            owner_id: 100,
            location:Coord::new(x, y),
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
    adding_to:u32,
}

impl NavTree {
    pub fn new()->NavTree{
        NavTree{
            nav_nodes: vec![],
            adding_id_set: false,
            adding_to: 0,
        }
    }
    
    pub fn navigate_adjacent(&self, from:u32, to:u32) -> Option<bool>{
        for node in &self.nav_nodes{
            if node.id == from{
                return Some(node.connections.contains(&to));
            }
        }
        None
    }

    #[allow(unused_variables)]
    pub fn navigate_move(&self, from:u32, to:u32, provs:&Vec<Province>) -> bool{
        todo!("impl dykstra and check if provs are owned by same person");
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
        let get_node = |id:&u32|{
            for node in &self.nav_nodes{
                if node.id == *id{
                    return Some(node);
                }
            }
            None
        };

        for node in &self.nav_nodes{
            for id in &node.connections{
                if !get_node(id).unwrap().connections.contains(&node.id){
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

}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewProvince{
    pub name:String,
    pub id:u32,
    pub owner_id:u32,
    pub army_count:u32,
    pub location:Coord,
    pub continent:Continent,
    pub card_type: TerritoryCardType,
    pub connections:Vec<u32>,
}

impl NewProvince{
    pub fn from_prov(p:&Province)-> NewProvince{
        return NewProvince{
            name: p.name.clone(),
            id: p.id,
            owner_id: p.owner_id,
            army_count: p.army_count,
            location: p.location.clone(),
            continent: p.continent.clone(),
            card_type: p.card_type.clone(),
            connections: vec![],
        }
    }
}



pub struct Player{
    pub id:u32,
    pub cards:Vec<TerritoryCardType>,
    pub color:String,
    pub is_computer:bool,
}

impl Player {
    fn new(id:u32, color:String, is_computer:bool) -> Player{
        return Player{
            id,
            cards: vec![],
            color,
            is_computer,
        }
    }

    fn get_owned_provs<'a>(&self, provs: &'a Vec<Province>) -> Vec<&'a Province>{
        let mut ret:Vec<&Province> = Vec::new();
        for prov in provs{
            if prov.owner_id == self.id{
                ret.push(prov)
            }
        }
        ret
    }
}



pub fn prov_array_from_json() -> Vec<Province>{
    return serde_json::from_str(&get_map_data()).unwrap();
}

pub fn prov_array_to_json(prov_array: &Vec<Province>){
    console_log!("{}",  serde_json::to_string(prov_array).unwrap())
}

pub fn new_prov_array_to_json(prov_array: &Vec<NewProvince>){
    console_log!("{}",  serde_json::to_string(prov_array).unwrap())
}
