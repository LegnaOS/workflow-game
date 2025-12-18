-- 常量节点
return {
    meta = {
        id = "math.constant",
        name = "常量",
        category = "数学",
        description = "输出一个固定的数值",
        color = "#2196F3",
        hideable = true
    },
    
    inputs = {},
    
    outputs = {
        { id = "value", name = "值", type = "number" }
    },
    
    properties = {
        { id = "value", name = "数值", type = "number", default = 0 }
    },
    
    execute = function(self, inputs)
        return { value = self.properties.value or 0 }
    end
}

