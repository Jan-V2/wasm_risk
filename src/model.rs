use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{HtmlCanvasElement, MouseEvent};


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


pub fn get_map_data() -> String{
    let str = r#"[{"name":"none","id":0,"army_count":0,"location":{"x":68,"y":57},"continent":"Africa"},{"name":"none","id":1,"army_count":0,"location":{"x":113,"y":62},"continent":"Africa"},{"name":"none","id":2,"army_count":0,"location":{"x":109,"y":92},"continent":"Africa"},{"name":"none","id":3,"army_count":0,"location":{"x":162,"y":97},"continent":"Africa"},{"name":"none","id":4,"army_count":0,"location":{"x":221,"y":92},"continent":"Africa"},{"name":"none","id":5,"army_count":0,"location":{"x":309,"y":34},"continent":"Africa"},{"name":"none","id":6,"army_count":0,"location":{"x":104,"y":127},"continent":"Africa"},{"name":"none","id":7,"army_count":0,"location":{"x":153,"y":151},"continent":"Africa"},{"name":"none","id":8,"army_count":0,"location":{"x":97,"y":191},"continent":"Africa"},{"name":"none","id":9,"army_count":0,"location":{"x":179,"y":251},"continent":"Africa"},{"name":"none","id":10,"army_count":0,"location":{"x":218,"y":301},"continent":"Africa"},{"name":"none","id":11,"army_count":0,"location":{"x":192,"y":325},"continent":"Africa"},{"name":"none","id":12,"army_count":0,"location":{"x":207,"y":387},"continent":"Africa"},{"name":"none","id":13,"army_count":0,"location":{"x":380,"y":195},"continent":"Africa"},{"name":"none","id":14,"army_count":0,"location":{"x":439,"y":182},"continent":"Africa"},{"name":"none","id":15,"army_count":0,"location":{"x":483,"y":236},"continent":"Africa"},{"name":"none","id":16,"army_count":0,"location":{"x":449,"y":288},"continent":"Africa"},{"name":"none","id":17,"army_count":0,"location":{"x":451,"y":340},"continent":"Africa"},{"name":"none","id":18,"army_count":0,"location":{"x":519,"y":343},"continent":"Africa"},{"name":"none","id":19,"army_count":0,"location":{"x":353,"y":59},"continent":"Africa"},{"name":"none","id":20,"army_count":0,"location":{"x":386,"y":95},"continent":"Africa"},{"name":"none","id":21,"army_count":0,"location":{"x":424,"y":69},"continent":"Africa"},{"name":"none","id":22,"army_count":0,"location":{"x":425,"y":101},"continent":"Africa"},{"name":"none","id":23,"army_count":0,"location":{"x":473,"y":81},"continent":"Africa"},{"name":"none","id":24,"army_count":0,"location":{"x":447,"y":126},"continent":"Africa"},{"name":"none","id":25,"army_count":0,"location":{"x":395,"y":115},"continent":"Africa"},{"name":"none","id":26,"army_count":0,"location":{"x":505,"y":160},"continent":"Africa"},{"name":"none","id":27,"army_count":0,"location":{"x":550,"y":117},"continent":"Africa"},{"name":"none","id":28,"army_count":0,"location":{"x":558,"y":74},"continent":"Africa"},{"name":"none","id":29,"army_count":0,"location":{"x":607,"y":51},"continent":"Africa"},{"name":"none","id":30,"army_count":0,"location":{"x":679,"y":61},"continent":"Africa"},{"name":"none","id":31,"army_count":0,"location":{"x":728,"y":66},"continent":"Africa"},{"name":"none","id":32,"army_count":0,"location":{"x":766,"y":149},"continent":"Africa"},{"name":"none","id":33,"army_count":0,"location":{"x":659,"y":122},"continent":"Africa"},{"name":"none","id":34,"army_count":0,"location":{"x":650,"y":158},"continent":"Africa"},{"name":"none","id":35,"army_count":0,"location":{"x":603,"y":191},"continent":"Africa"},{"name":"none","id":36,"army_count":0,"location":{"x":533,"y":165},"continent":"Africa"},{"name":"none","id":37,"army_count":0,"location":{"x":678,"y":219},"continent":"Africa"},{"name":"none","id":38,"army_count":0,"location":{"x":712,"y":280},"continent":"Africa"},{"name":"none","id":39,"army_count":0,"location":{"x":797,"y":295},"continent":"Africa"},{"name":"none","id":40,"army_count":0,"location":{"x":730,"y":375},"continent":"Africa"},{"name":"none","id":41,"army_count":0,"location":{"x":792,"y":360},"continent":"Africa"}]"#;
    return str.to_string()
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
    pub location:Coord,
    pub continent:Continent,
}

impl Province{
    pub fn from_i32_pair(x:i32, y:i32) -> Province{
        return Province{
            name: "none".to_string(),
            id:0,
            army_count: 0,
            location:Coord::new(x, y),
            continent:Continent::Africa,
        }
    }
}




pub struct Player{
    pub id:u32,
    pub owned_provs:Vec<Province>,
    pub cards:Vec<Card>,
}

pub struct Model{
    pub provinces:Vec<Province>
}

impl Model{
    pub fn new_from_json() -> Model{
        return Model{
            provinces: prov_array_from_json()
        }
    }
}



pub fn prov_array_from_json() -> Vec<Province>{
    return serde_json::from_str(&get_map_data()).unwrap();
}

pub fn prov_array_to_json(prov_array: &Vec<Province>){
    console_log!("{}",  serde_json::to_string(prov_array).unwrap())
}
