-- 数据分发Block
-- 将一个输入分发到多个输出

return {
    meta = {
        id = "util.splitter",
        name = "数据分发",
        category = "工具",
        description = "将一个输入复制到多个输出，或从表中提取字段",
        color = "#9C27B0"
    },

    properties = {
        { id = "key1", name = "提取键1", type = "string", default = "" },
        { id = "key2", name = "提取键2", type = "string", default = "" },
        { id = "key3", name = "提取键3", type = "string", default = "" },
        { id = "key4", name = "提取键4", type = "string", default = "" }
    },

    inputs = {
        { id = "input", name = "输入", type = "any" }
    },

    outputs = {
        { id = "out1", name = "输出1", type = "any" },
        { id = "out2", name = "输出2", type = "any" },
        { id = "out3", name = "输出3", type = "any" },
        { id = "out4", name = "输出4", type = "any" },
        { id = "passthrough", name = "透传", type = "any" }
    },

    execute = function(self, inputs)
        local data = inputs.input
        local props = self.properties
        local keys = { props.key1, props.key2, props.key3, props.key4 }

        local outputs = { out1 = nil, out2 = nil, out3 = nil, out4 = nil, passthrough = data }

        if type(data) == "table" then
            -- 从表中提取字段
            for i, key in ipairs(keys) do
                if key and key ~= "" then
                    outputs["out" .. i] = data[key]
                else
                    outputs["out" .. i] = data
                end
            end
        else
            -- 简单复制到所有输出
            for i = 1, 4 do
                outputs["out" .. i] = data
            end
        end

        return outputs
    end
}

