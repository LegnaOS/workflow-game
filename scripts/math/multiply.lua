-- 乘法节点（多输入）
return {
    meta = {
        id = "math.multiply",
        name = "乘法",
        category = "数学",
        description = "将多个数字相乘，支持4个输入",
        color = "#4CAF50"
    },

    inputs = {
        { id = "a", name = "A", type = "number", default = 1 },
        { id = "b", name = "B", type = "number", default = 1 },
        { id = "c", name = "C", type = "number", default = 1 },
        { id = "d", name = "D", type = "number", default = 1 }
    },

    outputs = {
        { id = "result", name = "结果", type = "number" },
        { id = "count", name = "输入数", type = "number" }
    },

    properties = {},

    execute = function(self, inputs)
        local product = 1
        local count = 0
        for _, key in ipairs({"a", "b", "c", "d"}) do
            local v = inputs[key]
            if v ~= nil and v ~= 1 then
                product = product * v
                count = count + 1
            elseif v == 1 then
                count = count + 1
            end
        end
        return { result = product, count = count }
    end
}

