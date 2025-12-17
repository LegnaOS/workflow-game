-- 逻辑运算Block
-- 通用逻辑运算，支持条件判断和布尔运算

return {
    meta = {
        id = "util.logic",
        name = "逻辑运算",
        category = "工具",
        description = "通用逻辑运算，支持比较和布尔运算",
        color = "#9C27B0"
    },

    properties = {
        { id = "operation", name = "运算", type = "string", default = "eq" }
    },

    inputs = {
        { id = "a", name = "A", type = "any", default = 0 },
        { id = "b", name = "B", type = "any", default = 0 }
    },

    outputs = {
        { id = "result", name = "结果", type = "boolean", default = false },
        { id = "not_result", name = "取反", type = "boolean", default = true }
    },

    execute = function(self, inputs)
        local a = inputs.a
        local b = inputs.b
        local op = self.properties.operation or "eq"

        local result = false

        -- 比较运算
        if op == "eq" then
            result = a == b
        elseif op == "ne" then
            result = a ~= b
        elseif op == "gt" then
            result = (tonumber(a) or 0) > (tonumber(b) or 0)
        elseif op == "ge" then
            result = (tonumber(a) or 0) >= (tonumber(b) or 0)
        elseif op == "lt" then
            result = (tonumber(a) or 0) < (tonumber(b) or 0)
        elseif op == "le" then
            result = (tonumber(a) or 0) <= (tonumber(b) or 0)
        -- 布尔运算
        elseif op == "and" then
            result = a and b
        elseif op == "or" then
            result = a or b
        elseif op == "not" then
            result = not a
        elseif op == "xor" then
            result = (a and not b) or (not a and b)
        -- 其他
        elseif op == "is_nil" then
            result = a == nil
        elseif op == "is_number" then
            result = type(a) == "number"
        elseif op == "is_string" then
            result = type(a) == "string"
        end

        return {
            result = result and true or false,
            not_result = not result
        }
    end
}

