-- 通用计算器（多输入多输出）
return {
    meta = {
        id = "math.calc",
        name = "计算器",
        category = "数学",
        description = "通用计算器，支持多输入和多种运算",
        color = "#2196F3"
    },

    properties = {
        { id = "operation", name = "运算", type = "string", default = "add" }
        -- add, sub, mul, div, avg, max, min, pow
    },

    inputs = {
        { id = "a", name = "A", type = "number", default = 0 },
        { id = "b", name = "B", type = "number", default = 0 },
        { id = "c", name = "C", type = "number", default = 0 },
        { id = "d", name = "D", type = "number", default = 0 }
    },

    outputs = {
        { id = "result", name = "结果", type = "number" },
        { id = "sum", name = "和", type = "number" },
        { id = "avg", name = "平均", type = "number" },
        { id = "max", name = "最大", type = "number" },
        { id = "min", name = "最小", type = "number" }
    },

    execute = function(self, inputs)
        local op = self.properties.operation or "add"
        local values = {}

        -- 收集非nil值
        for _, key in ipairs({"a", "b", "c", "d"}) do
            local v = inputs[key]
            if v ~= nil then
                table.insert(values, v)
            end
        end

        if #values == 0 then
            return { result = 0, sum = 0, avg = 0, max = 0, min = 0 }
        end

        -- 计算统计值
        local sum = 0
        local max_val = values[1]
        local min_val = values[1]
        for _, v in ipairs(values) do
            sum = sum + v
            if v > max_val then max_val = v end
            if v < min_val then min_val = v end
        end
        local avg = sum / #values

        -- 主运算
        local result = 0
        if op == "add" then
            result = sum
        elseif op == "sub" then
            result = values[1]
            for i = 2, #values do
                result = result - values[i]
            end
        elseif op == "mul" then
            result = 1
            for _, v in ipairs(values) do
                result = result * v
            end
        elseif op == "div" then
            result = values[1]
            for i = 2, #values do
                if values[i] ~= 0 then
                    result = result / values[i]
                end
            end
        elseif op == "avg" then
            result = avg
        elseif op == "max" then
            result = max_val
        elseif op == "min" then
            result = min_val
        elseif op == "pow" then
            result = values[1]
            for i = 2, #values do
                result = result ^ values[i]
            end
        end

        return {
            result = result,
            sum = sum,
            avg = avg,
            max = max_val,
            min = min_val
        }
    end
}

