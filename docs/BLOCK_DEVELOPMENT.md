# Block 开发指南

> Block 是工作流引擎的基本执行单元，每个 Block 是一个 Lua 脚本。
> 放到 `scripts/` 目录即可，支持热重载。

---

## 目录

- [快速开始](#快速开始)
- [脚本结构](#脚本结构)
- [数据类型](#数据类型)
- [核心概念](#核心概念)
- [交互控件](#交互控件)
- [动画系统](#动画系统)
- [USB 开发](#usb-开发)
- [最佳实践](#最佳实践)

---

## 快速开始

**最小示例** - 创建 `scripts/my/double.lua`：

```lua
return {
    meta = {
        id = "my.double",
        name = "翻倍",
        category = "我的",
        color = "#FF5722"
    },
    inputs = {
        { id = "value", name = "输入", type = "number", default = 0 }
    },
    outputs = {
        { id = "result", name = "结果", type = "number" }
    },
    execute = function(self, inputs)
        return { result = inputs.value * 2 }
    end
}
```

保存后立即在 IDE 左侧「我的」分类中出现。

---

## 脚本结构

```lua
return {
    -- ═══════════════════════════════════════════════════════════
    -- 元数据 (必需)
    -- ═══════════════════════════════════════════════════════════
    meta = {
        id = "category.name",       -- 唯一标识符 (必需)
        name = "显示名称",           -- Block 标题
        category = "分类",           -- 左侧面板分类
        description = "悬停提示",    -- 鼠标悬停显示
        color = "#4CAF50",          -- 标题栏颜色
        hideable = false,           -- 预览模式可隐藏 (可选)
        widget = nil                -- 交互控件类型 (可选)
    },

    -- ═══════════════════════════════════════════════════════════
    -- 属性 (可选) - 右侧面板可编辑
    -- ═══════════════════════════════════════════════════════════
    properties = {
        { id = "damage", name = "伤害", type = "number", default = 10, min = 0, max = 999 },
        { id = "name", name = "名称", type = "string", default = "英雄" },
        { id = "active", name = "激活", type = "boolean", default = true }
    },

    -- ═══════════════════════════════════════════════════════════
    -- 输入端口 (可选) - 左侧黄色圆点
    -- ═══════════════════════════════════════════════════════════
    inputs = {
        { id = "trigger", name = "触发", type = "event" },
        { id = "value", name = "数值", type = "number", default = 0 }
    },

    -- ═══════════════════════════════════════════════════════════
    -- 输出端口 (可选) - 右侧蓝色圆点
    -- ═══════════════════════════════════════════════════════════
    outputs = {
        { id = "result", name = "结果", type = "number" },
        { id = "done", name = "完成", type = "event" }
    },

    -- ═══════════════════════════════════════════════════════════
    -- 执行函数 (必需) - 核心逻辑
    -- ═══════════════════════════════════════════════════════════
    execute = function(self, inputs)
        -- self.properties  → 属性值
        -- self.state       → 持久化状态 (跨执行保持)
        -- inputs           → 输入端口值

        return {
            result = inputs.value * 2,
            done = true  -- event 类型：非 nil 表示触发
        }
    end
}
```

---

## 数据类型

| 类型 | Lua 类型 | 端口颜色 | 说明 |
|------|----------|----------|------|
| `number` | number | 蓝色 | 数值 |
| `string` | string | 绿色 | 字符串 |
| `boolean` | boolean | 橙色 | 布尔值 |
| `event` | any/nil | 黄色 | 事件触发 (非 nil = 触发) |
| `any` | any | 灰色 | 任意类型 |
| `table` | table | 紫色 | 表/数组 |

---

## 核心概念

### 状态管理

`self.state` 在多次执行间保持：

```lua
execute = function(self, inputs)
    local state = self.state or { count = 0 }
    state.count = state.count + 1
    self.state = state
    return { count = state.count }
end
```

### 事件流

事件控制执行流程，只有触发时才执行主逻辑：

```lua
execute = function(self, inputs)
    if not inputs.trigger then
        return { result = 0, done = nil }  -- nil = 不触发下游
    end
    -- 有触发时执行
    return { result = 42, done = true }
end
```

### 动态输出端口

返回未在 `outputs` 中定义的字段会自动创建动态端口：

```lua
execute = function(self, inputs)
    local result = { count = 3 }
    -- 动态生成 dev1_name, dev2_name, dev3_name 端口
    for i = 1, 3 do
        result["dev" .. i .. "_name"] = "Device " .. i
    end
    return result
end
```

### 调试

```lua
execute = function(self, inputs)
    print("输入值:", inputs.value)
    print("属性:", self.properties.damage)
    print("状态:", self.state)
    return { result = 42 }
end
```

控制台 (`Ctrl+`` ) 查看输出。也可连接 `debug/logger` Block。

---
## 交互控件

通过 `meta.widget` 启用交互控件：

| widget | 说明 | state 字段 |
|--------|------|-----------|
| `textinput` | 文本框 | `widget_text` |
| `password` | 密码框 | `widget_text` |
| `textarea` | 多行文本 | `widget_text` |
| `button` | 按钮 | `widget_checked` |
| `checkbox` | 复选框 | `widget_checked` |
| `slider` | 滑块 | `widget_value` |

**示例：文本输入**
```lua
return {
    meta = {
        id = "input.text",
        name = "文本输入",
        widget = "textinput",
        placeholder = "请输入..."
    },
    outputs = {
        { id = "value", name = "文本", type = "string" }
    },
    execute = function(self, inputs)
        return { value = self.state.widget_text or "" }
    end
}
```

**示例：按钮**
```lua
execute = function(self, inputs)
    local state = self.state or {}
    local was = state.last_checked or false
    local now = self.state.widget_checked or false
    local clicked = now and not was
    state.last_checked = now
    self.state = state
    return { clicked = clicked and true or nil }
end
```

### hideable 属性

`meta.hideable = true` 时，预览模式下：
- 有连线 → 隐藏
- 无连线 → Mini 模式
- 悬停时 → 临时展开

适用于：常量节点、装备附件、技能节点等叶子节点。

---

## 动画系统

通过 `self.state._animation` 实现位置偏移动画：

```lua
self.state._animation = { x = 30, y = 0, speed = 300 }
```

| 参数 | 说明 |
|------|------|
| `x` | 水平偏移 (正=右) |
| `y` | 垂直偏移 (正=下) |
| `speed` | 速度 (像素/秒)，0=瞬移 |

**示例：攻击前冲**
```lua
if inputs.attack then
    self.state._animation = { x = 30, y = 0, speed = 300 }
else
    self.state._animation = { x = 0, y = 0, speed = 200 }
end
```

---

## USB 开发

全局 `usb` 表提供完整 USB 通信 API。

### 设备枚举

```lua
local devices = usb.devices()
for i, dev in ipairs(devices) do
    print(string.format("VID:%04X PID:%04X - %s",
        dev.vendor_id, dev.product_id, dev.product or "Unknown"))
end
```

**设备信息字段：**
| 字段 | 类型 | 说明 |
|------|------|------|
| `vendor_id` | number | VID |
| `product_id` | number | PID |
| `bus_number` | number | 总线号 |
| `address` | number | 地址 |
| `speed` | string | "low"/"full"/"high"/"super" |
| `manufacturer` | string? | 制造商 |
| `product` | string? | 产品名 |
| `serial_number` | string? | 序列号 |

### 打开设备

```lua
-- 通过 VID/PID
local device = usb.open(0x1234, 0x5678)

-- 通过总线地址
local device = usb.open_by_address(1, 5)
```

### 数据传输

**Bulk 传输** (大数据量)：
```lua
device:claim_interface(0)
local n = device:write_bulk(0x01, "Hello", 1000)  -- endpoint, data, timeout_ms
local result = device:read_bulk(0x81, 64, 1000)   -- endpoint, size, timeout_ms
-- result.data, result.length
```

**Interrupt 传输** (小数据/低延迟)：
```lua
device:write_interrupt(0x02, "\x01\x02", 100)
local result = device:read_interrupt(0x82, 8, 100)
```

**Control 传输**：
```lua
local result = device:read_control({
    request_type = usb.request_type("in", "vendor", "device"),
    request = 0x01, value = 0, index = 0, size = 64, timeout = 1000
})
```

### 接口管理

```lua
device:set_auto_detach_kernel_driver(true)  -- 推荐
device:claim_interface(0)
-- ... 传输操作 ...
device:release_interface(0)
```

### USB Block 示例
```lua
return {
    meta = { id = "usb.scanner", name = "USB 扫描", category = "USB", color = "#9C27B0" },
    outputs = {
        { id = "devices", name = "设备列表", type = "table" },
        { id = "count", name = "数量", type = "number" }
    },
    execute = function(self, inputs)
        local devices = usb.devices()
        return { devices = devices, count = #devices }
    end
}
```

### 错误处理

使用 `pcall` 包装 USB 操作：
```lua
local ok, result = pcall(function()
    local device = usb.open(0x1234, 0x5678)
    device:claim_interface(0)
    return device:read_bulk(0x81, 64, 1000)
end)

if ok then print("OK: " .. result.length)
else print("Error: " .. tostring(result)) end
```

**常见错误：**
| 错误 | 解决方案 |
|------|---------|
| Device not found | 检查 VID/PID 和连接 |
| Access denied | Linux: udev 规则; Windows: Zadig |
| Resource busy | 分离内核驱动 |
| Timeout | 增加超时时间 |

### 平台注意事项

**Linux** - 创建 `/etc/udev/rules.d/99-usb.rules`:
```
SUBSYSTEM=="usb", ATTR{idVendor}=="1234", MODE="0666"
```

**Windows** - 使用 [Zadig](https://zadig.akeo.ie/) 安装 WinUSB 驱动

**macOS** - 使用 `set_auto_detach_kernel_driver(true)`

---

## 最佳实践

### 命名规范

| 类型 | 规范 | 示例 |
|------|------|------|
| meta.id | `分类.名称` | `game.attack`, `util.counter` |
| 端口 id | 小写下划线 | `attack_power`, `is_valid` |
| 属性 id | 小写下划线 | `max_hp`, `crit_rate` |

### 代码风格

```lua
-- ✅ 好：提前返回，减少嵌套
execute = function(self, inputs)
    if not inputs.trigger then return { result = 0 } end
    return { result = inputs.value * 2 }
end

-- ❌ 差：过度嵌套
execute = function(self, inputs)
    if inputs.trigger then
        if inputs.value then
            return { result = inputs.value * 2 }
        end
    end
    return { result = 0 }
end
```

### 性能建议

1. **缓存计算结果** - 不变的数据存到 `self.state`
2. **避免在 execute 中创建大表** - 复用已有表
3. **USB 设备复用** - 通过 state 缓存已打开的设备
4. **减少 print 调用** - 生产环境移除调试输出

### 文件编码

支持 UTF-8 和 GBK，自动检测。推荐使用 UTF-8。

---

## 附录：目录结构

```
scripts/
├── game/        # 游戏实体
├── lite/        # Lite RPG
├── logic/       # 逻辑控制
├── math/        # 数学运算
├── input/       # 交互输入
├── usb/         # USB 设备
├── event/       # 事件
├── util/        # 工具
└── debug/       # 调试
```

## 附录：文件格式

| 扩展名 | 格式 | 用途 |
|--------|------|------|
| `.L` | 明文 JSON | 开发调试 |
| `.LZ` | AES 加密 | 源码保护 |
| `.lpack` | 加密包 | 独立发布 |
