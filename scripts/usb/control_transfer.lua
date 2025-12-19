-- USB Control 传输 Block
-- 发送 USB 控制传输请求

return {
    meta = {
        id = "usb.control_transfer",
        name = "USB Control 传输",
        category = "USB",
        description = "发送 USB 控制传输请求（Vendor/Class/Standard）",
        color = "#9C27B0"
    },

    properties = {
        { id = "vid", name = "VID (十六进制)", type = "string", default = "0000" },
        { id = "pid", name = "PID (十六进制)", type = "string", default = "0000" },
        { id = "direction", name = "方向 (in/out)", type = "string", default = "in" },
        { id = "req_type", name = "类型 (vendor/class/standard)", type = "string", default = "vendor" },
        { id = "recipient", name = "接收者 (device/interface/endpoint)", type = "string", default = "device" },
        { id = "request", name = "请求码 (bRequest)", type = "number", default = 0, min = 0, max = 255 },
        { id = "value", name = "wValue", type = "number", default = 0, min = 0, max = 65535 },
        { id = "index", name = "wIndex", type = "number", default = 0, min = 0, max = 65535 },
        { id = "size", name = "数据大小", type = "number", default = 64, min = 0, max = 4096 },
        { id = "timeout", name = "超时(ms)", type = "number", default = 1000, min = 100, max = 30000 }
    },

    inputs = {
        { id = "trigger", name = "触发", type = "event" },
        { id = "data", name = "发送数据 (OUT)", type = "string" }
    },

    outputs = {
        { id = "response", name = "响应数据", type = "string" },
        { id = "length", name = "长度", type = "number" },
        { id = "success", name = "成功", type = "boolean" },
        { id = "error", name = "错误信息", type = "string" }
    },

    execute = function(self, inputs)
        -- 防御性检查
        local props = (self and self.properties) or {}

        local vid = tonumber(props.vid, 16) or 0
        local pid = tonumber(props.pid, 16) or 0

        -- 打开设备
        local ok, device = pcall(usb.open, vid, pid)
        if not ok then
            return {
                response = "",
                length = 0,
                success = false,
                error = "无法打开设备: " .. tostring(device)
            }
        end

        -- 构建 request_type
        local direction = props.direction or "in"
        local req_type = props.req_type or "vendor"
        local recipient = props.recipient or "device"

        local rt_ok, rt = pcall(usb.request_type, direction, req_type, recipient)
        if not rt_ok then
            pcall(function() device:close() end)
            return {
                response = "",
                length = 0,
                success = false,
                error = "无效的 request_type 参数: " .. tostring(rt)
            }
        end

        local out_response = ""
        local out_length = 0
        local out_success = false
        local out_error = ""

        -- 执行传输
        if direction == "in" then
            local ok, result = pcall(function()
                return device:read_control({
                    request_type = rt,
                    request = props.request or 0,
                    value = props.value or 0,
                    index = props.index or 0,
                    size = props.size or 64,
                    timeout = props.timeout or 1000
                })
            end)

            if ok then
                out_response = result.data
                out_length = result.length
                out_success = true
            else
                out_error = tostring(result)
            end
        else
            local data = inputs.data or ""
            local ok, n = pcall(function()
                return device:write_control({
                    request_type = rt,
                    request = props.request or 0,
                    value = props.value or 0,
                    index = props.index or 0,
                    data = data,
                    timeout = props.timeout or 1000
                })
            end)

            if ok then
                out_length = n
                out_success = true
            else
                out_error = tostring(n)
            end
        end

        -- 关闭设备
        pcall(function() device:close() end)

        return {
            response = out_response,
            length = out_length,
            success = out_success,
            error = out_error
        }
    end
}

