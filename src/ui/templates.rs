

pub fn template_start_placement(id_player:&String, id_count:&String)->String{
    format!(r#"
    <div id="{id_player}"></div>
    <div id="{id_count}"></div>
    "#)
}


pub const HTML_ARMY_PLACEMENT:&str = r#"
<div id="place_army_count"></div>
"#;

pub const HTML_TURN_START:&str = r#"
<div id="from_string">
<div id="label1"></div>
"#;

pub const HTML_COMBAT:&str = r#"
<div id="from_string">
<div id="label1"></div>
"#;

pub const HTML_COMBAT_END:&str = r#"
<div id="from_string">
<div id="label1"></div>
"#;

pub const HTML_GAME_END:&str = r#"
<div id="from_string">
<div id="label1"></div>
"#;

