-- 打印节点
return {
    meta = {
        id = "event.print",
        name = "打印",
        category = "事件",
        description = "打印消息到控制台",
        color = "#9C27B0"
    },
    
    inputs = {
        { id = "message", name = "消息", type = "any" }
    },
    
    outputs = {},
    
    properties = {
        { id = "prefix", name = "前缀", type = "string", default = "[LOG]" }
    },
    
    execute = function(self, inputs)
        local prefix = self.properties.prefix or "[LOG]"
        local message = inputs.message or ""
        print(prefix .. " " .. tostring(message))
        return {}
    end
}

