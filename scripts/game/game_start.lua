-- 游戏开始Block
-- 触发游戏开始事件

return {
    meta = {
        id = "game.game_start",
        name = "游戏开始",
        category = "游戏",
        color = "#E91E63",
        description = "游戏开始时触发事件"
    },

    properties = {
        {
            id = "auto_start",
            name = "自动开始",
            type = "boolean",
            default = true
        }
    },

    inputs = {},

    outputs = {
        {
            id = "trigger",
            name = "触发",
            type = "event",
            default = nil
        }
    },

    execute = function(self, inputs)
        local properties = self.properties

        -- 每次执行都触发（用于循环打怪）
        if properties.auto_start then
            return { trigger = true }
        end

        return { trigger = nil }
    end
}

