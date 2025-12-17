-- 加法节点（多输入）
return {
    meta = {
        id = "math.add",
        name = "加法",
        category = "数学",
        description = "将多个数字相加，支持4个输入",
        color = "#4CAF50"
    },

    inputs = {
        { id = "a", name = "A", type = "number", default = 0 },
        { id = "b", name = "B", type = "number", default = 0 },
        { id = "c", name = "C", type = "number", default = 0 },
        { id = "d", name = "D", type = "number", default = 0 }
    },

    outputs = {
        { id = "result", name = "结果", type = "number" },
        { id = "count", name = "输入数", type = "number" }
    },

    properties = {},

    execute = function(self, inputs)
        local sum = 0
        local count = 0
        for _, key in ipairs({"a", "b", "c", "d"}) do
            local v = inputs[key]
            if v ~= nil and v ~= 0 then
                sum = sum + v
                count = count + 1
            elseif v == 0 then
                -- 0也算有效输入
                count = count + 1
            end
        end
        return { result = sum, count = count }
    end
}

