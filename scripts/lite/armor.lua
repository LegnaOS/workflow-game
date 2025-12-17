-- 防具Block
-- 提供防御力和生命加成

return {
    meta = {
        id = "lite.armor",
        name = "防具",
        category = "Lite",
        color = "#607D8B",
        description = "防具装备，增加防御和生命"
    },

    properties = {
        { id = "name", name = "名称", type = "string", default = "布衣" },
        { id = "base_defense", name = "基础防御", type = "number", default = 3, min = 0 },
        { id = "base_hp", name = "生命加成", type = "number", default = 20, min = 0 },
        { id = "quality", name = "品质", type = "number", default = 1, min = 1, max = 5 }
    },

    inputs = {},

    outputs = {
        { id = "armor_out", name = "防具数据", type = "any", default = nil },
        { id = "defense", name = "防御力", type = "number", default = 3 },
        { id = "hp_bonus", name = "生命加成", type = "number", default = 20 }
    },

    execute = function(self, inputs)
        local props = self.properties
        
        local def = props.base_defense * props.quality
        local hp = props.base_hp * props.quality
        
        local armor_data = {
            defense = def,
            hp_bonus = hp
        }
        
        return {
            armor_out = armor_data,
            defense = def,
            hp_bonus = hp
        }
    end
}

