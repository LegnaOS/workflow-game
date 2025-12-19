-- USB 设备信息 Block
-- 根据 VID/PID 打开设备并获取详细信息

return {
    meta = {
        id = "usb.device_info",
        name = "USB 设备信息",
        category = "USB",
        description = "根据 VID/PID 获取 USB 设备详细信息",
        color = "#9C27B0"
    },

    properties = {
        { id = "vid_prop", name = "VID (十六进制)", type = "string", default = "" },
        { id = "pid_prop", name = "PID (十六进制)", type = "string", default = "" }
    },

    inputs = {
        { id = "vid", name = "VID", type = "number" },
        { id = "pid", name = "PID", type = "number" },
        { id = "trigger", name = "触发", type = "event" }
    },

    outputs = {
        { id = "vid_hex", name = "VID", type = "string" },
        { id = "pid_hex", name = "PID", type = "string" },
        { id = "manufacturer", name = "制造商", type = "string" },
        { id = "product", name = "产品名", type = "string" },
        { id = "serial", name = "序列号", type = "string" },
        { id = "success", name = "成功", type = "boolean" },
        { id = "error", name = "错误信息", type = "string" }
    },

    execute = function(self, inputs)
        local props = (self and self.properties) or {}

        -- 优先使用连接输入，否则用属性
        local vid = inputs.vid
        local pid = inputs.pid

        if (not vid or vid == 0) and props.vid_prop and props.vid_prop ~= "" then
            vid = tonumber(props.vid_prop, 16)
        end
        if (not pid or pid == 0) and props.pid_prop and props.pid_prop ~= "" then
            pid = tonumber(props.pid_prop, 16)
        end

        if not vid or not pid or vid == 0 or pid == 0 then
            return {
                vid_hex = "", pid_hex = "", manufacturer = "", product = "",
                serial = "", success = false,
                error = "未指定设备（请连接 VID/PID 输入或在属性中设置）"
            }
        end

        -- 尝试打开设备
        local ok, device = pcall(usb.open, vid, pid)
        if not ok then
            return {
                vid_hex = string.format("%04X", vid),
                pid_hex = string.format("%04X", pid),
                manufacturer = "", product = "", serial = "",
                success = false,
                error = "无法打开设备: " .. tostring(device)
            }
        end

        local out_manufacturer = ""
        local out_product = ""
        local out_serial = ""

        -- 尝试读取字符串描述符
        local ok, mfr = pcall(function() return device:manufacturer() end)
        if ok and mfr then out_manufacturer = mfr end

        local ok, prod = pcall(function() return device:product() end)
        if ok and prod then out_product = prod end

        local ok, serial = pcall(function() return device:serial_number() end)
        if ok and serial then out_serial = serial end

        pcall(function() device:close() end)

        return {
            vid_hex = string.format("%04X", vid),
            pid_hex = string.format("%04X", pid),
            manufacturer = out_manufacturer,
            product = out_product,
            serial = out_serial,
            success = true,
            error = ""
        }
    end
}

