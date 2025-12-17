-- 比较节点
return {
    meta = {
        id = "logic.compare",
        name = "比较",
        category = "逻辑",
        description = "比较两个数值",
        color = "#FF9800"
    },
    
    inputs = {
        { id = "a", name = "A", type = "number", default = 0 },
        { id = "b", name = "B", type = "number", default = 0 }
    },
    
    outputs = {
        { id = "equal", name = "相等", type = "boolean" },
        { id = "greater", name = "A>B", type = "boolean" },
        { id = "less", name = "A<B", type = "boolean" }
    },
    
    properties = {},
    
    execute = function(self, inputs)
        local a = inputs.a or 0
        local b = inputs.b or 0
        return {
            equal = a == b,
            greater = a > b,
            less = a < b
        }
    end
}

