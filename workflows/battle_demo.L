{
  "name": "战斗演示",
  "blocks": {
    "11111111-1111-1111-1111-111111111111": {
      "id": "11111111-1111-1111-1111-111111111111",
      "script_id": "game.turn_controller",
      "position": { "x": 50, "y": 50 },
      "size": { "x": 160, "y": 120 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "auto_turn": true,
        "turn_interval": 60
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "22222222-2222-2222-2222-222222222222": {
      "id": "22222222-2222-2222-2222-222222222222",
      "script_id": "game.character",
      "position": { "x": 50, "y": 220 },
      "size": { "x": 160, "y": 200 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "name": "勇者",
        "level": 5,
        "base_attack": 15,
        "base_defense": 8,
        "max_hp": 150,
        "max_mana": 80,
        "crit_rate": 15,
        "dodge_rate": 10,
        "spell_power": 20
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "33333333-3333-3333-3333-333333333333": {
      "id": "33333333-3333-3333-3333-333333333333",
      "script_id": "game.regeneration",
      "position": { "x": 280, "y": 50 },
      "size": { "x": 160, "y": 150 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "hp_regen": 5,
        "mana_regen": 8,
        "hp_regen_percent": 2,
        "mana_regen_percent": 5
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "44444444-4444-4444-4444-444444444444": {
      "id": "44444444-4444-4444-4444-444444444444",
      "script_id": "game.attack",
      "position": { "x": 280, "y": 250 },
      "size": { "x": 160, "y": 120 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "skill_name": "普通攻击",
        "damage_multiplier": 1.0,
        "ignore_defense": 0
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "55555555-5555-5555-5555-555555555555": {
      "id": "55555555-5555-5555-5555-555555555555",
      "script_id": "game.fireball",
      "position": { "x": 280, "y": 420 },
      "size": { "x": 160, "y": 180 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "base_damage": 40,
        "mana_cost": 25,
        "cooldown": 2.0,
        "crit_rate": 20,
        "crit_multiplier": 2.5,
        "burn_chance": 40,
        "burn_damage": 8
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "66666666-6666-6666-6666-666666666666": {
      "id": "66666666-6666-6666-6666-666666666666",
      "script_id": "game.monster",
      "position": { "x": 510, "y": 250 },
      "size": { "x": 160, "y": 180 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "name": "哥布林",
        "max_hp": 80,
        "attack": 12,
        "defense": 4,
        "dodge_rate": 15,
        "crit_rate": 5,
        "gold_drop": 25,
        "exp_drop": 30,
        "respawn_time": 5
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "77777777-7777-7777-7777-777777777777": {
      "id": "77777777-7777-7777-7777-777777777777",
      "script_id": "game.gold_collector",
      "position": { "x": 740, "y": 250 },
      "size": { "x": 160, "y": 120 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "bonus_percent": 10
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "88888888-8888-8888-8888-888888888888": {
      "id": "88888888-8888-8888-8888-888888888888",
      "script_id": "debug.logger",
      "position": { "x": 740, "y": 420 },
      "size": { "x": 160, "y": 150 },
      "input_values": {},
      "output_values": {},
      "properties": {
        "prefix": "[战报]",
        "log_level": "info"
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    }
  },
  "connections": {
    "c0000001-0000-0000-0000-000000000001": {
      "id": "c0000001-0000-0000-0000-000000000001",
      "from_block": "11111111-1111-1111-1111-111111111111",
      "from_port": "turn_event",
      "to_block": "22222222-2222-2222-2222-222222222222",
      "to_port": "action_trigger"
    },
    "c0000002-0000-0000-0000-000000000002": {
      "id": "c0000002-0000-0000-0000-000000000002",
      "from_block": "11111111-1111-1111-1111-111111111111",
      "from_port": "turn_event",
      "to_block": "33333333-3333-3333-3333-333333333333",
      "to_port": "turn_trigger"
    },
    "c0000003-0000-0000-0000-000000000003": {
      "id": "c0000003-0000-0000-0000-000000000003",
      "from_block": "22222222-2222-2222-2222-222222222222",
      "from_port": "action_out",
      "to_block": "44444444-4444-4444-4444-444444444444",
      "to_port": "action_in"
    },
    "c0000016-0000-0000-0000-000000000016": {
      "id": "c0000016-0000-0000-0000-000000000016",
      "from_block": "22222222-2222-2222-2222-222222222222",
      "from_port": "hp",
      "to_block": "33333333-3333-3333-3333-333333333333",
      "to_port": "current_hp"
    },
    "c0000017-0000-0000-0000-000000000017": {
      "id": "c0000017-0000-0000-0000-000000000017",
      "from_block": "22222222-2222-2222-2222-222222222222",
      "from_port": "max_hp",
      "to_block": "33333333-3333-3333-3333-333333333333",
      "to_port": "max_hp"
    },
    "c0000018-0000-0000-0000-000000000018": {
      "id": "c0000018-0000-0000-0000-000000000018",
      "from_block": "22222222-2222-2222-2222-222222222222",
      "from_port": "mana",
      "to_block": "33333333-3333-3333-3333-333333333333",
      "to_port": "current_mana"
    },
    "c0000019-0000-0000-0000-000000000019": {
      "id": "c0000019-0000-0000-0000-000000000019",
      "from_block": "22222222-2222-2222-2222-222222222222",
      "from_port": "max_mana",
      "to_block": "33333333-3333-3333-3333-333333333333",
      "to_port": "max_mana"
    },
    "c0000004-0000-0000-0000-000000000004": {
      "id": "c0000004-0000-0000-0000-000000000004",
      "from_block": "22222222-2222-2222-2222-222222222222",
      "from_port": "attack",
      "to_block": "44444444-4444-4444-4444-444444444444",
      "to_port": "attacker_attack"
    },
    "c0000005-0000-0000-0000-000000000005": {
      "id": "c0000005-0000-0000-0000-000000000005",
      "from_block": "22222222-2222-2222-2222-222222222222",
      "from_port": "action_out",
      "to_block": "55555555-5555-5555-5555-555555555555",
      "to_port": "cast_trigger"
    },
    "c0000006-0000-0000-0000-000000000006": {
      "id": "c0000006-0000-0000-0000-000000000006",
      "from_block": "22222222-2222-2222-2222-222222222222",
      "from_port": "attack",
      "to_block": "55555555-5555-5555-5555-555555555555",
      "to_port": "caster_attack"
    },
    "c0000007-0000-0000-0000-000000000007": {
      "id": "c0000007-0000-0000-0000-000000000007",
      "from_block": "22222222-2222-2222-2222-222222222222",
      "from_port": "mana",
      "to_block": "55555555-5555-5555-5555-555555555555",
      "to_port": "caster_mana"
    },
    "c0000008-0000-0000-0000-000000000008": {
      "id": "c0000008-0000-0000-0000-000000000008",
      "from_block": "22222222-2222-2222-2222-222222222222",
      "from_port": "spell_power",
      "to_block": "55555555-5555-5555-5555-555555555555",
      "to_port": "spell_power"
    },
    "c0000009-0000-0000-0000-000000000009": {
      "id": "c0000009-0000-0000-0000-000000000009",
      "from_block": "44444444-4444-4444-4444-444444444444",
      "from_port": "damage_out",
      "to_block": "66666666-6666-6666-6666-666666666666",
      "to_port": "damage_in"
    },
    "c0000010-0000-0000-0000-000000000010": {
      "id": "c0000010-0000-0000-0000-000000000010",
      "from_block": "44444444-4444-4444-4444-444444444444",
      "from_port": "attack_event",
      "to_block": "66666666-6666-6666-6666-666666666666",
      "to_port": "attack_event"
    },
    "c0000011-0000-0000-0000-000000000011": {
      "id": "c0000011-0000-0000-0000-000000000011",
      "from_block": "66666666-6666-6666-6666-666666666666",
      "from_port": "gold_reward",
      "to_block": "77777777-7777-7777-7777-777777777777",
      "to_port": "gold_in"
    },
    "c0000012-0000-0000-0000-000000000012": {
      "id": "c0000012-0000-0000-0000-000000000012",
      "from_block": "66666666-6666-6666-6666-666666666666",
      "from_port": "is_dead",
      "to_block": "77777777-7777-7777-7777-777777777777",
      "to_port": "collect_event"
    },
    "c0000013-0000-0000-0000-000000000013": {
      "id": "c0000013-0000-0000-0000-000000000013",
      "from_block": "77777777-7777-7777-7777-777777777777",
      "from_port": "total_gold",
      "to_block": "88888888-8888-8888-8888-888888888888",
      "to_port": "in1"
    },
    "c0000014-0000-0000-0000-000000000014": {
      "id": "c0000014-0000-0000-0000-000000000014",
      "from_block": "22222222-2222-2222-2222-222222222222",
      "from_port": "hp",
      "to_block": "88888888-8888-8888-8888-888888888888",
      "to_port": "in2"
    },
    "c0000015-0000-0000-0000-000000000015": {
      "id": "c0000015-0000-0000-0000-000000000015",
      "from_block": "66666666-6666-6666-6666-666666666666",
      "from_port": "current_hp",
      "to_block": "88888888-8888-8888-8888-888888888888",
      "to_port": "in3"
    }
  },
  "groups": {},
  "viewport": {
    "offset": { "x": 20, "y": 20 },
    "zoom": 1.0
  },
  "readonly": false
}

