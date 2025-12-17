-- Boss Block
-- 可被攻击，有反击能力

return {
    meta = {
        id = "lite.boss",
        name = "Boss",
        category = "Lite",
        color = "#E91E63",
        description = "Boss怪物，可被攻击，可反击"
    },

    properties = {
        { id = "name", name = "名称", type = "string", default = "稻草人" },
        { id = "max_hp", name = "最大生命", type = "number", default = 500, min = 1 },
        { id = "attack", name = "攻击力", type = "number", default = 15, min = 0 },
        { id = "defense", name = "防御力", type = "number", default = 5, min = 0 },
        { id = "attack_speed", name = "攻速(次/秒)", type = "number", default = 0.5, min = 0.1, max = 5 },
        { id = "gold_drop", name = "掉落金币", type = "number", default = 100, min = 0 },
        { id = "respawn_time", name = "重生(秒)", type = "number", default = 5, min = 1 }
    },

    inputs = {
        -- 受击输入（来自英雄行动）
        { id = "damage_in", name = "受到伤害", type = "number", default = 0 },
        { id = "hit_event", name = "受击事件", type = "event", default = nil }
    },

    outputs = {
        -- 状态输出
        { id = "hp", name = "当前生命", type = "number", default = 500 },
        { id = "max_hp", name = "最大生命", type = "number", default = 500 },
        { id = "hp_percent", name = "生命%", type = "number", default = 100 },
        { id = "is_alive", name = "存活", type = "boolean", default = true },
        { id = "dps_taken", name = "承受DPS", type = "number", default = 0 },
        -- 掉落输出
        { id = "gold_reward", name = "金币奖励", type = "number", default = 0 },
        { id = "kill_event", name = "击杀事件", type = "event", default = nil },
        -- 反击输出（连接到英雄的受击输入）
        { id = "counter_damage", name = "反击伤害", type = "number", default = 0 },
        { id = "counter_event", name = "反击事件", type = "event", default = nil }
    },

    execute = function(self, inputs)
        local props = self.properties
        local state = self.state or {}
        
        local max_hp = props.max_hp
        local hp = state.hp or max_hp
        local tick = state.tick or 0
        local dead_ticks = state.dead_ticks or 0
        local total_damage = state.total_damage or 0
        local damage_window = state.damage_window or {}
        
        tick = tick + 1
        
        -- 重生逻辑
        local respawn_ticks = props.respawn_time * 10
        if hp <= 0 then
            dead_ticks = dead_ticks + 1
            if dead_ticks >= respawn_ticks then
                hp = max_hp
                dead_ticks = 0
                total_damage = 0
                damage_window = {}
                print("[Boss] " .. props.name .. " 重生!")
            end
            state.hp = hp
            state.dead_ticks = dead_ticks
            state.tick = tick
            self.state = state
            return {
                hp = hp, max_hp = max_hp, hp_percent = 0,
                is_alive = false, dps_taken = 0,
                gold_reward = 0, kill_event = nil,
                counter_damage = 0, counter_event = nil
            }
        end
        
        -- 受击处理
        local gold_reward = 0
        local kill_event = nil
        
        if inputs.hit_event and inputs.damage_in > 0 then
            local dmg = inputs.damage_in
            local actual = math.max(1, dmg - props.defense)
            hp = math.max(0, hp - actual)
            total_damage = total_damage + actual
            
            -- 记录伤害用于计算DPS (保留最近10秒)
            table.insert(damage_window, { tick = tick, damage = actual })
            
            print("[Boss] " .. props.name .. " 受到 " .. actual .. " 伤害, HP: " .. hp .. "/" .. max_hp)
            state._animation = { x = -10, y = 0, speed = 400 }
            
            -- 死亡判定
            if hp <= 0 then
                gold_reward = props.gold_drop
                kill_event = true
                dead_ticks = 0
                print("[Boss] " .. props.name .. " 被击败! 掉落 " .. gold_reward .. " 金币")
                state._animation = { x = 0, y = 20, speed = 100 }
            end
        else
            state._animation = { x = 0, y = 0, speed = 150 }
        end
        
        -- 计算DPS (最近10秒的伤害)
        local dps_window = 100 -- 10秒 = 100 ticks
        local recent_damage = 0
        local new_window = {}
        for _, entry in ipairs(damage_window) do
            if tick - entry.tick < dps_window then
                recent_damage = recent_damage + entry.damage
                table.insert(new_window, entry)
            end
        end
        damage_window = new_window
        local dps_taken = math.floor(recent_damage / 10)
        
        -- 反击逻辑
        local attack_interval = math.floor(10 / props.attack_speed)
        local counter_damage = 0
        local counter_event = nil
        
        if hp > 0 and tick % attack_interval == 0 then
            counter_damage = props.attack
            counter_event = true
            print("[Boss] " .. props.name .. " 反击! " .. counter_damage)
        end
        
        state.hp = hp
        state.tick = tick
        state.dead_ticks = dead_ticks
        state.total_damage = total_damage
        state.damage_window = damage_window
        self.state = state
        
        return {
            hp = hp, max_hp = max_hp,
            hp_percent = math.floor(hp / max_hp * 100),
            is_alive = hp > 0,
            dps_taken = dps_taken,
            gold_reward = gold_reward,
            kill_event = kill_event,
            counter_damage = counter_damage,
            counter_event = counter_event
        }
    end
}

