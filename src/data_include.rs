pub fn get_map_data() -> String {
    let data = r#"[
  {
    "name": "Alaska",
    "id": 0,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 68,
      "y": 57
    },
    "continent": "NAmerica",
    "card_type": "Infantry"
  },
  {
    "name": "Northwest Territory",
    "id": 1,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 113,
      "y": 62
    },
    "continent": "NAmerica",
    "card_type": "Artillery"
  },
  {
    "name": "Alberta",
    "id": 2,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 109,
      "y": 92
    },
    "continent": "NAmerica",
    "card_type": "Cavalry"
  },
  {
    "name": "Ontario",
    "id": 3,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 162,
      "y": 97
    },
    "continent": "NAmerica",
    "card_type": "Cavalry"
  },
  {
    "name": "Quebec",
    "id": 4,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 221,
      "y": 92
    },
    "continent": "NAmerica",
    "card_type": "Cavalry"
  },
  {
    "name": "Greenland",
    "id": 5,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 309,
      "y": 34
    },
    "continent": "NAmerica",
    "card_type": "Cavalry"
  },
  {
    "name": "Western United States",
    "id": 6,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 104,
      "y": 127
    },
    "continent": "NAmerica",
    "card_type": "Artillery"
  },
  {
    "name": "Eastern United States",
    "id": 7,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 153,
      "y": 151
    },
    "continent": "NAmerica",
    "card_type": "Artillery"
  },
  {
    "name": "Central America",
    "id": 8,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 97,
      "y": 191
    },
    "continent": "NAmerica",
    "card_type": "Artillery"
  },
  {
    "name": "Venezuela",
    "id": 9,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 179,
      "y": 251
    },
    "continent": "SAmerica",
    "card_type": "Infantry"
  },
  {
    "name": "Brazil",
    "id": 10,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 218,
      "y": 301
    },
    "continent": "SAmerica",
    "card_type": "Artillery"
  },
  {
    "name": "Peru",
    "id": 11,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 192,
      "y": 325
    },
    "continent": "SAmerica",
    "card_type": "Infantry"
  },
  {
    "name": "Argentina",
    "id": 12,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 207,
      "y": 387
    },
    "continent": "SAmerica",
    "card_type": "Infantry"
  },
  {
    "name": "North Africa",
    "id": 13,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 380,
      "y": 195
    },
    "continent": "Africa",
    "card_type": "Cavalry"
  },
  {
    "name": "Egypt",
    "id": 14,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 439,
      "y": 182
    },
    "continent": "Africa",
    "card_type": "Infantry"
  },
  {
    "name": "East Africa",
    "id": 15,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 483,
      "y": 236
    },
    "continent": "Africa",
    "card_type": "Infantry"
  },
  {
    "name": "Congo",
    "id": 16,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 449,
      "y": 288
    },
    "continent": "Africa",
    "card_type": "Infantry"
  },
  {
    "name": "South Africa",
    "id": 17,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 451,
      "y": 340
    },
    "continent": "Africa",
    "card_type": "Artillery"
  },
  {
    "name": "Madagascar",
    "id": 18,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 519,
      "y": 343
    },
    "continent": "Africa",
    "card_type": "Cavalry"
  },
  {
    "name": "Iceland",
    "id": 19,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 353,
      "y": 59
    },
    "continent": "Europe",
    "card_type": "Infantry"
  },
  {
    "name": "Great Britain & Ireland",
    "id": 20,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 386,
      "y": 95
    },
    "continent": "Europe",
    "card_type": "Artillery"
  },
  {
    "name": "Scandinavia",
    "id": 21,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 424,
      "y": 69
    },
    "continent": "Europe",
    "card_type": "Cavalry"
  },
  {
    "name": "Central Europe",
    "id": 22,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 425,
      "y": 101
    },
    "continent": "Europe",
    "card_type": "Cavalry"
  },
  {
    "name": "Eastern Europe",
    "id": 23,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 473,
      "y": 81
    },
    "continent": "Europe",
    "card_type": "Cavalry"
  },
  {
    "name": "Southern Europe",
    "id": 24,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 447,
      "y": 126
    },
    "continent": "Europe",
    "card_type": "Artillery"
  },
  {
    "name": "Western Europe",
    "id": 25,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 395,
      "y": 115
    },
    "continent": "Europe",
    "card_type": "Artillery"
  },
  {
    "name": "Middle East",
    "id": 26,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 505,
      "y": 160
    },
    "continent": "Asia",
    "card_type": "Infantry"
  },
  {
    "name": "Afghanistan",
    "id": 27,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 550,
      "y": 117
    },
    "continent": "Asia",
    "card_type": "Cavalry"
  },
  {
    "name": "Ural",
    "id": 28,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 558,
      "y": 74
    },
    "continent": "Asia",
    "card_type": "Cavalry"
  },
  {
    "name": "Siberia",
    "id": 29,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 607,
      "y": 51
    },
    "continent": "Asia",
    "card_type": "Cavalry"
  },
  {
    "name": "Yakutsk",
    "id": 30,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 679,
      "y": 61
    },
    "continent": "Asia",
    "card_type": "Cavalry"
  },
  {
    "name": "Kamchatka",
    "id": 31,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 728,
      "y": 66
    },
    "continent": "Asia",
    "card_type": "Infantry"
  },
  {
    "name": "Japan",
    "id": 32,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 766,
      "y": 149
    },
    "continent": "Asia",
    "card_type": "Artillery"
  },
  {
    "name": "Mongolia",
    "id": 33,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 659,
      "y": 122
    },
    "continent": "Asia",
    "card_type": "Infantry"
  },
  {
    "name": "China",
    "id": 34,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 650,
      "y": 158
    },
    "continent": "Asia",
    "card_type": "Infantry"
  },
  {
    "name": "India",
    "id": 35,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 603,
      "y": 191
    },
    "continent": "Asia",
    "card_type": "Cavalry"
  },
  {
    "name": "Irkutsk",
    "id": 36,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 653,
      "y": 86
    },
    "continent": "Asia",
    "card_type": "Cavalry"
  },
  {
    "name": "Southeast Asia",
    "id": 37,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 678,
      "y": 219
    },
    "continent": "Asia",
    "card_type": "Infantry"
  },
  {
    "name": "Indonesia",
    "id": 38,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 712,
      "y": 280
    },
    "continent": "Australia",
    "card_type": "Artillery"
  },
  {
    "name": "New Guinea",
    "id": 39,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 797,
      "y": 295
    },
    "continent": "Africa",
    "card_type": "Infantry"
  },
  {
    "name": "Western Australia",
    "id": 40,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 730,
      "y": 375
    },
    "continent": "Australia",
    "card_type": "Artillery"
  },
  {
    "name": "Eastern Australia",
    "id": 41,
    "owner_id": 100,
    "army_count": 0,
    "location": {
      "x": 792,
      "y": 360
    },
    "continent": "Australia",
    "card_type": "Artillery"
  }
]"#;
    return data.to_string();
}


