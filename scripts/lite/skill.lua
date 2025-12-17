-- 技能Block
-- 消耗魔法造成额外伤害

return {
    meta = {
        id = "lite.skill",
        name = "技能",
        category = "Lite",
        color = "#9C27B0",
        description = "主动技能，消耗魔法造成伤害"
    },

    properties = {
        { id = "name", name = "技能名", type = "string", default = "火球术" },
        { id = "damage", name = "技能伤害", type = "number", default = 20, min = 0 },
        { id = "mp_cost", name = "魔法消耗", type = "number", default = 10, min = 0 },
        { id = "cooldown", name = "冷却(秒)", type = "number", default = 3, min = 0.5 }
    },

    inputs = {
        { id = "mp_in", name = "当前魔法", type = "number", default = 0 },
        { id = "trigger", name = "触发", type = "event", default = nil }
    },

    outputs = {
        { id = "skill_out", name = "技能数据", type = "any", default = nil },
        { id = "damage_out", name = "技能伤害", type = "number", default = 0 },
        { id = "mp_cost", name = "魔法消耗", type = "number", default = 0 },
        { id = "is_ready", name = "可用", type = "boolean", default = true }
    },

    execute = function(self, inputs)
        local props = self.properties
        local state = self.state or {}
        
        local cd_ticks = math.floor(props.cooldown * 10)
        local current_cd = state.cooldown or 0
        
        -- 冷却计时
        if current_cd > 0 then
            current_cd = current_cd - 1
        end
        
        local damage_out = 0
        local mp_used = 0
        local is_ready = current_cd <= 0
        
        -- 检查是否可以释放
        if inputs.trigger and is_ready and inputs.mp_in >= props.mp_cost then
            damage_out = props.damage
            mp_used = props.mp_cost
            current_cd = cd_ticks
            print("[技能] " .. props.name .. " 造成 " .. damage_out .. " 伤害")
        end
        
        state.cooldown = current_cd
        self.state = state
        
        return {
            skill_out = { damage = props.damage, mp_cost = props.mp_cost },
            damage_out = damage_out,
            mp_cost = mp_used,
            is_ready = is_ready
        }
    end
}

