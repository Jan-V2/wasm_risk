use std::fmt;
use std::fmt::Formatter;
use web_sys::{HtmlCanvasElement, MouseEvent};

#[derive(Debug)]
pub struct Coord{
    x:i32,
    y:i32
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

    pub fn to_json(&self) -> serde_json::Value {
        return serde_json::json!({
            "x": self.x,
            "y": self.y
        })
    }
}

impl fmt::Display for Coord{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return write!(f, "{{ x:{}, y:{} }}", self.x, self.y);
    }
}

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

pub enum Card{
    Infantry,
    Artillery,
    Cavalry
}


pub(crate) struct Province{
    name:String,
    id:u32,
    location:Coord,
    continent:Continent,
}

impl Province{
    pub fn from_i32_pair(x:i32, y:i32) -> Province{
        return Province{
            name: "none".to_string(),
            id:0,
            location:Coord::new(x, y),
            continent:Continent::Africa,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        return serde_json::json!({
            "name": self.name,
            "id": self.id,
            "location": self.location.to_json(),
            "continent": self.continent.to_string()
        })
    }
}

struct Player{
    id:u32,
    owned_provs:Vec<Province>,
    cards:Vec<Card>,
}

struct Model{

}
