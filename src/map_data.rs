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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
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
    "card_type": "Infantry"
  }
]"#;
    return data.to_string();
}