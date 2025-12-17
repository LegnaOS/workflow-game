-- 日志Block
-- 支持多输入，自动JSON格式化输出

return {
    meta = {
        id = "debug.logger",
        name = "日志",
        category = "调试",
        description = "支持多个数据输入，自动JSON格式化输出",
        color = "#607D8B"
    },

    properties = {
        { id = "level", name = "日志级别", type = "string", default = "INFO" },
        { id = "tag", name = "标签", type = "string", default = "Game" }
    },

    inputs = {
        { id = "in1", name = "输入1", type = "any" },
        { id = "in2", name = "输入2", type = "any" },
        { id = "in3", name = "输入3", type = "any" },
        { id = "in4", name = "输入4", type = "any" },
        { id = "trigger", name = "触发", type = "event" }
    },

    outputs = {
        { id = "log_text", name = "日志文本", type = "string" },
        { id = "json_out", name = "JSON", type = "string" }
    },

    execute = function(self, inputs)
        local props = self.properties
        local level = props.level or "INFO"
        local tag = props.tag or "Game"

        -- 收集所有非nil输入
        local data = {}
        local has_data = false
        for i = 1, 4 do
            local key = "in" .. i
            if inputs[key] ~= nil then
                data["input" .. i] = inputs[key]
                has_data = true
            end
        end

        -- 只在有数据或触发时输出
        if not has_data and not inputs.trigger then
            return { log_text = "", json_out = "{}" }
        end

        -- JSON格式化
        local function to_json(v, indent)
            indent = indent or 0
            local spaces = string.rep("  ", indent)
            local t = type(v)
            if t == "nil" then
                return "null"
            elseif t == "boolean" then
                return v and "true" or "false"
            elseif t == "number" then
                return tostring(v)
            elseif t == "string" then
                return '"' .. v:gsub('"', '\\"') .. '"'
            elseif t == "table" then
                local parts = {}
                local is_array = #v > 0
                for k, val in pairs(v) do
                    if is_array then
                        table.insert(parts, spaces .. "  " .. to_json(val, indent + 1))
                    else
                        table.insert(parts, spaces .. '  "' .. tostring(k) .. '": ' .. to_json(val, indent + 1))
                    end
                end
                if is_array then
                    return "[\n" .. table.concat(parts, ",\n") .. "\n" .. spaces .. "]"
                else
                    return "{\n" .. table.concat(parts, ",\n") .. "\n" .. spaces .. "}"
                end
            else
                return '"' .. tostring(v) .. '"'
            end
        end

        local json_str = to_json(data)
        local timestamp = os.date("%H:%M:%S")
        local log_text = string.format("[%s] [%s] [%s] %s", timestamp, level, tag, json_str)

        print(log_text)

        return { log_text = log_text, json_out = json_str }
    end
}

