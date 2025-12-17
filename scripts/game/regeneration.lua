-- 回合恢复Block
-- 每回合自动恢复生命和法力

return {
    meta = {
        id = "game.regeneration",
        name = "回合恢复",
        category = "游戏",
        description = "每回合自动恢复生命值和法力值",
        color = "#00BCD4"
    },

    properties = {
        { id = "hp_regen", name = "生命恢复/回合", type = "number", default = 5, min = 0 },
        { id = "mana_regen", name = "法力恢复/回合", type = "number", default = 3, min = 0 },
        { id = "hp_regen_percent", name = "生命恢复%", type = "number", default = 0, min = 0, max = 100 },
        { id = "mana_regen_percent", name = "法力恢复%", type = "number", default = 0, min = 0, max = 100 }
    },

    inputs = {
        { id = "turn_trigger", name = "回合触发", type = "event" },
        { id = "current_hp", name = "当前生命", type = "number", default = 100 },
        { id = "max_hp", name = "最大生命", type = "number", default = 100 },
        { id = "current_mana", name = "当前法力", type = "number", default = 50 },
        { id = "max_mana", name = "最大法力", type = "number", default = 50 }
    },

    outputs = {
        { id = "new_hp", name = "恢复后生命", type = "number" },
        { id = "new_mana", name = "恢复后法力", type = "number" },
        { id = "hp_healed", name = "本次治疗", type = "number" },
        { id = "mana_restored", name = "本次恢复", type = "number" },
        { id = "regen_event", name = "恢复事件", type = "event" }
    },

    execute = function(self, inputs)
        local props = self.properties
        local trigger = inputs.turn_trigger

        local current_hp = inputs.current_hp or 100
        local max_hp = inputs.max_hp or 100
        local current_mana = inputs.current_mana or 50
        local max_mana = inputs.max_mana or 50

        local hp_healed = 0
        local mana_restored = 0
        local regen_event = nil

        if trigger then
            -- 计算恢复量
            local hp_flat = props.hp_regen or 5
            local hp_pct = (props.hp_regen_percent or 0) / 100 * max_hp
            local mana_flat = props.mana_regen or 3
            local mana_pct = (props.mana_regen_percent or 0) / 100 * max_mana

            -- 应用恢复
            local hp_total = hp_flat + hp_pct
            local mana_total = mana_flat + mana_pct

            local new_hp = math.min(max_hp, current_hp + hp_total)
            local new_mana = math.min(max_mana, current_mana + mana_total)

            hp_healed = new_hp - current_hp
            mana_restored = new_mana - current_mana
            current_hp = new_hp
            current_mana = new_mana
            regen_event = true

            if hp_healed > 0 or mana_restored > 0 then
                print(string.format("[恢复] HP+%.0f(%.0f) MP+%.0f(%.0f)", 
                    hp_healed, current_hp, mana_restored, current_mana))
            end
        end

        return {
            new_hp = current_hp,
            new_mana = current_mana,
            hp_healed = hp_healed,
            mana_restored = mana_restored,
            regen_event = regen_event
        }
    end
}

