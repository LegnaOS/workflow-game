-- 回合控制器Block
-- 控制回合制游戏的节奏

return {
    meta = {
        id = "game.turn_controller",
        name = "回合控制",
        category = "游戏",
        description = "自动触发回合，控制游戏节奏",
        color = "#607D8B"
    },

    properties = {
        { id = "auto_turn", name = "自动回合", type = "boolean", default = true },
        { id = "turn_interval", name = "回合间隔(tick)", type = "number", default = 30, min = 1 }
    },

    inputs = {
        { id = "manual_trigger", name = "手动触发", type = "event" },
        { id = "pause", name = "暂停", type = "boolean", default = false }
    },

    outputs = {
        { id = "turn_event", name = "回合事件", type = "event" },
        { id = "turn_count", name = "回合数", type = "number" },
        { id = "is_running", name = "运行中", type = "boolean" }
    },

    execute = function(self, inputs)
        local props = self.properties
        local state = self.state or {}

        local tick = (state.tick or 0) + 1
        local turn_count = state.turn_count or 0
        local interval = props.turn_interval or 30
        local paused = inputs.pause or false

        local turn_event = nil
        local is_running = not paused

        -- 手动触发
        if inputs.manual_trigger then
            turn_event = true
            turn_count = turn_count + 1
            print("[回合] 第 " .. turn_count .. " 回合 (手动)")
        -- 自动触发
        elseif props.auto_turn and not paused and tick % interval == 0 then
            turn_event = true
            turn_count = turn_count + 1
            print("[回合] 第 " .. turn_count .. " 回合")
        end

        state.tick = tick
        state.turn_count = turn_count
        self.state = state

        return {
            turn_event = turn_event,
            turn_count = turn_count,
            is_running = is_running
        }
    end
}

