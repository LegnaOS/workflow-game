-- 武器Block
-- 3个宝石镶嵌孔，提供攻击力加成

return {
    meta = {
        id = "lite.weapon",
        name = "武器",
        category = "Lite",
        color = "#FF9800",
        description = "武器装备，有3个宝石镶嵌孔"
    },

    properties = {
        { id = "name", name = "名称", type = "string", default = "木剑" },
        { id = "base_attack", name = "基础攻击", type = "number", default = 5, min = 0 },
        { id = "quality", name = "品质", type = "number", default = 1, min = 1, max = 5 }
    },

    inputs = {
        -- 3个宝石插槽
        { id = "gem_1", name = "宝石1", type = "any", default = nil },
        { id = "gem_2", name = "宝石2", type = "any", default = nil },
        { id = "gem_3", name = "宝石3", type = "any", default = nil }
    },

    outputs = {
        -- 武器属性（输出给英雄）
        { id = "weapon_out", name = "武器数据", type = "any", default = nil },
        { id = "attack", name = "攻击力", type = "number", default = 5 },
        { id = "crit", name = "暴击率%", type = "number", default = 0 },
        { id = "dodge", name = "闪避率%", type = "number", default = 0 }
    },

    execute = function(self, inputs)
        local props = self.properties
        
        -- 基础属性（品质影响基础攻击）
        local base_atk = props.base_attack * props.quality
        local total_atk = base_atk
        local total_crit = 0
        local total_dodge = 0
        
        -- 收集宝石加成
        local gems = { inputs.gem_1, inputs.gem_2, inputs.gem_3 }
        for _, gem in ipairs(gems) do
            if gem and type(gem) == "table" then
                total_atk = total_atk + (gem.attack or 0)
                total_crit = total_crit + (gem.crit or 0)
                total_dodge = total_dodge + (gem.dodge or 0)
            end
        end
        
        -- 构建武器数据
        local weapon_data = {
            attack = total_atk,
            crit = total_crit,
            dodge = total_dodge
        }
        
        return {
            weapon_out = weapon_data,
            attack = total_atk,
            crit = total_crit,
            dodge = total_dodge
        }
    end
}

