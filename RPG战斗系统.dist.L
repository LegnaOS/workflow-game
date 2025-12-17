{
  "name": "RPG战斗系统",
  "blocks": {
    "00000004-0000-0000-0000-000000000004": {
      "id": "00000004-0000-0000-0000-000000000004",
      "script_id": "game.attack",
      "position": {
        "x": 520.0,
        "y": 80.0
      },
      "size": {
        "x": 160.0,
        "y": 100.0
      },
      "input_values": {
        "attacker_attack": 45.0,
        "action_in": true
      },
      "output_values": {
        "damage_out": 67.0,
        "attack_event": true
      },
      "properties": {
        "damage_multiplier": 1.5,
        "ignore_defense": 10.0,
        "skill_name": "重斩"
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000005-0000-0000-0000-000000000005": {
      "id": "00000005-0000-0000-0000-000000000005",
      "script_id": "game.fireball",
      "position": {
        "x": 520.0,
        "y": 400.0
      },
      "size": {
        "x": 180.0,
        "y": 220.0
      },
      "input_values": {
        "caster_attack": 28.0,
        "cast_trigger": true,
        "spell_power": 65.0,
        "caster_mana": 200.0
      },
      "output_values": {
        "is_burning": false,
        "burn_dps": 0.0,
        "damage_out": 0.0,
        "is_crit": false,
        "cast_event": true,
        "remaining_cd": 0.5,
        "is_ready": false,
        "mana_used": 0.0
      },
      "properties": {
        "crit_rate": 25.0,
        "burn_damage": 10.0,
        "crit_multiplier": 2.0,
        "burn_chance": 50.0,
        "base_damage": 50.0,
        "mana_cost": 30.0,
        "cooldown": 1.5
      },
      "state": {
        "tick": 930.0,
        "last_cast_tick": 920.0
      },
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000001-0000-0000-0000-000000000001": {
      "id": "00000001-0000-0000-0000-000000000001",
      "script_id": "game.turn_controller",
      "position": {
        "x": 50.0,
        "y": 200.0
      },
      "size": {
        "x": 160.0,
        "y": 100.0
      },
      "input_values": {},
      "output_values": {
        "turn_event": true,
        "is_running": true,
        "turn_count": 18.0
      },
      "properties": {
        "turn_interval": 50.0,
        "auto_turn": true
      },
      "state": {
        "turn_count": 18.0,
        "tick": 930.0
      },
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000003-0000-0000-0000-000000000003": {
      "id": "00000003-0000-0000-0000-000000000003",
      "script_id": "game.character",
      "position": {
        "x": 280.0,
        "y": 380.0
      },
      "size": {
        "x": 180.0,
        "y": 300.0
      },
      "input_values": {
        "action_trigger": true
      },
      "output_values": {
        "defense": 16.0,
        "is_alive": true,
        "spell_power": 65.0,
        "max_hp": 120.0,
        "max_mana": 200.0,
        "attack": 28.0,
        "action_out": true,
        "hp": 120.0,
        "dodge_rate": 11.0,
        "mana": 200.0,
        "crit_rate": 15.0
      },
      "properties": {
        "crit_rate": 10.0,
        "spell_power": 50.0,
        "dodge_rate": 8.0,
        "max_mana": 200.0,
        "max_hp": 120.0,
        "name": "法师",
        "level": 10.0,
        "base_defense": 6.0,
        "base_attack": 8.0
      },
      "state": {
        "current_mana": 200.0,
        "current_hp": 120.0
      },
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000002-0000-0000-0000-000000000002": {
      "id": "00000002-0000-0000-0000-000000000002",
      "script_id": "game.character",
      "position": {
        "x": 280.0,
        "y": 50.0
      },
      "size": {
        "x": 180.0,
        "y": 300.0
      },
      "input_values": {
        "action_trigger": true
      },
      "output_values": {
        "defense": 25.0,
        "dodge_rate": 18.0,
        "is_alive": true,
        "spell_power": 25.0,
        "mana": 100.0,
        "crit_rate": 25.0,
        "action_out": true,
        "attack": 45.0,
        "max_hp": 200.0,
        "max_mana": 100.0,
        "hp": 200.0
      },
      "properties": {
        "base_defense": 15.0,
        "base_attack": 25.0,
        "dodge_rate": 15.0,
        "level": 10.0,
        "name": "剑士",
        "max_mana": 100.0,
        "spell_power": 10.0,
        "crit_rate": 20.0,
        "max_hp": 200.0
      },
      "state": {
        "current_hp": 200.0,
        "current_mana": 100.0
      },
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000006-0000-0000-0000-000000000006": {
      "id": "00000006-0000-0000-0000-000000000006",
      "script_id": "game.monster",
      "position": {
        "x": 760.0,
        "y": 50.0
      },
      "size": {
        "x": 180.0,
        "y": 200.0
      },
      "input_values": {
        "damage_in": 67.0,
        "attack_event": true
      },
      "output_values": {
        "is_dead": true,
        "current_hp": 0.0,
        "dodge_rate": 20.0,
        "attack_out": true,
        "gold_reward": 0.0,
        "exp_reward": 0.0,
        "monster_defense": 5.0,
        "monster_attack": 15.0
      },
      "properties": {
        "crit_rate": 10.0,
        "attack": 15.0,
        "dodge_rate": 20.0,
        "max_hp": 100.0,
        "defense": 5.0,
        "gold_drop": 30.0,
        "respawn_time": 4.0,
        "exp_drop": 50.0,
        "name": "哥布林战士"
      },
      "state": {
        "current_hp": 0.0,
        "dead_ticks": 12.0
      },
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000008-0000-0000-0000-000000000008": {
      "id": "00000008-0000-0000-0000-000000000008",
      "script_id": "game.monster",
      "position": {
        "x": 760.0,
        "y": 510.0
      },
      "size": {
        "x": 180.0,
        "y": 200.0
      },
      "input_values": {
        "damage_in": 0.0,
        "attack_event": true
      },
      "output_values": {
        "attack_out": true,
        "dodge_rate": 25.0,
        "is_dead": false,
        "monster_defense": 15.0,
        "monster_attack": 35.0,
        "current_hp": 250.0,
        "gold_reward": 0.0,
        "exp_reward": 0.0
      },
      "properties": {
        "attack": 35.0,
        "exp_drop": 200.0,
        "respawn_time": 10.0,
        "crit_rate": 20.0,
        "gold_drop": 150.0,
        "name": "精英哥布林",
        "max_hp": 250.0,
        "defense": 15.0,
        "dodge_rate": 25.0
      },
      "state": {
        "current_hp": 250.0,
        "dead_ticks": 0.0
      },
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000009-0000-0000-0000-000000000009": {
      "id": "00000009-0000-0000-0000-000000000009",
      "script_id": "game.regeneration",
      "position": {
        "x": 280.0,
        "y": 710.0
      },
      "size": {
        "x": 180.0,
        "y": 160.0
      },
      "input_values": {
        "turn_trigger": true
      },
      "output_values": {
        "hp_healed": 0.0,
        "mana_restored": 0.0,
        "regen_event": true,
        "new_hp": 100.0,
        "new_mana": 50.0
      },
      "properties": {
        "hp_regen": 5.0,
        "mana_regen_percent": 5.0,
        "hp_regen_percent": 3.0,
        "mana_regen": 10.0
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "0000000a-0000-0000-0000-00000000000a": {
      "id": "0000000a-0000-0000-0000-00000000000a",
      "script_id": "game.inventory",
      "position": {
        "x": 1000.0,
        "y": 200.0
      },
      "size": {
        "x": 160.0,
        "y": 120.0
      },
      "input_values": {
        "gold_in": 0.0
      },
      "output_values": {
        "item_count": 0.0,
        "is_full": false,
        "total_gold": 2560.0
      },
      "properties": {
        "capacity": 50.0,
        "initial_gold": 100.0
      },
      "state": {
        "item_count": 0.0,
        "total_gold": 2560.0
      },
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "0000000d-0000-0000-0000-00000000000d": {
      "id": "0000000d-0000-0000-0000-00000000000d",
      "script_id": "logic.compare",
      "position": {
        "x": 520.0,
        "y": 280.0
      },
      "size": {
        "x": 140.0,
        "y": 100.0
      },
      "input_values": {
        "a": 200.0,
        "b": 120.0
      },
      "output_values": {
        "greater": true,
        "less": false,
        "equal": false
      },
      "properties": {},
      "state": {},
      "selected": true,
      "collapsed": false,
      "group_id": null
    },
    "0000000c-0000-0000-0000-00000000000c": {
      "id": "0000000c-0000-0000-0000-00000000000c",
      "script_id": "debug.logger",
      "position": {
        "x": 1000.0,
        "y": 380.0
      },
      "size": {
        "x": 180.0,
        "y": 180.0
      },
      "input_values": {
        "in4": 18.0,
        "in1": 2560.0,
        "in2": 200.0,
        "in3": 120.0
      },
      "output_values": {
        "json_out": "{\n  \"input2\": 200.0,\n  \"input3\": 120.0,\n  \"input1\": 2560.0,\n  \"input4\": 18.0\n}",
        "log_text": "[13:38:00] [INFO] [战报] {\n  \"input2\": 200.0,\n  \"input3\": 120.0,\n  \"input1\": 2560.0,\n  \"input4\": 18.0\n}"
      },
      "properties": {
        "tag": "战报",
        "level": "INFO"
      },
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "00000007-0000-0000-0000-000000000007": {
      "id": "00000007-0000-0000-0000-000000000007",
      "script_id": "game.monster",
      "position": {
        "x": 760.0,
        "y": 280.0
      },
      "size": {
        "x": 180.0,
        "y": 200.0
      },
      "input_values": {
        "damage_in": 0.0,
        "attack_event": true
      },
      "output_values": {
        "exp_reward": 0.0,
        "current_hp": 0.0,
        "attack_out": true,
        "monster_attack": 25.0,
        "monster_defense": 2.0,
        "dodge_rate": 10.0,
        "gold_reward": 0.0,
        "is_dead": true
      },
      "properties": {
        "respawn_time": 5.0,
        "attack": 25.0,
        "dodge_rate": 10.0,
        "max_hp": 60.0,
        "name": "哥布林法师",
        "crit_rate": 15.0,
        "defense": 2.0,
        "gold_drop": 50.0,
        "exp_drop": 80.0
      },
      "state": {
        "dead_ticks": 10.0,
        "current_hp": 0.0
      },
      "selected": false,
      "collapsed": false,
      "group_id": null
    },
    "0000000b-0000-0000-0000-00000000000b": {
      "id": "0000000b-0000-0000-0000-00000000000b",
      "script_id": "math.add",
      "position": {
        "x": 1000.0,
        "y": 50.0
      },
      "size": {
        "x": 140.0,
        "y": 120.0
      },
      "input_values": {
        "c": 0.0,
        "a": 0.0,
        "b": 0.0
      },
      "output_values": {
        "count": 3.0,
        "result": 0.0
      },
      "properties": {},
      "state": {},
      "selected": false,
      "collapsed": false,
      "group_id": null
    }
  },
  "connections": {
    "c0000022-0001-0000-0000-000000000001": {
      "id": "c0000022-0001-0000-0000-000000000001",
      "from_block": "00000003-0000-0000-0000-000000000003",
      "from_port": "hp",
      "to_block": "0000000c-0000-0000-0000-00000000000c",
      "to_port": "in3"
    },
    "c0000005-0001-0000-0000-000000000001": {
      "id": "c0000005-0001-0000-0000-000000000001",
      "from_block": "00000002-0000-0000-0000-000000000002",
      "from_port": "attack",
      "to_block": "00000004-0000-0000-0000-000000000004",
      "to_port": "attacker_attack"
    },
    "c0000023-0001-0000-0000-000000000001": {
      "id": "c0000023-0001-0000-0000-000000000001",
      "from_block": "00000001-0000-0000-0000-000000000001",
      "from_port": "turn_count",
      "to_block": "0000000c-0000-0000-0000-00000000000c",
      "to_port": "in4"
    },
    "c0000013-0001-0000-0000-000000000001": {
      "id": "c0000013-0001-0000-0000-000000000001",
      "from_block": "00000005-0000-0000-0000-000000000005",
      "from_port": "cast_event",
      "to_block": "00000007-0000-0000-0000-000000000007",
      "to_port": "attack_event"
    },
    "c0000015-0001-0000-0000-000000000001": {
      "id": "c0000015-0001-0000-0000-000000000001",
      "from_block": "00000005-0000-0000-0000-000000000005",
      "from_port": "cast_event",
      "to_block": "00000008-0000-0000-0000-000000000008",
      "to_port": "attack_event"
    },
    "c0000024-0001-0000-0000-000000000001": {
      "id": "c0000024-0001-0000-0000-000000000001",
      "from_block": "00000002-0000-0000-0000-000000000002",
      "from_port": "hp",
      "to_block": "0000000d-0000-0000-0000-00000000000d",
      "to_port": "a"
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
    "c0000019-0001-0000-0000-000000000001": {
      "id": "c0000019-0001-0000-0000-000000000001",
      "from_block": "0000000b-0000-0000-0000-00000000000b",
      "from_port": "result",
      "to_block": "0000000a-0000-0000-0000-00000000000a",
      "to_port": "gold_in"
    },
    "c0000014-0001-0000-0000-000000000001": {
      "id": "c0000014-0001-0000-0000-000000000001",
      "from_block": "00000005-0000-0000-0000-000000000005",
      "from_port": "damage_out",
      "to_block": "00000008-0000-0000-0000-000000000008",
      "to_port": "damage_in"
    },
    "c0000020-0001-0000-0000-000000000001": {
      "id": "c0000020-0001-0000-0000-000000000001",
      "from_block": "0000000a-0000-0000-0000-00000000000a",
      "from_port": "total_gold",
      "to_block": "0000000c-0000-0000-0000-00000000000c",
      "to_port": "in1"
    },
    "c0000025-0001-0000-0000-000000000001": {
      "id": "c0000025-0001-0000-0000-000000000001",
      "from_block": "00000003-0000-0000-0000-000000000003",
      "from_port": "hp",
      "to_block": "0000000d-0000-0000-0000-00000000000d",
      "to_port": "b"
    },
    "c0000007-0001-0000-0000-000000000001": {
      "id": "c0000007-0001-0000-0000-000000000001",
      "from_block": "00000003-0000-0000-0000-000000000003",
      "from_port": "attack",
      "to_block": "00000005-0000-0000-0000-000000000005",
      "to_port": "caster_attack"
    },
    "c0000010-0001-0000-0000-000000000001": {
      "id": "c0000010-0001-0000-0000-000000000001",
      "from_block": "00000004-0000-0000-0000-000000000004",
      "from_port": "damage_out",
      "to_block": "00000006-0000-0000-0000-000000000006",
      "to_port": "damage_in"
    },
    "c0000018-0001-0000-0000-000000000001": {
      "id": "c0000018-0001-0000-0000-000000000001",
      "from_block": "00000008-0000-0000-0000-000000000008",
      "from_port": "gold_reward",
      "to_block": "0000000b-0000-0000-0000-00000000000b",
      "to_port": "c"
    },
    "c0000002-0001-0000-0000-000000000001": {
      "id": "c0000002-0001-0000-0000-000000000001",
      "from_block": "00000001-0000-0000-0000-000000000001",
      "from_port": "turn_event",
      "to_block": "00000003-0000-0000-0000-000000000003",
      "to_port": "action_trigger"
    },
    "c0000012-0001-0000-0000-000000000001": {
      "id": "c0000012-0001-0000-0000-000000000001",
      "from_block": "00000005-0000-0000-0000-000000000005",
      "from_port": "damage_out",
      "to_block": "00000007-0000-0000-0000-000000000007",
      "to_port": "damage_in"
    },
    "c0000004-0001-0000-0000-000000000001": {
      "id": "c0000004-0001-0000-0000-000000000001",
      "from_block": "00000002-0000-0000-0000-000000000002",
      "from_port": "action_out",
      "to_block": "00000004-0000-0000-0000-000000000004",
      "to_port": "action_in"
    },
    "c0000003-0001-0000-0000-000000000001": {
      "id": "c0000003-0001-0000-0000-000000000001",
      "from_block": "00000001-0000-0000-0000-000000000001",
      "from_port": "turn_event",
      "to_block": "00000009-0000-0000-0000-000000000009",
      "to_port": "turn_trigger"
    },
    "c0000001-0001-0000-0000-000000000001": {
      "id": "c0000001-0001-0000-0000-000000000001",
      "from_block": "00000001-0000-0000-0000-000000000001",
      "from_port": "turn_event",
      "to_block": "00000002-0000-0000-0000-000000000002",
      "to_port": "action_trigger"
    },
    "c0000006-0001-0000-0000-000000000001": {
      "id": "c0000006-0001-0000-0000-000000000001",
      "from_block": "00000003-0000-0000-0000-000000000003",
      "from_port": "action_out",
      "to_block": "00000005-0000-0000-0000-000000000005",
      "to_port": "cast_trigger"
    },
    "c0000016-0001-0000-0000-000000000001": {
      "id": "c0000016-0001-0000-0000-000000000001",
      "from_block": "00000006-0000-0000-0000-000000000006",
      "from_port": "gold_reward",
      "to_block": "0000000b-0000-0000-0000-00000000000b",
      "to_port": "a"
    },
    "c0000011-0001-0000-0000-000000000001": {
      "id": "c0000011-0001-0000-0000-000000000001",
      "from_block": "00000004-0000-0000-0000-000000000004",
      "from_port": "attack_event",
      "to_block": "00000006-0000-0000-0000-000000000006",
      "to_port": "attack_event"
    },
    "c0000017-0001-0000-0000-000000000001": {
      "id": "c0000017-0001-0000-0000-000000000001",
      "from_block": "00000007-0000-0000-0000-000000000007",
      "from_port": "gold_reward",
      "to_block": "0000000b-0000-0000-0000-00000000000b",
      "to_port": "b"
    },
    "c0000021-0001-0000-0000-000000000001": {
      "id": "c0000021-0001-0000-0000-000000000001",
      "from_block": "00000002-0000-0000-0000-000000000002",
      "from_port": "hp",
      "to_block": "0000000c-0000-0000-0000-00000000000c",
      "to_port": "in2"
    }
  },
  "groups": {},
  "viewport": {
    "offset": {
      "x": -28.009155,
      "y": 125.06641
    },
    "zoom": 1.3556459
  },
  "readonly": true
}