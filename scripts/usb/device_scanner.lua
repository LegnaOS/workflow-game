-- USB 设备扫描器 Block
-- 扫描并列出所有连接的 USB 设备
-- 使用动态端口为每个设备生成独立的 VID/PID 输出

return {
    meta = {
        id = "usb.device_scanner",
        name = "USB 设备扫描",
        category = "USB",
        description = "扫描所有 USB 设备，动态生成每个设备的 VID/PID 输出端口",
        color = "#9C27B0"
    },

    properties = {
        { id = "vid_filter", name = "VID 过滤 (十六进制)", type = "string", default = "" },
        { id = "pid_filter", name = "PID 过滤 (十六进制)", type = "string", default = "" }
    },

    inputs = {
        { id = "trigger", name = "触发", type = "event" }
    },

    -- 静态输出端口（设备数量）
    outputs = {
        { id = "count", name = "设备数量", type = "number" }
    },

    execute = function(self, inputs)
        local props = (self and self.properties) or {}
        local all_devices = usb.devices()
        local filtered = {}

        -- 解析过滤器
        local vid_filter = nil
        local pid_filter = nil

        if props.vid_filter and props.vid_filter ~= "" then
            vid_filter = tonumber(props.vid_filter, 16)
        end
        if props.pid_filter and props.pid_filter ~= "" then
            pid_filter = tonumber(props.pid_filter, 16)
        end

        -- 过滤设备
        for _, dev in ipairs(all_devices) do
            local match = true
            if vid_filter and dev.vendor_id ~= vid_filter then
                match = false
            end
            if pid_filter and dev.product_id ~= pid_filter then
                match = false
            end
            if match then
                table.insert(filtered, {
                    vid = dev.vendor_id or 0,
                    pid = dev.product_id or 0,
                    name = dev.product or dev.manufacturer or "Unknown"
                })
            end
        end

        -- 构建动态输出
        local result = {
            count = #filtered
        }

        -- 为每个设备生成动态端口
        for i, dev in ipairs(filtered) do
            local prefix = "dev" .. i .. "_"
            result[prefix .. "vid"] = dev.vid
            result[prefix .. "pid"] = dev.pid
            result[prefix .. "name"] = dev.name
        end

        return result
    end
}

