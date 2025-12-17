-- 角色Block
-- 玩家角色，有属性和行动能力

return {
    meta = {
        id = "game.character",
        name = "角色",
        category = "游戏",
        color = "#4CAF50",
        description = "玩家角色，可以输出属性和执行行动"
    },

    properties = {
        { id = "name", name = "名称", type = "string", default = "勇者" },
        { id = "level", name = "等级", type = "number", default = 1, min = 1, max = 100 },
        { id = "base_attack", name = "基础攻击", type = "number", default = 10, min = 1 },
        { id = "base_defense", name = "基础防御", type = "number", default = 5, min = 0 },
        { id = "max_hp", name = "最大生命", type = "number", default = 100, min = 1 },
        { id = "max_mana", name = "最大法力", type = "number", default = 50, min = 0 },
        { id = "crit_rate", name = "暴击率%", type = "number", default = 10, min = 0, max = 100 },
        { id = "dodge_rate", name = "闪避率%", type = "number", default = 5, min = 0, max = 100 },
        { id = "spell_power", name = "法术强度", type = "number", default = 0, min = 0 }
    },

    inputs = {
        { id = "action_trigger", name = "行动触发", type = "event", default = nil },
        { id = "damage_in", name = "受到伤害", type = "number", default = 0 }
    },

    outputs = {
        { id = "attack", name = "攻击力", type = "number", default = 10 },
        { id = "defense", name = "防御力", type = "number", default = 5 },
        { id = "hp", name = "当前生命", type = "number", default = 100 },
        { id = "max_hp", name = "最大生命", type = "number", default = 100 },
        { id = "mana", name = "当前法力", type = "number", default = 50 },
        { id = "max_mana", name = "最大法力", type = "number", default = 50 },
        { id = "crit_rate", name = "暴击率", type = "number", default = 10 },
        { id = "dodge_rate", name = "闪避率", type = "number", default = 5 },
        { id = "spell_power", name = "法术强度", type = "number", default = 0 },
        { id = "action_out", name = "行动输出", type = "event", default = nil },
        { id = "is_alive", name = "存活", type = "boolean", default = true }
    },

    execute = function(self, inputs)
        local props = self.properties
        local state = self.state or {}
        local level = props.level or 1

        -- 计算属性（基础 + 等级加成）
        local attack = (props.base_attack or 10) + level * 2
        local defense = (props.base_defense or 5) + level
        local max_hp = props.max_hp or 100
        local max_mana = props.max_mana or 50
        local crit = (props.crit_rate or 10) + level * 0.5
        local dodge = (props.dodge_rate or 5) + level * 0.3
        local spell = (props.spell_power or 0) + level * 1.5

        -- 初始化状态
        local current_hp = state.current_hp or max_hp
        local current_mana = state.current_mana or max_mana

        -- 受到伤害处理
        local damage = inputs.damage_in or 0
        if damage > 0 then
            -- 闪避判定
            local dodge_roll = math.random(1, 100)
            if dodge_roll <= dodge then
                print("[角色] 闪避成功!")
            else
                local actual_dmg = math.max(0, damage - defense)
                current_hp = math.max(0, current_hp - actual_dmg)
                print("[角色] 受到 " .. actual_dmg .. " 伤害, HP: " .. current_hp)
            end
        end

        -- 法力恢复（每tick恢复1点）
        current_mana = math.min(max_mana, current_mana + 1)

        -- 保存状态
        state.current_hp = current_hp
        state.current_mana = current_mana
        self.state = state

        return {
            attack = attack,
            defense = defense,
            hp = current_hp,
            max_hp = max_hp,
            mana = current_mana,
            max_mana = max_mana,
            crit_rate = crit,
            dodge_rate = dodge,
            spell_power = spell,
            action_out = inputs.action_trigger,
            is_alive = current_hp > 0
        }
    end
}

