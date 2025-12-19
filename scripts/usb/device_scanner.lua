-- USB 设备扫描器 Block
-- 扫描并列出所有连接的 USB 设备

return {
    meta = {
        id = "usb.device_scanner",
        name = "USB 设备扫描",
        category = "USB",
        description = "扫描并列出所有 USB 设备，可按 VID/PID 过滤",
        color = "#9C27B0"
    },

    properties = {
        { id = "vid_filter", name = "VID 过滤 (十六进制)", type = "string", default = "" },
        { id = "pid_filter", name = "PID 过滤 (十六进制)", type = "string", default = "" },
        { id = "auto_scan", name = "自动扫描", type = "boolean", default = false }
    },

    inputs = {
        { id = "trigger", name = "触发", type = "event" }
    },

    outputs = {
        { id = "devices", name = "设备列表", type = "table" },
        { id = "count", name = "设备数量", type = "number" },
        { id = "first", name = "第一个设备", type = "table" }
    },

    execute = function(inputs, outputs, props, state)
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
                    bus = dev.bus_number,
                    address = dev.address,
                    vid = dev.vendor_id,
                    pid = dev.product_id,
                    vid_hex = string.format("%04X", dev.vendor_id or 0),
                    pid_hex = string.format("%04X", dev.product_id or 0),
                    manufacturer = dev.manufacturer,
                    product = dev.product,
                    serial = dev.serial_number,
                    speed = dev.speed,
                    class = dev.class_code
                })
            end
        end
        
        outputs.devices = filtered
        outputs.count = #filtered
        outputs.first = filtered[1] or {}
    end
}