pub fn get_setup_player_select() -> String{
    // todo format
    let data = r#"        <label id="side_menu_header" >Setup</label>
        <div id="side_menu_inner">
        <select class="form-select" style="width: fit-content">
            <option>Choose the number of players</option>
            <option value="1">1</option>
            <option value="2">2</option>
            <option value="3">3</option>
            <option value="4">4</option>
            <option value="5">5</option>
            <option value="6">6</option>
        </select>
        </div>
"#;

    return data.to_string()
}


pub fn get_side_menu_setup_footer() -> String{
    let data = r#"        <div id="side_menu_setup_footer">
            <button id="next_btn" type="button" class="btn btn-primary" style="margin-top: 10px; margin-bottom: 5px;">
                Next
            </button>
            <div id="side_menu_error">
                You must select an amount of players to continue.
            </div>
        </div>
    </div>"#;

    return data.to_string()
}

pub fn get_player_setup(color_array:&Vec<String>, player_num:&u32) -> String{
    let mut out:String = "".to_string();

    for color in color_array{
        out = format!(r#"{out} <option value='#{color}'>{color}</option>"#);
    }

    out = format!(r#"            <label>Player {player_num}</label>
            <select class="form-select" style="width: fit-content">
                <option>Choose color</option>
                {out}
            </select>
            <input type="checkbox" id="player_{player_num}_is_ai" value="is_ai">
            <label for="player_{player_num}_is_ai"> Is AI</label><br>"#, );


    return out;
    /*return format!( r#"        <div id="setup_player_config">
    {}
        </div>"#, );*/

}

pub fn get_colors_array() -> [&'static str; 6] {
    ["CadetBlue", "DarkOrchid", "DarkKhaki", "LimeGreen", "OrangeRed", "PeachPuff"]
}