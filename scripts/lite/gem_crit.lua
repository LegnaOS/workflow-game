-- 暴击宝石Block
-- 镶嵌到武器增加暴击率

return {
    meta = {
        id = "lite.gem_crit",
        name = "暴击宝石",
        category = "Lite",
        color = "#FF5722",
        description = "橙色宝石，增加暴击率",
        hideable = true
    },

    properties = {
        { id = "level", name = "等级", type = "number", default = 1, min = 1, max = 10 },
        { id = "base_crit", name = "基础暴击%", type = "number", default = 5, min = 1 }
    },

    inputs = {},

    outputs = {
        { id = "gem_out", name = "宝石数据", type = "any", default = nil },
        { id = "crit", name = "暴击加成%", type = "number", default = 5 }
    },

    execute = function(self, inputs)
        local props = self.properties
        local crit = props.base_crit * props.level
        
        return {
            gem_out = { attack = 0, crit = crit, dodge = 0 },
            crit = crit
        }
    end
}

