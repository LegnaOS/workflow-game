-- 常量值Block
-- 输出常量值，用于构建工作流

return {
    meta = {
        id = "util.value",
        name = "常量",
        category = "工具",
        description = "输出一个常量值",
        color = "#795548"
    },

    properties = {
        { id = "number_val", name = "数字值", type = "number", default = 0 },
        { id = "string_val", name = "字符串值", type = "string", default = "" },
        { id = "bool_val", name = "布尔值", type = "boolean", default = false }
    },

    inputs = {},

    outputs = {
        { id = "number", name = "数字", type = "number", default = 0 },
        { id = "string", name = "字符串", type = "string", default = "" },
        { id = "bool", name = "布尔", type = "boolean", default = false }
    },

    execute = function(self, inputs)
        local props = self.properties
        return {
            number = props.number_val or 0,
            string = props.string_val or "",
            bool = props.bool_val or false
        }
    end
}

