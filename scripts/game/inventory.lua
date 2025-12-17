-- 背包Block
-- 存储玩家获得的物品和金币

return {
    meta = {
        id = "game.inventory",
        name = "背包",
        category = "游戏",
        color = "#FF9800",
        description = "玩家背包，存储金币和物品"
    },

    properties = {
        {
            id = "initial_gold",
            name = "初始金币",
            type = "number",
            default = 0,
            min = 0
        },
        {
            id = "capacity",
            name = "容量",
            type = "number",
            default = 20,
            min = 1,
            max = 100
        }
    },

    inputs = {
        {
            id = "gold_in",
            name = "获得金币",
            type = "number",
            default = 0
        },
        {
            id = "item_in",
            name = "获得物品",
            type = "string",
            default = ""
        }
    },

    outputs = {
        {
            id = "total_gold",
            name = "总金币",
            type = "number",
            default = 0
        },
        {
            id = "item_count",
            name = "物品数量",
            type = "number",
            default = 0
        },
        {
            id = "is_full",
            name = "是否已满",
            type = "boolean",
            default = false
        }
    },

    execute = function(self, inputs)
        local properties = self.properties
        local state = self.state or {}

        local initial_gold = properties.initial_gold or 0
        local capacity = properties.capacity or 20
        local gold_in = inputs.gold_in or 0

        -- 从state获取累计金币
        local total_gold = state.total_gold or initial_gold

        -- 累加获得的金币
        if gold_in > 0 then
            total_gold = total_gold + gold_in
            print("[背包] 获得 " .. gold_in .. " 金币, 总计: " .. total_gold)
        end

        -- 简化处理：每次输入一个物品
        local item_count = state.item_count or 0
        if inputs.item_in and inputs.item_in ~= "" then
            item_count = item_count + 1
        end
        local is_full = item_count >= capacity

        -- 保存状态
        self.state.total_gold = total_gold
        self.state.item_count = item_count

        return {
            total_gold = total_gold,
            item_count = item_count,
            is_full = is_full
        }
    end
}

