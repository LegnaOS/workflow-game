-- USB Bulk 传输 Block
-- 从 USB 设备读取或写入 Bulk 数据

return {
    meta = {
        id = "usb.bulk_transfer",
        name = "USB Bulk 传输",
        category = "USB",
        description = "USB Bulk 读取/写入操作",
        color = "#9C27B0"
    },

    properties = {
        { id = "vid", name = "VID (十六进制)", type = "string", default = "0000" },
        { id = "pid", name = "PID (十六进制)", type = "string", default = "0000" },
        { id = "interface", name = "接口号", type = "number", default = 0, min = 0, max = 15 },
        { id = "endpoint", name = "端点地址", type = "number", default = 129, min = 0, max = 255 },
        { id = "direction", name = "方向 (read/write)", type = "string", default = "read" },
        { id = "size", name = "读取大小", type = "number", default = 64, min = 1, max = 4096 },
        { id = "timeout", name = "超时(ms)", type = "number", default = 1000, min = 100, max = 30000 }
    },

    inputs = {
        { id = "trigger", name = "触发", type = "event" },
        { id = "data", name = "发送数据", type = "string" }
    },

    outputs = {
        { id = "data", name = "数据", type = "string" },
        { id = "length", name = "长度", type = "number" },
        { id = "success", name = "成功", type = "boolean" },
        { id = "error", name = "错误信息", type = "string" }
    },

    execute = function(self, inputs)
        -- 防御性检查
        local props = (self and self.properties) or {}
        local state = (self and self.state) or {}

        local vid = tonumber(props.vid, 16) or 0
        local pid = tonumber(props.pid, 16) or 0

        -- 默认输出
        local out_data = ""
        local out_length = 0
        local out_success = false
        local out_error = ""

        -- 尝试复用已打开的设备
        if not state.device or state.vid ~= vid or state.pid ~= pid then
            -- 关闭旧设备
            if state.device then
                pcall(function() state.device:close() end)
                state.device = nil
            end

            -- 打开新设备
            local ok, dev = pcall(usb.open, vid, pid)
            if not ok then
                return {
                    data = "",
                    length = 0,
                    success = false,
                    error = "无法打开设备: " .. tostring(dev)
                }
            end

            state.device = dev
            state.vid = vid
            state.pid = pid
            state.claimed = {}

            -- 自动分离内核驱动
            pcall(function() dev:set_auto_detach_kernel_driver(true) end)
        end

        -- 确保接口已声明
        local iface = props.interface or 0
        if not state.claimed then state.claimed = {} end
        if not state.claimed[iface] then
            local ok, err = pcall(function()
                state.device:claim_interface(iface)
            end)
            if not ok then
                return {
                    data = "",
                    length = 0,
                    success = false,
                    error = "无法声明接口: " .. tostring(err)
                }
            end
            state.claimed[iface] = true
        end

        -- 执行传输
        local direction = props.direction or "read"
        if direction == "read" or direction == "in" then
            local ok, result = pcall(function()
                return state.device:read_bulk(props.endpoint or 129, props.size or 64, props.timeout or 1000)
            end)

            if ok then
                out_data = result.data
                out_length = result.length
                out_success = true
            else
                out_error = tostring(result)
            end
        else
            local data = inputs.data or ""
            local ok, n = pcall(function()
                return state.device:write_bulk(props.endpoint or 1, data, props.timeout or 1000)
            end)

            if ok then
                out_length = n
                out_success = true
            else
                out_error = tostring(n)
            end
        end

        return {
            data = out_data,
            length = out_length,
            success = out_success,
            error = out_error
        }
    end
}

