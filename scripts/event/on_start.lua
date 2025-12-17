-- 游戏开始事件
return {
    meta = {
        id = "event.on_start",
        name = "游戏开始",
        category = "事件",
        description = "游戏开始时触发",
        color = "#E91E63"
    },
    
    inputs = {},
    
    outputs = {
        { id = "trigger", name = "触发", type = "event" }
    },
    
    properties = {},
    
    execute = function(self, inputs)
        return { trigger = true }
    end
}

