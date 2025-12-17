-- 密码输入Block
-- 提供密码输入功能，显示为掩码

return {
    meta = {
        id = "input.password",
        name = "密码输入",
        category = "输入",
        color = "#FF5722",
        description = "用户可输入密码的交互Block（显示为掩码）",
        widget = "password",
        placeholder = "请输入密码..."
    },

    properties = {
        { id = "min_length", name = "最小长度", type = "number", default = 6, min = 0 }
    },

    inputs = {},

    outputs = {
        { id = "value", name = "密码值", type = "string", default = "" },
        { id = "length", name = "密码长度", type = "number", default = 0 },
        { id = "is_valid", name = "有效", type = "boolean", default = false }
    },

    execute = function(self, inputs)
        local password = self.state.widget_text or ""
        local min_len = self.properties.min_length or 6
        
        return {
            value = password,
            length = string.len(password),
            is_valid = string.len(password) >= min_len
        }
    end
}

