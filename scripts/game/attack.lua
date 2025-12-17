-- 攻击行动Block
-- 接收角色行动，计算伤害输出

return {
    meta = {
        id = "game.attack",
        name = "攻击",
        category = "游戏",
        color = "#F44336",
        description = "攻击行动，计算对目标的伤害"
    },

    properties = {
        {
            id = "skill_name",
            name = "技能名称",
            type = "string",
            default = "普通攻击"
        },
        {
            id = "damage_multiplier",
            name = "伤害倍率",
            type = "number",
            default = 1.0,
            min = 0.1,
            max = 10.0
        },
        {
            id = "ignore_defense",
            name = "无视防御",
            type = "number",
            default = 0,
            min = 0,
            max = 100
        }
    },

    inputs = {
        {
            id = "action_in",
            name = "行动输入",
            type = "event",
            default = nil
        },
        {
            id = "attacker_attack",
            name = "攻击者攻击力",
            type = "number",
            default = 10
        }
    },

    outputs = {
        {
            id = "damage_out",
            name = "伤害输出",
            type = "number",
            default = 0
        },
        {
            id = "attack_event",
            name = "攻击事件",
            type = "event",
            default = nil
        }
    },

    execute = function(self, inputs)
        local properties = self.properties
        local attack = inputs.attacker_attack or 10
        local multiplier = properties.damage_multiplier or 1.0
        local damage = math.floor(attack * multiplier)

        -- 只有当收到行动输入时才触发攻击事件
        local attack_event = nil
        if inputs.action_in then
            attack_event = true
            print("[攻击] 造成 " .. damage .. " 点伤害")
        end

        return {
            damage_out = damage,
            attack_event = attack_event
        }
    end
}

