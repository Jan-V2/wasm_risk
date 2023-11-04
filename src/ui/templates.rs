

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


pub fn template_turn_start()->String{
    format!(r#"
<div id=""></div>
"#)
}

pub fn template_combat()->String{
    format!(r#"
<div id=""></div>
"#)
}

pub fn template_combat_end()->String{
    format!(r#"
<div id=""></div>
"#)
}

pub fn template_game_end()->String{
    format!(r#"
<div id=""></div>
"#)
}
