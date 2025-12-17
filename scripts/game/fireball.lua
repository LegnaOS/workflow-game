-- 火球术Block
-- 展示高阶复杂函数运算：冷却、法力消耗、暴击、元素伤害

return {
    meta = {
        id = "game.fireball",
        name = "火球术",
        category = "游戏",
        description = "法术技能：消耗法力释放火球，有冷却时间和暴击机制",
        color = "#FF5722"
    },

    properties = {
        { id = "base_damage", name = "基础伤害", type = "number", default = 30, min = 1 },
        { id = "mana_cost", name = "法力消耗", type = "number", default = 20, min = 0 },
        { id = "cooldown", name = "冷却时间(秒)", type = "number", default = 3.0, min = 0 },
        { id = "crit_rate", name = "暴击率%", type = "number", default = 15, min = 0, max = 100 },
        { id = "crit_multiplier", name = "暴击倍率", type = "number", default = 2.0, min = 1 },
        { id = "burn_chance", name = "灼烧几率%", type = "number", default = 30, min = 0, max = 100 },
        { id = "burn_damage", name = "灼烧伤害/秒", type = "number", default = 5, min = 0 }
    },

    inputs = {
        { id = "cast_trigger", name = "施法触发", type = "event" },
        { id = "caster_attack", name = "施法者攻击力", type = "number", default = 10 },
        { id = "caster_mana", name = "施法者法力", type = "number", default = 100 },
        { id = "spell_power", name = "法术强度", type = "number", default = 0 }
    },

    outputs = {
        { id = "damage_out", name = "输出伤害", type = "number", default = 0 },
        { id = "is_crit", name = "是否暴击", type = "boolean", default = false },
        { id = "mana_used", name = "消耗法力", type = "number", default = 0 },
        { id = "is_burning", name = "造成灼烧", type = "boolean", default = false },
        { id = "burn_dps", name = "灼烧DPS", type = "number", default = 0 },
        { id = "is_ready", name = "技能就绪", type = "boolean", default = true },
        { id = "remaining_cd", name = "剩余冷却", type = "number", default = 0 },
        { id = "cast_event", name = "施法事件", type = "event" }
    },

    execute = function(self, inputs)
        local props = self.properties
        local state = self.state or {}
        local cast = inputs.cast_trigger
        local caster_atk = inputs.caster_attack or 10
        local caster_mana = inputs.caster_mana or 100
        local spell_power = inputs.spell_power or 0

        -- 获取当前时间（使用简单计数器模拟）
        local current_tick = (state.tick or 0) + 1
        local last_cast_tick = state.last_cast_tick or -9999
        local ticks_since_cast = current_tick - last_cast_tick
        local cd_ticks = props.cooldown * 10  -- 假设10tick/秒
        local remaining_cd = math.max(0, cd_ticks - ticks_since_cast) / 10
        local is_ready = remaining_cd <= 0

        -- 默认输出
        local result = {
            damage_out = 0,
            is_crit = false,
            mana_used = 0,
            is_burning = false,
            burn_dps = 0,
            is_ready = is_ready,
            remaining_cd = remaining_cd,
            cast_event = nil
        }

        -- 施法逻辑
        if cast and is_ready and caster_mana >= props.mana_cost then
            -- 计算基础伤害 = 基础 + 攻击力*0.5 + 法术强度*1.2
            local damage = props.base_damage + caster_atk * 0.5 + spell_power * 1.2

            -- 暴击判定
            local crit_roll = math.random(1, 100)
            local is_crit = crit_roll <= props.crit_rate
            if is_crit then
                damage = damage * props.crit_multiplier
                print("[火球术] 暴击! " .. math.floor(damage) .. " 点伤害!")
            else
                print("[火球术] 命中! " .. math.floor(damage) .. " 点伤害")
            end

            -- 灼烧判定
            local burn_roll = math.random(1, 100)
            local is_burning = burn_roll <= props.burn_chance
            local burn_dps = is_burning and props.burn_damage or 0
            if is_burning then
                print("[火球术] 目标被灼烧! 每秒 " .. burn_dps .. " 点伤害")
            end

            result.damage_out = math.floor(damage)
            result.is_crit = is_crit
            result.mana_used = props.mana_cost
            result.is_burning = is_burning
            result.burn_dps = burn_dps
            result.cast_event = true

            -- 更新冷却
            state.last_cast_tick = current_tick
            result.is_ready = false
            result.remaining_cd = props.cooldown
        elseif cast and not is_ready then
            print("[火球术] 技能冷却中... " .. string.format("%.1f", remaining_cd) .. "秒")
        elseif cast and caster_mana < props.mana_cost then
            print("[火球术] 法力不足! 需要 " .. props.mana_cost .. ", 当前 " .. caster_mana)
        end

        -- 保存状态
        state.tick = current_tick
        self.state = state

        return result
    end
}

