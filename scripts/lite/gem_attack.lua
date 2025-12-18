-- 攻击宝石Block
-- 镶嵌到武器增加攻击力

return {
    meta = {
        id = "lite.gem_attack",
        name = "攻击宝石",
        category = "Lite",
        color = "#F44336",
        description = "红色宝石，增加攻击力",
        hideable = true
    },

    properties = {
        { id = "level", name = "等级", type = "number", default = 1, min = 1, max = 10 },
        { id = "base_attack", name = "基础攻击", type = "number", default = 3, min = 1 }
    },

    inputs = {},

    outputs = {
        { id = "gem_out", name = "宝石数据", type = "any", default = nil },
        { id = "attack", name = "攻击加成", type = "number", default = 3 }
    },

    execute = function(self, inputs)
        local props = self.properties
        local atk = props.base_attack * props.level
        
        return {
            gem_out = { attack = atk, crit = 0, dodge = 0 },
            attack = atk
        }
    end
}

