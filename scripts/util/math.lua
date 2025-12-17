-- 数学运算Block
-- 通用数学运算，支持加减乘除和自定义公式

return {
    meta = {
        id = "util.math",
        name = "数学运算",
        category = "工具",
        description = "通用数学运算，支持四则运算和自定义公式",
        color = "#2196F3"
    },

    properties = {
        { id = "operation", name = "运算", type = "string", default = "add" },
        { id = "formula", name = "公式(可选)", type = "string", default = "" }
    },

    inputs = {
        { id = "a", name = "A", type = "number", default = 0 },
        { id = "b", name = "B", type = "number", default = 0 },
        { id = "c", name = "C", type = "number", default = 0 },
        { id = "d", name = "D", type = "number", default = 0 }
    },

    outputs = {
        { id = "result", name = "结果", type = "number", default = 0 }
    },

    execute = function(self, inputs)
        local a = inputs.a or 0
        local b = inputs.b or 0
        local c = inputs.c or 0
        local d = inputs.d or 0
        local op = self.properties.operation or "add"
        local formula = self.properties.formula or ""

        local result = 0

        -- 如果有自定义公式，使用公式计算
        if formula ~= "" then
            -- 替换变量
            local expr = formula:gsub("A", tostring(a))
                                :gsub("B", tostring(b))
                                :gsub("C", tostring(c))
                                :gsub("D", tostring(d))
            -- 安全执行
            local fn = load("return " .. expr)
            if fn then
                local ok, val = pcall(fn)
                if ok then result = val end
            end
        else
            -- 预定义运算
            if op == "add" then
                result = a + b
            elseif op == "sub" then
                result = a - b
            elseif op == "mul" then
                result = a * b
            elseif op == "div" then
                result = b ~= 0 and a / b or 0
            elseif op == "mod" then
                result = b ~= 0 and a % b or 0
            elseif op == "pow" then
                result = a ^ b
            elseif op == "sqrt" then
                result = math.sqrt(math.abs(a))
            elseif op == "min" then
                result = math.min(a, b)
            elseif op == "max" then
                result = math.max(a, b)
            elseif op == "abs" then
                result = math.abs(a)
            elseif op == "floor" then
                result = math.floor(a)
            elseif op == "ceil" then
                result = math.ceil(a)
            elseif op == "round" then
                result = math.floor(a + 0.5)
            end
        end

        return { result = result }
    end
}

