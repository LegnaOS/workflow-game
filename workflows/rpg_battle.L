{
  "name": "RPG战斗系统",
  "description": "双英雄对战三哥布林：剑士用重斩攻击战士怪，法师用火球群攻法师怪和精英怪",
  "blocks": {
    "00000001-0000-0000-0000-000000000001": {
      "id": "00000001-0000-0000-0000-000000000001",
      "script_id": "game.turn_controller",
      "position": { "x": 50, "y": 200 },
      "size": { "x": 160, "y": 100 },
      "input_values": {},
      "output_values": {},
      "properties": { "auto_turn": true, "turn_interval": 50 },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000002-0000-0000-0000-000000000002": {
      "id": "00000002-0000-0000-0000-000000000002",
      "script_id": "game.character",
      "position": { "x": 280, "y": 50 },
      "size": { "x": 180, "y": 300 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "name": "剑士",
        "level": 10,
        "base_attack": 25,
        "base_defense": 15,
        "max_hp": 200,
        "max_mana": 100,
        "crit_rate": 20,
        "dodge_rate": 15,
        "spell_power": 10
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000003-0000-0000-0000-000000000003": {
      "id": "00000003-0000-0000-0000-000000000003",
      "script_id": "game.character",
      "position": { "x": 280, "y": 380 },
      "size": { "x": 180, "y": 300 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "name": "法师",
        "level": 10,
        "base_attack": 8,
        "base_defense": 6,
        "max_hp": 120,
        "max_mana": 200,
        "crit_rate": 10,
        "dodge_rate": 8,
        "spell_power": 50
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000004-0000-0000-0000-000000000004": {
      "id": "00000004-0000-0000-0000-000000000004",
      "script_id": "game.attack",
      "position": { "x": 520, "y": 80 },
      "size": { "x": 160, "y": 100 },
      "input_values": {},
      "output_values": {},
      "properties": { "skill_name": "重斩", "damage_multiplier": 1.5, "ignore_defense": 10 },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000005-0000-0000-0000-000000000005": {
      "id": "00000005-0000-0000-0000-000000000005",
      "script_id": "game.fireball",
      "position": { "x": 520, "y": 400 },
      "size": { "x": 180, "y": 220 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "base_damage": 50,
        "mana_cost": 30,
        "cooldown": 1.5,
        "crit_rate": 25,
        "crit_multiplier": 2.0,
        "burn_chance": 50,
        "burn_damage": 10
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000006-0000-0000-0000-000000000006": {
      "id": "00000006-0000-0000-0000-000000000006",
      "script_id": "game.monster",
      "position": { "x": 760, "y": 50 },
      "size": { "x": 180, "y": 200 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "name": "哥布林战士",
        "max_hp": 100,
        "attack": 15,
        "defense": 5,
        "dodge_rate": 20,
        "crit_rate": 10,
        "gold_drop": 30,
        "exp_drop": 50,
        "respawn_time": 4
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000007-0000-0000-0000-000000000007": {
      "id": "00000007-0000-0000-0000-000000000007",
      "script_id": "game.monster",
      "position": { "x": 760, "y": 280 },
      "size": { "x": 180, "y": 200 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "name": "哥布林法师",
        "max_hp": 60,
        "attack": 25,
        "defense": 2,
        "dodge_rate": 10,
        "crit_rate": 15,
        "gold_drop": 50,
        "exp_drop": 80,
        "respawn_time": 5
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000008-0000-0000-0000-000000000008": {
      "id": "00000008-0000-0000-0000-000000000008",
      "script_id": "game.monster",
      "position": { "x": 760, "y": 510 },
      "size": { "x": 180, "y": 200 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "name": "精英哥布林",
        "max_hp": 250,
        "attack": 35,
        "defense": 15,
        "dodge_rate": 25,
        "crit_rate": 20,
        "gold_drop": 150,
        "exp_drop": 200,
        "respawn_time": 10
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000009-0000-0000-0000-000000000009": {
      "id": "00000009-0000-0000-0000-000000000009",
      "script_id": "game.regeneration",
      "position": { "x": 280, "y": 710 },
      "size": { "x": 180, "y": 160 },
      "input_values": {},
      "output_values": {},
      "properties": { "hp_regen": 5, "mana_regen": 10, "hp_regen_percent": 3, "mana_regen_percent": 5 },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "0000000a-0000-0000-0000-00000000000a": {
      "id": "0000000a-0000-0000-0000-00000000000a",
      "script_id": "game.inventory",
      "position": { "x": 1000, "y": 200 },
      "size": { "x": 160, "y": 120 },
      "input_values": {},
      "output_values": {},
      "properties": { "initial_gold": 100, "capacity": 50 },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "0000000b-0000-0000-0000-00000000000b": {
      "id": "0000000b-0000-0000-0000-00000000000b",
      "script_id": "math.add",
      "position": { "x": 1000, "y": 50 },
      "size": { "x": 140, "y": 120 },
      "input_values": {},
      "output_values": {},
      "properties": {},
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "0000000c-0000-0000-0000-00000000000c": {
      "id": "0000000c-0000-0000-0000-00000000000c",
      "script_id": "debug.logger",
      "position": { "x": 1000, "y": 380 },
      "size": { "x": 180, "y": 180 },
      "input_values": {},
      "output_values": {},
      "properties": { "level": "INFO", "tag": "战报" },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "0000000d-0000-0000-0000-00000000000d": {
      "id": "0000000d-0000-0000-0000-00000000000d",
      "script_id": "logic.compare",
      "position": { "x": 520, "y": 220 },
      "size": { "x": 140, "y": 100 },
      "input_values": {},
      "output_values": {},
      "properties": {},
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    }
  },
  "connections": {
    "c0000001-0001-0000-0000-000000000001": {
      "id": "c0000001-0001-0000-0000-000000000001",
      "from_block": "00000001-0000-0000-0000-000000000001",
      "from_port": "turn_event",
      "to_block": "00000002-0000-0000-0000-000000000002",
      "to_port": "action_trigger"
    },
    "c0000002-0001-0000-0000-000000000001": {
      "id": "c0000002-0001-0000-0000-000000000001",
      "from_block": "00000001-0000-0000-0000-000000000001",
      "from_port": "turn_event",
      "to_block": "00000003-0000-0000-0000-000000000003",
      "to_port": "action_trigger"
    },
    "c0000003-0001-0000-0000-000000000001": {
      "id": "c0000003-0001-0000-0000-000000000001",
      "from_block": "00000001-0000-0000-0000-000000000001",
      "from_port": "turn_event",
      "to_block": "00000009-0000-0000-0000-000000000009",
      "to_port": "turn_trigger"
    },
    "c0000004-0001-0000-0000-000000000001": {
      "id": "c0000004-0001-0000-0000-000000000001",
      "from_block": "00000002-0000-0000-0000-000000000002",
      "from_port": "action_out",
      "to_block": "00000004-0000-0000-0000-000000000004",
      "to_port": "action_in"
    },
    "c0000005-0001-0000-0000-000000000001": {
      "id": "c0000005-0001-0000-0000-000000000001",
      "from_block": "00000002-0000-0000-0000-000000000002",
      "from_port": "attack",
      "to_block": "00000004-0000-0000-0000-000000000004",
      "to_port": "attacker_attack"
    },
    "c0000006-0001-0000-0000-000000000001": {
      "id": "c0000006-0001-0000-0000-000000000001",
      "from_block": "00000003-0000-0000-0000-000000000003",
      "from_port": "action_out",
      "to_block": "00000005-0000-0000-0000-000000000005",
      "to_port": "cast_trigger"
    },
    "c0000007-0001-0000-0000-000000000001": {
      "id": "c0000007-0001-0000-0000-000000000001",
      "from_block": "00000003-0000-0000-0000-000000000003",
      "from_port": "attack",
      "to_block": "00000005-0000-0000-0000-000000000005",
      "to_port": "caster_attack"
    },
    "c0000008-0001-0000-0000-000000000001": {
      "id": "c0000008-0001-0000-0000-000000000001",
      "from_block": "00000003-0000-0000-0000-000000000003",
      "from_port": "mana",
      "to_block": "00000005-0000-0000-0000-000000000005",
      "to_port": "caster_mana"
    },
    "c0000009-0001-0000-0000-000000000001": {
      "id": "c0000009-0001-0000-0000-000000000001",
      "from_block": "00000003-0000-0000-0000-000000000003",
      "from_port": "spell_power",
      "to_block": "00000005-0000-0000-0000-000000000005",
      "to_port": "spell_power"
    },
    "c0000010-0001-0000-0000-000000000001": {
      "id": "c0000010-0001-0000-0000-000000000001",
      "from_block": "00000004-0000-0000-0000-000000000004",
      "from_port": "damage_out",
      "to_block": "00000006-0000-0000-0000-000000000006",
      "to_port": "damage_in"
    },
    "c0000011-0001-0000-0000-000000000001": {
      "id": "c0000011-0001-0000-0000-000000000001",
      "from_block": "00000004-0000-0000-0000-000000000004",
      "from_port": "attack_event",
      "to_block": "00000006-0000-0000-0000-000000000006",
      "to_port": "attack_event"
    },
    "c0000012-0001-0000-0000-000000000001": {
      "id": "c0000012-0001-0000-0000-000000000001",
      "from_block": "00000005-0000-0000-0000-000000000005",
      "from_port": "damage_out",
      "to_block": "00000007-0000-0000-0000-000000000007",
      "to_port": "damage_in"
    },
    "c0000013-0001-0000-0000-000000000001": {
      "id": "c0000013-0001-0000-0000-000000000001",
      "from_block": "00000005-0000-0000-0000-000000000005",
      "from_port": "cast_event",
      "to_block": "00000007-0000-0000-0000-000000000007",
      "to_port": "attack_event"
    },
    "c0000014-0001-0000-0000-000000000001": {
      "id": "c0000014-0001-0000-0000-000000000001",
      "from_block": "00000005-0000-0000-0000-000000000005",
      "from_port": "damage_out",
      "to_block": "00000008-0000-0000-0000-000000000008",
      "to_port": "damage_in"
    },
    "c0000015-0001-0000-0000-000000000001": {
      "id": "c0000015-0001-0000-0000-000000000001",
      "from_block": "00000005-0000-0000-0000-000000000005",
      "from_port": "cast_event",
      "to_block": "00000008-0000-0000-0000-000000000008",
      "to_port": "attack_event"
    },
    "c0000016-0001-0000-0000-000000000001": {
      "id": "c0000016-0001-0000-0000-000000000001",
      "from_block": "00000006-0000-0000-0000-000000000006",
      "from_port": "gold_reward",
      "to_block": "0000000b-0000-0000-0000-00000000000b",
      "to_port": "a"
    },
    "c0000017-0001-0000-0000-000000000001": {
      "id": "c0000017-0001-0000-0000-000000000001",
      "from_block": "00000007-0000-0000-0000-000000000007",
      "from_port": "gold_reward",
      "to_block": "0000000b-0000-0000-0000-00000000000b",
      "to_port": "b"
    },
    "c0000018-0001-0000-0000-000000000001": {
      "id": "c0000018-0001-0000-0000-000000000001",
      "from_block": "00000008-0000-0000-0000-000000000008",
      "from_port": "gold_reward",
      "to_block": "0000000b-0000-0000-0000-00000000000b",
      "to_port": "c"
    },
    "c0000019-0001-0000-0000-000000000001": {
      "id": "c0000019-0001-0000-0000-000000000001",
      "from_block": "0000000b-0000-0000-0000-00000000000b",
      "from_port": "result",
      "to_block": "0000000a-0000-0000-0000-00000000000a",
      "to_port": "gold_in"
    },
    "c0000020-0001-0000-0000-000000000001": {
      "id": "c0000020-0001-0000-0000-000000000001",
      "from_block": "0000000a-0000-0000-0000-00000000000a",
      "from_port": "total_gold",
      "to_block": "0000000c-0000-0000-0000-00000000000c",
      "to_port": "in1"
    },
    "c0000021-0001-0000-0000-000000000001": {
      "id": "c0000021-0001-0000-0000-000000000001",
      "from_block": "00000002-0000-0000-0000-000000000002",
      "from_port": "hp",
      "to_block": "0000000c-0000-0000-0000-00000000000c",
      "to_port": "in2"
    },
    "c0000022-0001-0000-0000-000000000001": {
      "id": "c0000022-0001-0000-0000-000000000001",
      "from_block": "00000003-0000-0000-0000-000000000003",
      "from_port": "hp",
      "to_block": "0000000c-0000-0000-0000-00000000000c",
      "to_port": "in3"
    },
    "c0000023-0001-0000-0000-000000000001": {
      "id": "c0000023-0001-0000-0000-000000000001",
      "from_block": "00000001-0000-0000-0000-000000000001",
      "from_port": "turn_count",
      "to_block": "0000000c-0000-0000-0000-00000000000c",
      "to_port": "in4"
    },
    "c0000024-0001-0000-0000-000000000001": {
      "id": "c0000024-0001-0000-0000-000000000001",
      "from_block": "00000002-0000-0000-0000-000000000002",
      "from_port": "hp",
      "to_block": "0000000d-0000-0000-0000-00000000000d",
      "to_port": "a"
    },
    "c0000025-0001-0000-0000-000000000001": {
      "id": "c0000025-0001-0000-0000-000000000001",
      "from_block": "00000003-0000-0000-0000-000000000003",
      "from_port": "hp",
      "to_block": "0000000d-0000-0000-0000-00000000000d",
      "to_port": "b"
    }
  },
  "groups": {},
  "viewport": { "offset": { "x": 0, "y": 0 }, "zoom": 0.85 },
  "readonly": false
}

