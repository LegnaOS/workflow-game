-- 数据合并Block
-- 将多个输入合并为一个表/对象输出

return {
    meta = {
        id = "util.merger",
        name = "数据合并",
        category = "工具",
        description = "将多个输入合并为JSON对象输出",
        color = "#9C27B0"
    },

    properties = {
        { id = "key1", name = "键名1", type = "string", default = "a" },
        { id = "key2", name = "键名2", type = "string", default = "b" },
        { id = "key3", name = "键名3", type = "string", default = "c" },
        { id = "key4", name = "键名4", type = "string", default = "d" }
    },

    inputs = {
        { id = "in1", name = "输入1", type = "any" },
        { id = "in2", name = "输入2", type = "any" },
        { id = "in3", name = "输入3", type = "any" },
        { id = "in4", name = "输入4", type = "any" }
    },

    outputs = {
        { id = "merged", name = "合并结果", type = "any" },
        { id = "count", name = "项目数", type = "number" },
        { id = "json", name = "JSON", type = "string" }
    },

    execute = function(self, inputs)
        local props = self.properties
        local result = {}
        local count = 0

        local keys = {
            props.key1 or "a",
            props.key2 or "b",
            props.key3 or "c",
            props.key4 or "d"
        }
        local vals = { inputs.in1, inputs.in2, inputs.in3, inputs.in4 }

        for i, v in ipairs(vals) do
            if v ~= nil then
                result[keys[i]] = v
                count = count + 1
            end
        end

        -- 简单JSON序列化
        local function to_json(t)
            local parts = {}
            for k, v in pairs(t) do
                local val_str
                if type(v) == "string" then
                    val_str = '"' .. v .. '"'
                elseif type(v) == "boolean" then
                    val_str = v and "true" or "false"
                elseif type(v) == "table" then
                    val_str = to_json(v)
                else
                    val_str = tostring(v)
                end
                table.insert(parts, '"' .. k .. '":' .. val_str)
            end
            return "{" .. table.concat(parts, ",") .. "}"
        end

        return {
            merged = result,
            count = count,
            json = to_json(result)
        }
    end
}

