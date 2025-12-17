-- 金币收集器Block
-- 收集掉落的金币

return {
    meta = {
        id = "lite.gold",
        name = "金币袋",
        category = "Lite",
        color = "#FFC107",
        description = "收集掉落的金币"
    },

    properties = {
        { id = "initial", name = "初始金币", type = "number", default = 0, min = 0 }
    },

    inputs = {
        { id = "gold_in", name = "金币输入", type = "number", default = 0 },
        { id = "collect_event", name = "收集事件", type = "event", default = nil }
    },

    outputs = {
        { id = "total", name = "总金币", type = "number", default = 0 }
    },

    execute = function(self, inputs)
        local state = self.state or {}
        local total = state.total or self.properties.initial
        
        if inputs.collect_event and inputs.gold_in > 0 then
            total = total + inputs.gold_in
            print("[金币] +" .. inputs.gold_in .. " 总计: " .. total)
        end
        
        state.total = total
        self.state = state
        
        return { total = total }
    end
}

