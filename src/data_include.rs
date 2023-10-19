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

pub fn get_navtree_data() ->String{
    let data = r#"
    {
    "nav_nodes": [
        {
            "id": 11,
            "connections": [
                12,
                10,
                9
            ]
        },
        {
            "id": 12,
            "connections": [
                11,
                10
            ]
        },
        {
            "id": 10,
            "connections": [
                9,
                11,
                12,
                13
            ]
        },
        {
            "id": 9,
            "connections": [
                11,
                10,
                8
            ]
        },
        {
            "id": 8,
            "connections": [
                6,
                7,
                9
            ]
        },
        {
            "id": 6,
            "connections": [
                8,
                7,
                3,
                2
            ]
        },
        {
            "id": 7,
            "connections": [
                6,
                8,
                4,
                3,
                2
            ]
        },
        {
            "id": 2,
            "connections": [
                6,
                7,
                3,
                1,
                0
            ]
        },
        {
            "id": 3,
            "connections": [
                1,
                2,
                6,
                7,
                4,
                5
            ]
        },
        {
            "id": 4,
            "connections": [
                3,
                7,
                5
            ]
        },
        {
            "id": 1,
            "connections": [
                0,
                2,
                3,
                5
            ]
        },
        {
            "id": 0,
            "connections": [
                2,
                1,
                31
            ]
        },
        {
            "id": 5,
            "connections": [
                1,
                3,
                4,
                19
            ]
        },
        {
            "id": 20,
            "connections": [
                19,
                21,
                22,
                25
            ]
        },
        {
            "id": 21,
            "connections": [
                19,
                20,
                22,
                23
            ]
        },
        {
            "id": 22,
            "connections": [
                20,
                25,
                24,
                23,
                21
            ]
        },
        {
            "id": 25,
            "connections": [
                13,
                24,
                22,
                20
            ]
        },
        {
            "id": 24,
            "connections": [
                25,
                22,
                23,
                26,
                14
            ]
        },
        {
            "id": 23,
            "connections": [
                21,
                22,
                24,
                26,
                27,
                28
            ]
        },
        {
            "id": 13,
            "connections": [
                10,
                25,
                14,
                15,
                16
            ]
        },
        {
            "id": 16,
            "connections": [
                13,
                15,
                17
            ]
        },
        {
            "id": 17,
            "connections": [
                16,
                15,
                18
            ]
        },
        {
            "id": 18,
            "connections": [
                17,
                15
            ]
        },
        {
            "id": 14,
            "connections": [
                13,
                15,
                26,
                24
            ]
        },
        {
            "id": 15,
            "connections": [
                14,
                13,
                16,
                17,
                18,
                26
            ]
        },
        {
            "id": 26,
            "connections": [
                14,
                15,
                35,
                27,
                23,
                24
            ]
        },
        {
            "id": 27,
            "connections": [
                23,
                26,
                35,
                34,
                28
            ]
        },
        {
            "id": 28,
            "connections": [
                23,
                27,
                34,
                29
            ]
        },
        {
            "id": 29,
            "connections": [
                28,
                34,
                33,
                36,
                30
            ]
        },
        {
            "id": 30,
            "connections": [
                29,
                36,
                31
            ]
        },
        {
            "id": 31,
            "connections": [
                30,
                36,
                33,
                32,
                0
            ]
        },
        {
            "id": 36,
            "connections": [
                29,
                33,
                31,
                30
            ]
        },
        {
            "id": 33,
            "connections": [
                34,
                29,
                36,
                31,
                32
            ]
        },
        {
            "id": 34,
            "connections": [
                33,
                29,
                28,
                27,
                35,
                37
            ]
        },
        {
            "id": 35,
            "connections": [
                26,
                27,
                34,
                37
            ]
        },
        {
            "id": 32,
            "connections": [
                33,
                31
            ]
        },
        {
            "id": 37,
            "connections": [
                34,
                35,
                38
            ]
        },
        {
            "id": 38,
            "connections": [
                37,
                40,
                39,
                41
            ]
        },
        {
            "id": 39,
            "connections": [
                41,
                40,
                38
            ]
        },
        {
            "id": 40,
            "connections": [
                41,
                39,
                38
            ]
        },
        {
            "id": 41,
            "connections": [
                40,
                38,
                39
            ]
        },
        {
            "id": 19,
            "connections": [
                5,
                20,
                21
            ]
        }
    ],
    "adding_id_set": false,
    "adding_to": 19,
    "currently_selected": 0,
    "selection_active": false
}"#;
    return data.to_string();
}

