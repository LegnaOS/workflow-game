-- 金币收集器Block
-- 收集并累计金币

return {
    meta = {
        id = "game.gold_collector",
        name = "金币收集",
        category = "游戏",
        description = "收集战斗掉落的金币并累计",
        color = "#FFC107"
    },

    properties = {
        { id = "bonus_percent", name = "金币加成%", type = "number", default = 0, min = 0, max = 500 }
    },

    inputs = {
        { id = "gold_in", name = "获得金币", type = "number", default = 0 },
        { id = "collect_event", name = "收集事件", type = "event" }
    },

    outputs = {
        { id = "total_gold", name = "总金币", type = "number" },
        { id = "last_collect", name = "本次获得", type = "number" },
        { id = "collect_count", name = "收集次数", type = "number" }
    },

    execute = function(self, inputs)
        local props = self.properties
        local state = self.state or {}

        local total = state.total_gold or 0
        local count = state.collect_count or 0
        local last = 0

        local gold_in = inputs.gold_in or 0
        local event = inputs.collect_event

        if event and gold_in > 0 then
            -- 应用加成
            local bonus = 1 + (props.bonus_percent or 0) / 100
            last = math.floor(gold_in * bonus)
            total = total + last
            count = count + 1
            print(string.format("[金币] +%d (总计: %d)", last, total))
        end

        state.total_gold = total
        state.collect_count = count
        self.state = state

        return {
            total_gold = total,
            last_collect = last,
            collect_count = count
        }
    end
}

