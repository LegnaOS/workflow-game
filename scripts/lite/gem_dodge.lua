-- 闪避宝石Block
-- 镶嵌到武器增加闪避率

return {
    meta = {
        id = "lite.gem_dodge",
        name = "闪避宝石",
        category = "Lite",
        color = "#8BC34A",
        description = "绿色宝石，增加闪避率"
    },

    properties = {
        { id = "level", name = "等级", type = "number", default = 1, min = 1, max = 10 },
        { id = "base_dodge", name = "基础闪避%", type = "number", default = 3, min = 1 }
    },

    inputs = {},

    outputs = {
        { id = "gem_out", name = "宝石数据", type = "any", default = nil },
        { id = "dodge", name = "闪避加成%", type = "number", default = 3 }
    },

    execute = function(self, inputs)
        local props = self.properties
        local dodge = props.base_dodge * props.level
        
        return {
            gem_out = { attack = 0, crit = 0, dodge = dodge },
            dodge = dodge
        }
    end
}

