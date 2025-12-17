-- 分支节点
return {
    meta = {
        id = "logic.branch",
        name = "分支",
        category = "逻辑",
        description = "根据条件选择输出",
        color = "#FF9800"
    },
    
    inputs = {
        { id = "condition", name = "条件", type = "boolean", default = false },
        { id = "true_value", name = "真值", type = "any" },
        { id = "false_value", name = "假值", type = "any" }
    },
    
    outputs = {
        { id = "result", name = "结果", type = "any" }
    },
    
    properties = {},
    
    execute = function(self, inputs)
        if inputs.condition then
            return { result = inputs.true_value }
        else
            return { result = inputs.false_value }
        end
    end
}

