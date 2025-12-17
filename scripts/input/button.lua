-- 按钮Block
-- 提供用户点击交互

return {
    meta = {
        id = "input.button",
        name = "按钮",
        category = "输入",
        color = "#4CAF50",
        description = "可点击的按钮，触发事件",
        widget = "button",
        placeholder = "点击"
    },

    properties = {
        { id = "button_text", name = "按钮文字", type = "string", default = "点击" }
    },

    inputs = {},

    outputs = {
        { id = "clicked", name = "点击事件", type = "event", default = nil },
        { id = "click_count", name = "点击次数", type = "number", default = 0 }
    },

    execute = function(self, inputs)
        local state = self.state or {}
        local count = state.click_count or 0
        local was_checked = state.last_checked or false
        local is_checked = self.state.widget_checked or false
        
        -- 检测点击（从未选中变为选中）
        local clicked = is_checked and not was_checked
        if clicked then
            count = count + 1
        end
        
        state.click_count = count
        state.last_checked = is_checked
        self.state = state
        
        return {
            clicked = clicked and true or nil,
            click_count = count
        }
    end
}

