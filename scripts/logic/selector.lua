-- 多路选择器Block
-- 根据索引选择多个输入中的一个输出

return {
    meta = {
        id = "logic.selector",
        name = "多路选择",
        category = "逻辑",
        description = "根据索引选择多个输入中的一个",
        color = "#FF9800"
    },

    properties = {},

    inputs = {
        { id = "index", name = "索引", type = "number", default = 1 },
        { id = "in1", name = "选项1", type = "any" },
        { id = "in2", name = "选项2", type = "any" },
        { id = "in3", name = "选项3", type = "any" },
        { id = "in4", name = "选项4", type = "any" },
        { id = "default", name = "默认值", type = "any" }
    },

    outputs = {
        { id = "result", name = "结果", type = "any" },
        { id = "selected", name = "选中索引", type = "number" }
    },

    execute = function(self, inputs)
        local index = math.floor(inputs.index or 1)
        local options = { inputs.in1, inputs.in2, inputs.in3, inputs.in4 }

        local result = inputs.default
        local selected = 0

        if index >= 1 and index <= 4 then
            if options[index] ~= nil then
                result = options[index]
                selected = index
            end
        end

        return { result = result, selected = selected }
    end
}

