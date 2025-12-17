-- 英雄Block
-- 2装备插槽(武器/防具) + 4技能插槽 + 行动输出

return {
    meta = {
        id = "lite.hero",
        name = "英雄",
        category = "Lite",
        color = "#4CAF50",
        description = "放置英雄，装备武器防具，释放技能"
    },

    properties = {
        { id = "name", name = "名称", type = "string", default = "新人小李" },
        { id = "base_hp", name = "基础生命", type = "number", default = 100, min = 1 },
        { id = "base_mp", name = "基础魔法", type = "number", default = 50, min = 0 },
        { id = "base_atk", name = "基础攻击", type = "number", default = 10, min = 1 },
        { id = "attack_speed", name = "攻速(次/秒)", type = "number", default = 1.0, min = 0.1, max = 10 },
        { id = "hp_regen", name = "生命恢复/秒", type = "number", default = 1, min = 0 },
        { id = "mp_regen", name = "魔法恢复/秒", type = "number", default = 2, min = 0 }
    },

    inputs = {
        -- 装备插槽
        { id = "weapon", name = "武器", type = "any", default = nil },
        { id = "armor", name = "防具", type = "any", default = nil },
        -- 技能插槽
        { id = "skill_1", name = "技能1", type = "any", default = nil },
        { id = "skill_2", name = "技能2", type = "any", default = nil },
        { id = "skill_3", name = "技能3", type = "any", default = nil },
        { id = "skill_4", name = "技能4", type = "any", default = nil },
        -- 受击输入（来自Boss反击）
        { id = "damage_in", name = "受到伤害", type = "number", default = 0 },
        { id = "hit_event", name = "受击事件", type = "event", default = nil }
    },

    outputs = {
        -- 状态输出
        { id = "hp", name = "当前生命", type = "number", default = 100 },
        { id = "max_hp", name = "最大生命", type = "number", default = 100 },
        { id = "mp", name = "当前魔法", type = "number", default = 50 },
        { id = "max_mp", name = "最大魔法", type = "number", default = 50 },
        { id = "atk", name = "攻击力", type = "number", default = 10 },
        { id = "def", name = "防御力", type = "number", default = 0 },
        { id = "crit", name = "暴击率%", type = "number", default = 0 },
        { id = "dodge", name = "闪避率%", type = "number", default = 0 },
        { id = "dps", name = "DPS", type = "number", default = 10 },
        { id = "is_alive", name = "存活", type = "boolean", default = true },
        -- 行动输出
        { id = "action_damage", name = "行动伤害", type = "number", default = 0 },
        { id = "action_event", name = "行动事件", type = "event", default = nil }
    },

    execute = function(self, inputs)
        local props = self.properties
        local state = self.state or {}
        
        -- 从武器获取属性（确保是表）
        local weapon = (type(inputs.weapon) == "table") and inputs.weapon or {}
        local weapon_atk = weapon.attack or 0
        local weapon_crit = weapon.crit or 0
        local weapon_dodge = weapon.dodge or 0

        -- 从防具获取属性（确保是表）
        local armor = (type(inputs.armor) == "table") and inputs.armor or {}
        local armor_def = armor.defense or 0
        local armor_hp = armor.hp_bonus or 0
        
        -- 计算最终属性
        local max_hp = props.base_hp + armor_hp
        local max_mp = props.base_mp
        local atk = props.base_atk + weapon_atk
        local def = armor_def
        local crit = weapon_crit
        local dodge = weapon_dodge
        
        -- 初始化状态
        local hp = state.hp or max_hp
        local mp = state.mp or max_mp
        local tick = state.tick or 0
        tick = tick + 1
        
        -- 生命/魔法恢复 (每10tick = 1秒)
        if tick % 10 == 0 then
            hp = math.min(max_hp, hp + props.hp_regen)
            mp = math.min(max_mp, mp + props.mp_regen)
        end
        
        -- 受击处理
        if inputs.hit_event and inputs.damage_in > 0 then
            local dmg = inputs.damage_in
            -- 闪避判定
            if math.random(100) <= dodge then
                print("[" .. props.name .. "] 闪避!")
            else
                local actual = math.max(1, dmg - def)
                hp = math.max(0, hp - actual)
                print("[" .. props.name .. "] 受到 " .. actual .. " 伤害, HP: " .. hp)
                state._animation = { x = -15, y = 0, speed = 300 }
            end
        else
            state._animation = { x = 0, y = 0, speed = 150 }
        end
        
        -- 攻击判定 (根据攻速)
        local attack_interval = math.floor(10 / props.attack_speed)
        local action_event = nil
        local action_damage = 0
        
        if hp > 0 and tick % attack_interval == 0 then
            -- 暴击判定
            local is_crit = math.random(100) <= crit
            action_damage = is_crit and math.floor(atk * 2) or atk
            action_event = true
            state._animation = { x = 20, y = 0, speed = 250 }
            
            if is_crit then
                print("[" .. props.name .. "] 暴击! " .. action_damage)
            end
        end
        
        -- 计算DPS
        local dps = atk * props.attack_speed * (1 + crit / 100)
        
        state.hp = hp
        state.mp = mp
        state.tick = tick
        self.state = state
        
        return {
            hp = hp, max_hp = max_hp,
            mp = mp, max_mp = max_mp,
            atk = atk, def = def,
            crit = crit, dodge = dodge,
            dps = math.floor(dps),
            is_alive = hp > 0,
            action_damage = action_damage,
            action_event = action_event
        }
    end
}

