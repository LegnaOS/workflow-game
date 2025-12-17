-- 条件切换Block
-- 根据条件选择输出

return {
    meta = {
        id = "util.switch",
        name = "条件切换",
        category = "工具",
        description = "根据条件选择输出A或B",
        color = "#FF9800"
    },

    properties = {},

    inputs = {
        { id = "condition", name = "条件", type = "boolean", default = false },
        { id = "value_true", name = "真值", type = "any", default = nil },
        { id = "value_false", name = "假值", type = "any", default = nil }
    },

    outputs = {
        { id = "result", name = "结果", type = "any", default = nil }
    },

    execute = function(self, inputs)
        local condition = inputs.condition
        local result
        if condition then
            result = inputs.value_true
        else
            result = inputs.value_false
        end
        return { result = result }
    end
}

