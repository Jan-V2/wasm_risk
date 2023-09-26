use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{HtmlCanvasElement, MouseEvent};
use crate::map_data::get_map_data;


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
    pub players:Vec<Player>,
}

impl Model{
    pub fn new_from_json() -> Model{
        return Model{
            provinces: prov_array_from_json(),
            players: vec![],
        }
    }

    pub fn test_add_players(&mut self, count:i32){
        let colors = ["CadetBlue", "DarkOrchid", "DarkKhaki", "LimeGreen", "OrangeRed", "PeachPuff"];
        for i in 0..count{
            self.players.push(Player::new(i as u32, colors[i as usize].to_string()));
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
pub enum Card{
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
    pub card_type:Card,
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
            card_type: Card::Infantry,
        }
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
    pub card_type:Card,
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
        }
    }
}



pub struct Player{
    pub id:u32,
    pub cards:Vec<Card>,
    pub color:String,
}

impl Player {
    fn new(id:u32, color:String) -> Player{
        return Player {
            id,
            cards: vec![],
            color,
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
