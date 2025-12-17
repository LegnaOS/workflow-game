{
  "name": "Lite RPG 放置游戏",
  "description": "英雄装备武器(镶嵌3宝石)+防具，自动攻击Boss，Boss反击英雄",
  "blocks": {
    "10000001-0000-0000-0000-000000000001": {
      "id": "10000001-0000-0000-0000-000000000001",
      "script_id": "lite.gem_attack",
      "position": { "x": 50, "y": 50 },
      "size": { "x": 120, "y": 80 },
      "input_values": {},
      "output_values": {},
      "properties": { "level": 2, "base_attack": 3 },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "10000002-0000-0000-0000-000000000002": {
      "id": "10000002-0000-0000-0000-000000000002",
      "script_id": "lite.gem_crit",
      "position": { "x": 50, "y": 150 },
      "size": { "x": 120, "y": 80 },
      "input_values": {},
      "output_values": {},
      "properties": { "level": 1, "base_crit": 5 },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "10000003-0000-0000-0000-000000000003": {
      "id": "10000003-0000-0000-0000-000000000003",
      "script_id": "lite.gem_dodge",
      "position": { "x": 50, "y": 250 },
      "size": { "x": 120, "y": 80 },
      "input_values": {},
      "output_values": {},
      "properties": { "level": 1, "base_dodge": 3 },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "10000004-0000-0000-0000-000000000004": {
      "id": "10000004-0000-0000-0000-000000000004",
      "script_id": "lite.weapon",
      "position": { "x": 220, "y": 100 },
      "size": { "x": 140, "y": 160 },
      "input_values": {},
      "output_values": {},
      "properties": { "name": "木剑", "base_attack": 5, "quality": 2 },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "10000005-0000-0000-0000-000000000005": {
      "id": "10000005-0000-0000-0000-000000000005",
      "script_id": "lite.armor",
      "position": { "x": 220, "y": 300 },
      "size": { "x": 140, "y": 100 },
      "input_values": {},
      "output_values": {},
      "properties": { "name": "布衣", "base_defense": 3, "base_hp": 20, "quality": 2 },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "10000006-0000-0000-0000-000000000006": {
      "id": "10000006-0000-0000-0000-000000000006",
      "script_id": "lite.hero",
      "position": { "x": 420, "y": 80 },
      "size": { "x": 180, "y": 340 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "name": "新人小李",
        "base_hp": 100,
        "base_mp": 50,
        "base_atk": 10,
        "attack_speed": 1.0,
        "hp_regen": 2,
        "mp_regen": 3
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "10000007-0000-0000-0000-000000000007": {
      "id": "10000007-0000-0000-0000-000000000007",
      "script_id": "lite.boss",
      "position": { "x": 680, "y": 100 },
      "size": { "x": 180, "y": 280 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "name": "稻草人",
        "max_hp": 500,
        "attack": 15,
        "defense": 5,
        "attack_speed": 0.5,
        "gold_drop": 100,
        "respawn_time": 5
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "10000008-0000-0000-0000-000000000008": {
      "id": "10000008-0000-0000-0000-000000000008",
      "script_id": "lite.gold",
      "position": { "x": 920, "y": 200 },
      "size": { "x": 120, "y": 80 },
      "input_values": {},
      "output_values": {},
      "properties": { "initial": 0 },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    }
  },
  "connections": {
    "c0000001-0000-0000-0000-000000000001": {
      "id": "c0000001-0000-0000-0000-000000000001",
      "from_block": "10000001-0000-0000-0000-000000000001",
      "from_port": "gem_out",
      "to_block": "10000004-0000-0000-0000-000000000004",
      "to_port": "gem_1"
    },
    "c0000002-0000-0000-0000-000000000002": {
      "id": "c0000002-0000-0000-0000-000000000002",
      "from_block": "10000002-0000-0000-0000-000000000002",
      "from_port": "gem_out",
      "to_block": "10000004-0000-0000-0000-000000000004",
      "to_port": "gem_2"
    },
    "c0000003-0000-0000-0000-000000000003": {
      "id": "c0000003-0000-0000-0000-000000000003",
      "from_block": "10000003-0000-0000-0000-000000000003",
      "from_port": "gem_out",
      "to_block": "10000004-0000-0000-0000-000000000004",
      "to_port": "gem_3"
    },
    "c0000004-0000-0000-0000-000000000004": {
      "id": "c0000004-0000-0000-0000-000000000004",
      "from_block": "10000004-0000-0000-0000-000000000004",
      "from_port": "weapon_out",
      "to_block": "10000006-0000-0000-0000-000000000006",
      "to_port": "weapon"
    },
    "c0000005-0000-0000-0000-000000000005": {
      "id": "c0000005-0000-0000-0000-000000000005",
      "from_block": "10000005-0000-0000-0000-000000000005",
      "from_port": "armor_out",
      "to_block": "10000006-0000-0000-0000-000000000006",
      "to_port": "armor"
    },
    "c0000006-0000-0000-0000-000000000006": {
      "id": "c0000006-0000-0000-0000-000000000006",
      "from_block": "10000006-0000-0000-0000-000000000006",
      "from_port": "action_damage",
      "to_block": "10000007-0000-0000-0000-000000000007",
      "to_port": "damage_in"
    },
    "c0000007-0000-0000-0000-000000000007": {
      "id": "c0000007-0000-0000-0000-000000000007",
      "from_block": "10000006-0000-0000-0000-000000000006",
      "from_port": "action_event",
      "to_block": "10000007-0000-0000-0000-000000000007",
      "to_port": "hit_event"
    },
    "c0000008-0000-0000-0000-000000000008": {
      "id": "c0000008-0000-0000-0000-000000000008",
      "from_block": "10000007-0000-0000-0000-000000000007",
      "from_port": "counter_damage",
      "to_block": "10000006-0000-0000-0000-000000000006",
      "to_port": "damage_in"
    },
    "c0000009-0000-0000-0000-000000000009": {
      "id": "c0000009-0000-0000-0000-000000000009",
      "from_block": "10000007-0000-0000-0000-000000000007",
      "from_port": "counter_event",
      "to_block": "10000006-0000-0000-0000-000000000006",
      "to_port": "hit_event"
    },
    "c000000a-0000-0000-0000-00000000000a": {
      "id": "c000000a-0000-0000-0000-00000000000a",
      "from_block": "10000007-0000-0000-0000-000000000007",
      "from_port": "gold_reward",
      "to_block": "10000008-0000-0000-0000-000000000008",
      "to_port": "gold_in"
    },
    "c000000b-0000-0000-0000-00000000000b": {
      "id": "c000000b-0000-0000-0000-00000000000b",
      "from_block": "10000007-0000-0000-0000-000000000007",
      "from_port": "kill_event",
      "to_block": "10000008-0000-0000-0000-000000000008",
      "to_port": "collect_event"
    }
  },
  "groups": {},
  "viewport": { "offset": { "x": 0, "y": 0 }, "zoom": 1.0 },
  "readonly": false
}

