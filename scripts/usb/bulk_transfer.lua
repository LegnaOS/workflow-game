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
        { id = "direction", name = "方向", type = "string", default = "read" },
        { id = "size", name = "读取大小", type = "number", default = 64, min = 1, max = 4096 },
        { id = "timeout", name = "超时(ms)", type = "number", default = 1000, min = 100, max = 30000 }
    },

    inputs = {
        { id = "trigger", name = "触发", type = "event" },
        { id = "data", name = "发送数据", type = "string", optional = true }
    },

    outputs = {
        { id = "data", name = "数据", type = "string" },
        { id = "length", name = "长度", type = "number" },
        { id = "success", name = "成功", type = "boolean" },
        { id = "error", name = "错误信息", type = "string" }
    },

    execute = function(inputs, outputs, props, state)
        local vid = tonumber(props.vid, 16) or 0
        local pid = tonumber(props.pid, 16) or 0
        
        -- 初始化输出
        outputs.data = ""
        outputs.length = 0
        outputs.success = false
        outputs.error = ""
        
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
                outputs.error = "无法打开设备: " .. tostring(dev)
                return
            end
            
            state.device = dev
            state.vid = vid
            state.pid = pid
            state.claimed = {}
            
            -- 自动分离内核驱动
            pcall(function() dev:set_auto_detach_kernel_driver(true) end)
        end
        
        -- 确保接口已声明
        if not state.claimed[props.interface] then
            local ok, err = pcall(function()
                state.device:claim_interface(props.interface)
            end)
            if not ok then
                outputs.error = "无法声明接口: " .. tostring(err)
                return
            end
            state.claimed[props.interface] = true
        end
        
        -- 执行传输
        if props.direction == "read" or props.direction == "in" then
            local ok, result = pcall(function()
                return state.device:read_bulk(props.endpoint, props.size, props.timeout)
            end)
            
            if ok then
                outputs.data = result.data
                outputs.length = result.length
                outputs.success = true
            else
                outputs.error = tostring(result)
            end
        else
            local data = inputs.data or ""
            local ok, n = pcall(function()
                return state.device:write_bulk(props.endpoint, data, props.timeout)
            end)
            
            if ok then
                outputs.length = n
                outputs.success = true
            else
                outputs.error = tostring(n)
            end
        end
    end
}

