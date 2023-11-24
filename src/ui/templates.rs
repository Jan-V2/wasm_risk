use html_builder::*;
use std::fmt::Write;

pub fn template_start_army_placement(id_player:&String, id_count:&String) ->String{
    format!(r#"
    <div id="{id_player}"></div>
    <div id="{id_count}"></div>
    "#)
}


pub fn template_army_placement(id_label_count:&String)->String{
    format!(r#"
<div id="{id_label_count}"></div>
"#)
}

pub fn template_turn_menu(player_label:&String, btn_next_turn:&String)->String{

    let mut buf = Buffer::new();
    let mut main  = buf.div();
    writeln!(main.div().attr(
        fmt_style("margin-bottom: 15px;").as_str()).attr(fmt_id(player_label).as_str())
             , "Player:").unwrap();
    writeln!(main.button().attr(fmt_id(btn_next_turn).as_str()), "Next turn").unwrap();
    buf.finish()
}

pub fn template_dice_roll(id_canvases:&(String, String), next_btn:&String)->String{
    const CANVAS_WIDTH: u32 = 300;
    const CANVAS_HEIGHT: u32 = 100;
    let mut buf = Buffer::new();
    let mut main = buf.div();
    let mut attack = main.div().attr(fmt_style("margin-bottom: 15px;").as_str());
    writeln!(attack.h4(), "Attacker rolled" ).unwrap();
    let _ = attack.canvas().attr(format!("width='{CANVAS_WIDTH}'").as_str())
        .attr(format!("height='{CANVAS_HEIGHT}'").as_str())
        .attr(fmt_id(&id_canvases.0).as_str());
    let mut defend = main.div().attr(fmt_style("margin-bottom: 15px;").as_str());
    writeln!(defend.h4(), "Defender rolled" ).unwrap();
    let _ = defend.canvas().attr(format!("width='{CANVAS_WIDTH}'").as_str())
        .attr(format!("height='{CANVAS_HEIGHT}'").as_str())
        .attr(fmt_id(&id_canvases.1).as_str());
    writeln!(buf.button().attr(fmt_id(next_btn).as_str()), "Next").unwrap();

    buf.finish()
}



pub fn template_combat_menu(title:&String, location_text:&String, balance_text:&String,
                            select:&(String, String), player_text:&(String, String),
                            button_action:&(String, String), id_main:&(String, String)) ->String{
    let mut buf = Buffer::new();
    let mut main = buf.div().attr(fmt_id(title).as_str());

    writeln!(main.h3().attr(fmt_id(location_text).as_str()), "Attack in example").unwrap();
    writeln!(main.div().attr(fmt_id(balance_text).as_str()).attr(
        fmt_style("margin-bottom: 15px;").as_str()
    ), "balance example").unwrap();
    army_selector(&mut main, true, &id_main.0,
                  &select.0, &player_text.0, &button_action.0);
    army_selector(&mut main, false, &id_main.1,
                  &select.1, &player_text.1, &button_action.1);
    buf.finish()
}

pub fn army_selector(node:&mut Node, is_attack:bool, id_main:&String, id_select:&String, id_player_label:&String,
                    id_button:&String){
    let mut main = node.div().attr(fmt_style("margin-bottom: 30px;").as_str())
        .attr(fmt_id(id_main).as_str());
    writeln!(main.div().attr(fmt_id(&id_player_label).as_str())
                 .attr(fmt_style("margin-bottom: 10px").as_str()),
             "player example"
    ).unwrap();
    let mut body = main.div();

    if is_attack{
        writeln!(body.label().attr(fmt_style("float: left;white-space: break-spaces;").as_str())
                     .attr(format!("for={id_select}").as_str()),
                 "Attack with"
        ).unwrap()
    }else{
        writeln!(body.label().attr(fmt_style("float: left;white-space: break-spaces;").as_str())
                     .attr(format!("for={id_select}").as_str()),
                 "Defend with"
        ).unwrap()
    }

    let mut select = body.select().attr(fmt_style("float: left;white-space: break-spaces;").as_str())
        .attr(fmt_id(&id_select).as_str());


    add_option(&mut select, "1");
    add_option(&mut select, "2");
    add_option(&mut select, "3");
    
    writeln!(body.div().attr(fmt_style("white-space: break-spaces;").as_str()), "armies").unwrap();
    let mut footer = main.div();
    let _ = footer.div().attr(fmt_style("height: 5px;").as_str());
    if is_attack{
        writeln!(footer.button().attr(fmt_id(&id_button).as_str()), "Attack").unwrap()
    }else {
        writeln!(footer.button().attr(fmt_id(&id_button).as_str()), "Defend").unwrap()
    }
}

fn add_option(node:&mut Node, value:&str){
    writeln!(node.option().attr(fmt_value(value.to_string()).as_str()), "{}", value).unwrap();
}

fn fmt_id(s: &String)->String{
    format!("id='{s}'")
}

fn fmt_style(s: &str)->String{
    format!("style='{s}'")
}

fn fmt_value(s: String)->String{
    format!("value='{s}'")
}
