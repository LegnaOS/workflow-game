-- 文本输入Block
-- 提供用户文本输入功能

return {
    meta = {
        id = "input.text_input",
        name = "文本输入",
        category = "输入",
        color = "#2196F3",
        description = "用户可输入文本的交互Block",
        widget = "textinput",
        placeholder = "请输入文本..."
    },

    properties = {
        { id = "label", name = "标签", type = "string", default = "输入" }
    },

    inputs = {},

    outputs = {
        { id = "value", name = "文本值", type = "string", default = "" },
        { id = "length", name = "文本长度", type = "number", default = 0 }
    },

    execute = function(self, inputs)
        -- widget_text 会自动从Block的widget状态同步
        local text = self.state.widget_text or ""
        
        return {
            value = text,
            length = string.len(text)
        }
    end
}

