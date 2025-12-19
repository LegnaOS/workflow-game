-- USB 设备信息 Block
-- 获取 USB 设备的详细描述符信息

return {
    meta = {
        id = "usb.device_info",
        name = "USB 设备信息",
        category = "USB",
        description = "获取 USB 设备的详细描述符信息",
        color = "#9C27B0"
    },

    properties = {
        { id = "vid", name = "VID (十六进制)", type = "string", default = "0000" },
        { id = "pid", name = "PID (十六进制)", type = "string", default = "0000" }
    },

    inputs = {
        { id = "trigger", name = "触发", type = "event" }
    },

    outputs = {
        { id = "descriptor", name = "设备描述符", type = "table" },
        { id = "config", name = "配置描述符", type = "table" },
        { id = "manufacturer", name = "制造商", type = "string" },
        { id = "product", name = "产品名", type = "string" },
        { id = "serial", name = "序列号", type = "string" },
        { id = "success", name = "成功", type = "boolean" },
        { id = "error", name = "错误信息", type = "string" }
    },

    execute = function(self, inputs)
        local props = self.properties or {}

        local vid = tonumber(props.vid, 16) or 0
        local pid = tonumber(props.pid, 16) or 0

        -- 打开设备
        local ok, device = pcall(usb.open, vid, pid)
        if not ok then
            return {
                descriptor = {},
                config = {},
                manufacturer = "",
                product = "",
                serial = "",
                success = false,
                error = "无法打开设备: " .. tostring(device)
            }
        end

        local out_descriptor = {}
        local out_config = {}
        local out_manufacturer = ""
        local out_product = ""
        local out_serial = ""

        -- 获取设备描述符
        local ok, desc = pcall(function() return device:descriptor() end)
        if ok then
            out_descriptor = desc
        end

        -- 获取配置描述符
        local ok, config = pcall(function() return device:config() end)
        if ok then
            out_config = config
        end

        -- 获取字符串描述符
        local ok, mfr = pcall(function() return device:manufacturer() end)
        if ok and mfr then out_manufacturer = mfr end

        local ok, prod = pcall(function() return device:product() end)
        if ok and prod then out_product = prod end

        local ok, serial = pcall(function() return device:serial_number() end)
        if ok and serial then out_serial = serial end

        -- 关闭设备
        pcall(function() device:close() end)

        return {
            descriptor = out_descriptor,
            config = out_config,
            manufacturer = out_manufacturer,
            product = out_product,
            serial = out_serial,
            success = true,
            error = ""
        }
    end
}

