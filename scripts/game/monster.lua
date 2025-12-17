-- 怪物Block
-- 敌方怪物，有属性并掉落金币

return {
    meta = {
        id = "game.monster",
        name = "怪物",
        category = "游戏",
        color = "#9C27B0",
        description = "敌方怪物，有生命值和掉落金币"
    },

    properties = {
        { id = "name", name = "怪物名称", type = "string", default = "史莱姆" },
        { id = "max_hp", name = "最大生命", type = "number", default = 50, min = 1 },
        { id = "attack", name = "攻击力", type = "number", default = 8, min = 0 },
        { id = "defense", name = "防御力", type = "number", default = 3, min = 0 },
        { id = "dodge_rate", name = "闪避率%", type = "number", default = 10, min = 0, max = 100 },
        { id = "crit_rate", name = "暴击率%", type = "number", default = 5, min = 0, max = 100 },
        { id = "gold_drop", name = "掉落金币", type = "number", default = 10, min = 0 },
        { id = "exp_drop", name = "掉落经验", type = "number", default = 20, min = 0 },
        { id = "respawn_time", name = "重生时间(秒)", type = "number", default = 3, min = 0 }
    },

    inputs = {
        { id = "damage_in", name = "受到伤害", type = "number", default = 0 },
        { id = "attack_event", name = "被攻击事件", type = "event", default = nil }
    },

    outputs = {
        { id = "current_hp", name = "当前生命", type = "number", default = 50 },
        { id = "monster_attack", name = "攻击力", type = "number", default = 8 },
        { id = "monster_defense", name = "防御力", type = "number", default = 3 },
        { id = "dodge_rate", name = "闪避率", type = "number", default = 10 },
        { id = "gold_reward", name = "金币奖励", type = "number", default = 0 },
        { id = "exp_reward", name = "经验奖励", type = "number", default = 0 },
        { id = "is_dead", name = "是否死亡", type = "boolean", default = false },
        { id = "attack_out", name = "攻击事件", type = "event", default = nil }
    },

    execute = function(self, inputs)
        local props = self.properties
        local state = self.state or {}

        local max_hp = props.max_hp or 50
        local defense = props.defense or 3
        local dodge = props.dodge_rate or 10
        local damage = inputs.damage_in or 0
        local attack_event = inputs.attack_event

        -- 从state获取当前HP和重生计时
        local current_hp = state.current_hp
        local dead_ticks = state.dead_ticks or 0
        local respawn_ticks = (props.respawn_time or 3) * 10

        -- 死亡状态处理
        if current_hp ~= nil and current_hp <= 0 then
            dead_ticks = dead_ticks + 1
            if dead_ticks >= respawn_ticks then
                current_hp = max_hp
                dead_ticks = 0
                print("[怪物] 重生! HP: " .. current_hp)
            end
        elseif current_hp == nil then
            current_hp = max_hp
        end

        local gold_reward = 0
        local exp_reward = 0

        -- 只有当收到攻击事件且存活时才处理
        if attack_event and current_hp > 0 then
            -- 闪避判定
            local dodge_roll = math.random(1, 100)
            if dodge_roll <= dodge then
                print("[怪物] 闪避了攻击!")
            else
                local actual_damage = math.max(0, damage - defense)
                current_hp = math.max(0, current_hp - actual_damage)
                print("[怪物] 受到 " .. actual_damage .. " 点伤害, 剩余HP: " .. current_hp)

                -- 死亡时掉落
                if current_hp <= 0 then
                    gold_reward = props.gold_drop or 10
                    exp_reward = props.exp_drop or 20
                    dead_ticks = 0
                    print("[怪物] 死亡! 掉落 " .. gold_reward .. " 金币, " .. exp_reward .. " 经验")
                end
            end
        end

        local is_dead = current_hp <= 0

        -- 保存状态
        state.current_hp = current_hp
        state.dead_ticks = dead_ticks
        self.state = state

        return {
            current_hp = current_hp,
            monster_attack = props.attack or 8,
            monster_defense = defense,
            dodge_rate = dodge,
            gold_reward = gold_reward,
            exp_reward = exp_reward,
            is_dead = is_dead,
            attack_out = not is_dead and attack_event or nil
        }
    end
}

