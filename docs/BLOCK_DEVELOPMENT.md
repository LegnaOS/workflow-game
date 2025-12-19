# Block 开发指南

## 概述

Block 是工作流引擎的基本执行单元。每个 Block 是一个 Lua 脚本，定义了：

- **元数据** - ID、名称、颜色
- **端口** - 输入/输出
- **属性** - 可配置参数
- **执行逻辑** - Lua 函数

脚本放到 `scripts/` 目录即可，引擎自动扫描加载，修改后自动热重载。

## 目录结构

```
scripts/
├── game/           # 游戏实体
│   ├── character.lua   # 角色
│   ├── monster.lua     # 怪物
│   ├── attack.lua      # 攻击
│   ├── fireball.lua    # 火球术
│   └── inventory.lua   # 背包
├── input/          # 交互输入
│   ├── text_input.lua  # 文本输入
│   ├── password_input.lua # 密码输入
│   └── button.lua      # 按钮
├── logic/          # 逻辑控制
│   ├── branch.lua      # 条件分支
│   ├── compare.lua     # 比较
│   └── selector.lua    # 选择器
├── math/           # 数学运算
│   ├── add.lua         # 加法
│   ├── multiply.lua    # 乘法
│   ├── calc.lua        # 表达式
│   └── constant.lua    # 常量
├── util/           # 工具
│   ├── splitter.lua    # 分流
│   ├── merger.lua      # 合并
│   ├── switch.lua      # 开关
│   └── value.lua       # 取值
├── event/          # 事件
│   ├── on_start.lua    # 启动
│   └── print.lua       # 打印
└── debug/          # 调试
    └── logger.lua      # 日志
```

## Block 脚本格式

```lua
return {
    -- 元数据（必须）
    meta = {
        id = "category.name",      -- 唯一ID（必须）
        name = "显示名称",          -- UI显示名称
        category = "分类",          -- 分类名称
        description = "描述文字",   -- 悬停提示
        color = "#4CAF50",         -- 十六进制颜色
        hideable = false           -- 预览模式下可隐藏（可选，默认false）
    },

    -- 属性定义（可编辑参数）
    properties = {
        {
            id = "prop_id",        -- 属性ID
            name = "属性名",        -- 显示名称
            type = "number",       -- 类型：number/string/boolean
            default = 10,          -- 默认值
            min = 0,               -- 最小值（number类型）
            max = 100              -- 最大值（number类型）
        }
    },

    -- 输入端口
    inputs = {
        {
            id = "input_id",       -- 端口ID
            name = "输入名",        -- 显示名称
            type = "number",       -- 类型：number/string/boolean/event/any
            default = 0            -- 默认值
        }
    },

    -- 输出端口
    outputs = {
        {
            id = "output_id",      -- 端口ID
            name = "输出名",        -- 显示名称
            type = "number",       -- 类型
            default = 0            -- 默认值
        }
    },

    -- 执行函数（核心逻辑）
    execute = function(self, inputs)
        -- self.properties: 访问属性值
        -- self.state: 持久化状态（跨执行保持）
        -- inputs: 输入端口值
        
        local result = inputs.input_id * 2
        
        return {
            output_id = result     -- 返回输出值
        }
    end
}
```

## 数据类型

| 类型 | Lua类型 | 说明 |
|------|---------|------|
| `number` | number | 数值 |
| `string` | string | 字符串 |
| `boolean` | boolean | 布尔值 |
| `event` | any/nil | 事件触发（非nil表示触发） |
| `any` | any | 任意类型 |

## 状态管理

Block 可以通过 `self.state` 保持跨执行的状态：

```lua
execute = function(self, inputs)
    local state = self.state or {}
    
    -- 读取状态
    local count = state.count or 0
    count = count + 1
    
    -- 保存状态
    self.state = { count = count }
    
    return { count_out = count }
end
```

## 事件系统

事件用于控制执行流程：

```lua
-- 输入事件
inputs = {
    { id = "trigger", name = "触发", type = "event" }
},

execute = function(self, inputs)
    -- 检查事件是否触发
    if inputs.trigger then
        -- 执行逻辑
        return { result = 42, event_out = true }
    end
    return { result = 0, event_out = nil }
end
```

## 示例：计数器Block

```lua
return {
    meta = {
        id = "util.counter",
        name = "计数器",
        category = "工具",
        color = "#2196F3"
    },

    properties = {
        { id = "step", name = "步长", type = "number", default = 1 },
        { id = "max", name = "最大值", type = "number", default = 100 }
    },

    inputs = {
        { id = "increment", name = "增加", type = "event" },
        { id = "reset", name = "重置", type = "event" }
    },

    outputs = {
        { id = "value", name = "当前值", type = "number", default = 0 },
        { id = "overflow", name = "溢出", type = "event" }
    },

    execute = function(self, inputs)
        local state = self.state or { value = 0 }
        local props = self.properties
        
        if inputs.reset then
            state.value = 0
        elseif inputs.increment then
            state.value = state.value + (props.step or 1)
        end
        
        local overflow = nil
        if state.value >= (props.max or 100) then
            overflow = true
            state.value = 0
        end
        
        self.state = state
        return { value = state.value, overflow = overflow }
    end
}
```

## 热重载

保存脚本后，引擎自动重新加载。控制台会显示加载日志。

如果脚本有语法错误，会在控制台输出错误信息，Block 列表中不会显示该 Block。

## 调试

```lua
execute = function(self, inputs)
    -- 打印到控制台
    print("收到输入:", inputs.value)
    print("当前属性:", self.properties.damage)
    print("当前状态:", self.state)

    return { result = 42 }
end
```

也可以连接 `debug/logger` Block 在界面上查看数据流。

## 编码

脚本支持 UTF-8 和 GBK 编码，自动检测。Windows 用户可以用记事本直接编辑，不用担心中文乱码。

## 完整示例：伤害计算

```lua
return {
    meta = {
        id = "game.damage_calc",
        name = "伤害计算",
        category = "战斗",
        description = "计算最终伤害 = (攻击力 - 防御力) * 暴击倍率",
        color = "#E91E63"
    },

    properties = {
        { id = "crit_mult", name = "暴击倍率", type = "number", default = 1.5, min = 1, max = 5 },
        { id = "min_damage", name = "最小伤害", type = "number", default = 1 }
    },

    inputs = {
        { id = "attack", name = "攻击力", type = "number", default = 0 },
        { id = "defense", name = "防御力", type = "number", default = 0 },
        { id = "is_crit", name = "是否暴击", type = "boolean", default = false }
    },

    outputs = {
        { id = "damage", name = "最终伤害", type = "number", default = 0 },
        { id = "is_kill", name = "是否击杀", type = "event" }
    },

    execute = function(self, inputs)
        local props = self.properties
        local base = inputs.attack - inputs.defense

        -- 保底伤害
        if base < props.min_damage then
            base = props.min_damage
        end

        -- 暴击
        local final = base
        if inputs.is_crit then
            final = base * props.crit_mult
        end

        -- 记录统计
        local state = self.state or { total = 0, count = 0 }
        state.total = state.total + final
        state.count = state.count + 1
        self.state = state

        return {
            damage = final,
            is_kill = nil  -- 需要连接目标HP来判断
        }
    end
}
```

## 进阶：事件链

事件类型用于控制执行流程。只有当输入事件为非 nil 时，Block 才会执行主逻辑。

```lua
return {
    meta = {
        id = "game.on_hit",
        name = "受击事件",
        category = "事件",
        color = "#FF9800"
    },

    inputs = {
        { id = "trigger", name = "触发", type = "event" },
        { id = "damage", name = "伤害值", type = "number", default = 0 }
    },

    outputs = {
        { id = "on_normal", name = "普通受击", type = "event" },
        { id = "on_critical", name = "重击", type = "event" },
        { id = "on_death", name = "死亡", type = "event" }
    },

    execute = function(self, inputs)
        if not inputs.trigger then
            return { on_normal = nil, on_critical = nil, on_death = nil }
        end

        local dmg = inputs.damage
        if dmg >= 100 then
            return { on_normal = nil, on_critical = nil, on_death = true }
        elseif dmg >= 50 then
            return { on_normal = nil, on_critical = true, on_death = nil }
        else
            return { on_normal = true, on_critical = nil, on_death = nil }
        end
    end
}
```

## Block 动画系统

Block 可以通过设置 `self.state._animation` 来实现位置偏移动画效果（如攻击时前冲、受击后退等）。

```lua
execute = function(self, inputs)
    -- 设置动画：x/y 为偏移量（像素），speed 为移动速度（像素/秒）
    if inputs.attack_trigger then
        -- 攻击时向右移动 30 像素
        self.state._animation = { x = 30, y = 0, speed = 300 }
    else
        -- 没有攻击时回到原位
        self.state._animation = { x = 0, y = 0, speed = 200 }
    end

    return { ... }
end
```

### 动画参数

| 参数 | 类型 | 说明 |
|------|------|------|
| `x` | number | 水平偏移量（正值向右，负值向左） |
| `y` | number | 垂直偏移量（正值向下，负值向上） |
| `speed` | number | 移动速度（像素/秒），0 表示瞬移 |

### 示例：角色攻击动画

```lua
-- 角色攻击时前冲
if inputs.action_trigger then
    self.state._animation = { x = 30, y = 0, speed = 300 }
else
    self.state._animation = { x = 0, y = 0, speed = 200 }
end
```

### 示例：怪物受击动画

```lua
-- 怪物受击时后退
if inputs.attack_event then
    self.state._animation = { x = -20, y = 0, speed = 400 }
elseif is_dead then
    -- 死亡时下沉
    self.state._animation = { x = 0, y = 30, speed = 100 }
else
    self.state._animation = { x = 0, y = 0, speed = 200 }
end
```

## 可交互 Block

Block 可以包含交互控件（输入框、按钮等），通过在 `meta` 中设置 `widget` 属性启用。

### 控件类型

| widget 值 | 说明 | 用途 |
|-----------|------|------|
| `textinput` | 文本输入框 | 用户输入文本 |
| `password` | 密码输入框 | 密码输入（显示掩码） |
| `textarea` | 多行文本框 | 长文本输入 |
| `checkbox` | 复选框 | 开关选项 |
| `slider` | 滑块 | 数值调节 |
| `button` | 按钮 | 触发事件 |

### meta 扩展字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `widget` | string | 控件类型 |
| `placeholder` | string | 占位符/提示文字 |
| `options` | array | 下拉选项（dropdown 类型） |
| `hideable` | boolean | 预览模式下可隐藏（默认 false） |

### hideable 属性

当 `hideable = true` 时，该 Block 在预览模式下的行为：
- **有连线时**：Block 被隐藏，不显示在画布上
- **孤立时（无连线）**：Block 以 Mini 模式显示
- **悬停/选中时**：临时展开为完整模式

适合设置 `hideable = true` 的 Block 类型：
- 常量值节点（只输出固定值）
- 宝石/装备附件（镶嵌到主装备上）
- 技能节点（连接到角色上）
- 其他"叶子节点"（无输入端口，只提供数据）

### 示例：文本输入 Block

```lua
return {
    meta = {
        id = "input.text_input",
        name = "文本输入",
        category = "输入",
        color = "#2196F3",
        widget = "textinput",           -- 启用文本输入控件
        placeholder = "请输入文本..."    -- 占位符
    },

    outputs = {
        { id = "value", name = "文本值", type = "string", default = "" },
        { id = "length", name = "文本长度", type = "number", default = 0 }
    },

    execute = function(self, inputs)
        -- 控件值自动同步到 output 的 value 端口
        local text = self.state.widget_text or ""
        return {
            value = text,
            length = string.len(text)
        }
    end
}
```

### 示例：密码输入 Block

```lua
return {
    meta = {
        id = "input.password",
        name = "密码输入",
        color = "#FF5722",
        widget = "password",
        placeholder = "请输入密码..."
    },

    properties = {
        { id = "min_length", name = "最小长度", type = "number", default = 6 }
    },

    outputs = {
        { id = "value", name = "密码值", type = "string", default = "" },
        { id = "is_valid", name = "有效", type = "boolean", default = false }
    },

    execute = function(self, inputs)
        local password = self.state.widget_text or ""
        local min_len = self.properties.min_length or 6
        return {
            value = password,
            is_valid = string.len(password) >= min_len
        }
    end
}
```

### 示例：按钮 Block

```lua
return {
    meta = {
        id = "input.button",
        name = "按钮",
        color = "#4CAF50",
        widget = "button",
        placeholder = "点击执行"
    },

    outputs = {
        { id = "clicked", name = "点击事件", type = "event" },
        { id = "click_count", name = "点击次数", type = "number", default = 0 }
    },

    execute = function(self, inputs)
        local state = self.state or {}
        local count = state.click_count or 0
        local was_checked = state.last_checked or false
        local is_checked = self.state.widget_checked or false

        local clicked = is_checked and not was_checked
        if clicked then
            count = count + 1
        end

        state.click_count = count
        state.last_checked = is_checked
        self.state = state

        return {
            clicked = clicked and true or nil,
            click_count = count
        }
    end
}
```

## 图层系统

工作流支持多图层，每个图层是画布的一个独立区域。切换图层时视口会自动跳转到该图层的位置。

- **新建图层**：点击左侧图层面板的 "+" 按钮
- **切换图层**：点击图层名称
- **重命名**：双击图层名称进入编辑模式
- **删除图层**：点击图层右侧的 "×" 按钮

图层信息保存在工作流文件中，与 Block 一起持久化。

## 工作流文件格式

| 扩展名 | 说明 | 用途 |
|--------|------|------|
| `.L` | 明文 JSON | 开发调试 |
| `.LZ` | AES 加密 | 源码保护 |
| `.lpack` | 加密游戏包 | 独立发布 |

### .lpack 游戏包

通过 IDE 的「📦 发布」按钮生成，包含：
- 工作流定义（节点、连线、图层）
- 所有用到的 Lua 脚本源码
- AES-128-CBC 加密保护

游戏包可以被独立播放器 `workflow_player` 直接运行，无需 IDE。

```
发布目录结构：
游戏名_publish/
├── workflow_player      # 播放器
└── 游戏名.lpack         # 加密游戏包
```

---

## USB 模块开发指南

工作流引擎内置了完整的 USB 通信支持，可以在 Lua Block 中直接访问 USB 设备。

### 1. USB 模块概述

USB 模块通过全局 `usb` 表暴露，提供以下功能：

| 功能分类 | 说明 |
|---------|------|
| 设备枚举 | 列出所有连接的 USB 设备 |
| 设备打开 | 通过 VID/PID 或总线地址打开设备 |
| 数据传输 | Bulk、Interrupt、Control 三种传输类型 |
| 接口管理 | 声明/释放接口、设置配置 |
| 内核驱动 | 分离/附加内核驱动（Linux/macOS） |
| 描述符读取 | 设备、配置、接口、端点描述符 |

### 2. 快速入门

#### 2.1 列出所有 USB 设备

```lua
-- 获取所有连接的设备
local devices = usb.devices()

for i, dev in ipairs(devices) do
    print(string.format("[%d] VID:%04X PID:%04X - %s",
        i, dev.vendor_id, dev.product_id, dev.product or "Unknown"))
end
```

返回的设备信息表包含：

| 字段 | 类型 | 说明 |
|------|------|------|
| `bus_number` | number | USB 总线号 |
| `address` | number | 设备地址 |
| `vendor_id` | number | 厂商 ID (VID) |
| `product_id` | number | 产品 ID (PID) |
| `class_code` | number | 设备类代码 |
| `subclass_code` | number | 子类代码 |
| `protocol_code` | number | 协议代码 |
| `speed` | string | 速度: "low"/"full"/"high"/"super" |
| `manufacturer` | string? | 制造商名称（可能为 nil） |
| `product` | string? | 产品名称（可能为 nil） |
| `serial_number` | string? | 序列号（可能为 nil） |

#### 2.2 打开设备

```lua
-- 方法1: 通过 VID/PID 打开
local device = usb.open(0x1234, 0x5678)

-- 方法2: 通过总线地址打开
local device = usb.open_by_address(1, 5)  -- bus=1, address=5

-- 方法3: 使用独立上下文（隔离资源）
local ctx = usb.context()
local device = ctx:open(0x1234, 0x5678)
```

### 3. 设备操作 API

#### 3.1 配置与接口

```lua
-- 获取设备描述符
local desc = device:descriptor()
print("VID:PID = " .. string.format("%04X:%04X", desc.vendor_id, desc.product_id))
print("USB Version: " .. desc.usb_version)
print("Device Version: " .. desc.device_version)

-- 获取当前/指定配置
local config = device:config()       -- 当前激活的配置
local config = device:config(0)      -- 第一个配置

-- 设置配置
device:set_configuration(1)

-- 声明接口（必须在传输前调用）
device:claim_interface(0)

-- 释放接口
device:release_interface(0)

-- 设置备用设置
device:set_alternate_setting(0, 1)  -- interface=0, alt_setting=1

-- 重置设备
device:reset()

-- 清除端点停止状态
device:clear_halt(0x81)  -- endpoint 0x81
```

#### 3.2 内核驱动管理（Linux/macOS）

```lua
-- 检查内核驱动是否激活
if device:kernel_driver_active(0) then
    -- 分离内核驱动
    device:detach_kernel_driver(0)
end

-- 设置自动分离（推荐）
device:set_auto_detach_kernel_driver(true)

-- 重新附加内核驱动
device:attach_kernel_driver(0)
```

### 4. 数据传输

#### 4.1 Bulk 传输（大数据量）

```lua
-- 声明接口
device:claim_interface(0)

-- 写入数据
local data = "Hello USB!"
local bytes_written = device:write_bulk(0x01, data, 1000)  -- endpoint, data, timeout_ms
print("Written: " .. bytes_written .. " bytes")

-- 读取数据
local result = device:read_bulk(0x81, 64, 1000)  -- endpoint, size, timeout_ms
print("Read: " .. result.length .. " bytes")
print("Data: " .. result.data)
```

#### 4.2 Interrupt 传输（小数据、低延迟）

```lua
-- 写入
local n = device:write_interrupt(0x02, "\x01\x02\x03", 100)

-- 读取
local result = device:read_interrupt(0x82, 8, 100)
```

#### 4.3 Control 传输（配置/控制）

```lua
-- 读取控制传输
local result = device:read_control({
    request_type = usb.request_type("in", "vendor", "device"),
    request = 0x01,
    value = 0x0000,
    index = 0x0000,
    size = 64,
    timeout = 1000
})

-- 写入控制传输
local n = device:write_control({
    request_type = usb.request_type("out", "vendor", "device"),
    request = 0x02,
    value = 0x0100,
    index = 0x0000,
    data = "\x01\x02\x03\x04",
    timeout = 1000
})
```

#### 4.4 request_type 构建器

```lua
-- usb.request_type(direction, type, recipient)
-- direction: "in" / "out"
-- type: "standard" / "class" / "vendor"
-- recipient: "device" / "interface" / "endpoint" / "other"

local rt = usb.request_type("in", "vendor", "device")   -- 0xC0
local rt = usb.request_type("out", "class", "interface") -- 0x21
```

### 5. 字符串描述符

```lua
-- 读取 ASCII 字符串描述符
local str = device:read_string_ascii(1)  -- index

-- 便捷方法
local manufacturer = device:manufacturer()
local product = device:product()
local serial = device:serial_number()

-- 获取支持的语言列表
local langs = device:languages(1000)  -- timeout_ms
for _, lang in ipairs(langs) do
    print("Language ID: " .. lang.lang_id)
end
```

### 6. 常量定义

所有 USB 常量通过 `usb.const` 表访问：

```lua
-- 端点方向
usb.const.ENDPOINT_IN   -- 0x80
usb.const.ENDPOINT_OUT  -- 0x00

-- 传输类型
usb.const.TRANSFER_TYPE_CONTROL     -- 0
usb.const.TRANSFER_TYPE_ISOCHRONOUS -- 1
usb.const.TRANSFER_TYPE_BULK        -- 2
usb.const.TRANSFER_TYPE_INTERRUPT   -- 3

-- 请求类型
usb.const.REQUEST_TYPE_STANDARD -- 0x00
usb.const.REQUEST_TYPE_CLASS    -- 0x20
usb.const.REQUEST_TYPE_VENDOR   -- 0x40

-- 请求接收者
usb.const.RECIPIENT_DEVICE    -- 0x00
usb.const.RECIPIENT_INTERFACE -- 0x01
usb.const.RECIPIENT_ENDPOINT  -- 0x02
usb.const.RECIPIENT_OTHER     -- 0x03

-- 标准请求
usb.const.REQUEST_GET_STATUS        -- 0x00
usb.const.REQUEST_CLEAR_FEATURE     -- 0x01
usb.const.REQUEST_SET_FEATURE       -- 0x03
usb.const.REQUEST_GET_DESCRIPTOR    -- 0x06
usb.const.REQUEST_SET_CONFIGURATION -- 0x09

-- 描述符类型
usb.const.DESCRIPTOR_TYPE_DEVICE    -- 0x01
usb.const.DESCRIPTOR_TYPE_CONFIG    -- 0x02
usb.const.DESCRIPTOR_TYPE_STRING    -- 0x03
usb.const.DESCRIPTOR_TYPE_INTERFACE -- 0x04
usb.const.DESCRIPTOR_TYPE_ENDPOINT  -- 0x05

-- 设备类代码
usb.const.CLASS_HID          -- 0x03
usb.const.CLASS_MASS_STORAGE -- 0x08
usb.const.CLASS_HUB          -- 0x09
usb.const.CLASS_VENDOR_SPEC  -- 0xFF
```

### 7. 版本与功能检测

```lua
-- libusb 版本
local ver = usb.version()
print(string.format("libusb %d.%d.%d", ver.major, ver.minor, ver.micro))

-- 功能检测
if usb.has_hotplug() then
    print("支持热插拔")
end

if usb.supports_detach_kernel_driver() then
    print("支持分离内核驱动")
end
```

### 8. USB Block 开发示例

#### 8.1 设备扫描 Block

```lua
-- scripts/usb/device_scanner.lua
return {
    meta = {
        id = "usb.device_scanner",
        name = "USB 设备扫描",
        category = "USB",
        description = "扫描并列出所有 USB 设备",
        color = "#9C27B0"
    },

    properties = {
        { id = "vid_filter", name = "VID 过滤", type = "string", default = "" },
        { id = "pid_filter", name = "PID 过滤", type = "string", default = "" }
    },

    inputs = {
        { id = "trigger", name = "触发", type = "event" }
    },

    outputs = {
        { id = "devices", name = "设备列表", type = "table" },
        { id = "count", name = "设备数量", type = "number" }
    },

    execute = function(self, inputs)
        local props = self.properties or {}
        local all_devices = usb.devices()
        local filtered = {}

        local vid_filter = props.vid_filter ~= "" and tonumber(props.vid_filter, 16) or nil
        local pid_filter = props.pid_filter ~= "" and tonumber(props.pid_filter, 16) or nil

        for _, dev in ipairs(all_devices) do
            local match = true
            if vid_filter and dev.vendor_id ~= vid_filter then match = false end
            if pid_filter and dev.product_id ~= pid_filter then match = false end
            if match then table.insert(filtered, dev) end
        end

        return { devices = filtered, count = #filtered }
    end
}
```

#### 8.2 Bulk 读取 Block

```lua
-- scripts/usb/bulk_reader.lua
return {
    meta = {
        id = "usb.bulk_reader",
        name = "USB Bulk 读取",
        category = "USB",
        description = "从 USB 设备读取 Bulk 数据",
        color = "#9C27B0"
    },

    properties = {
        { id = "vid", name = "VID (十六进制)", type = "string", default = "0000" },
        { id = "pid", name = "PID (十六进制)", type = "string", default = "0000" },
        { id = "endpoint", name = "端点地址", type = "number", default = 129, min = 0, max = 255 },
        { id = "size", name = "读取大小", type = "number", default = 64, min = 1, max = 4096 },
        { id = "timeout", name = "超时(ms)", type = "number", default = 1000, min = 100, max = 30000 }
    },

    inputs = {
        { id = "trigger", name = "触发", type = "event" }
    },

    outputs = {
        { id = "data", name = "数据", type = "string" },
        { id = "length", name = "长度", type = "number" },
        { id = "success", name = "成功", type = "boolean" },
        { id = "error", name = "错误", type = "string" }
    },

    execute = function(self, inputs)
        local props = self.properties or {}
        local state = self.state or {}

        local vid = tonumber(props.vid, 16) or 0
        local pid = tonumber(props.pid, 16) or 0

        -- 尝试复用已打开的设备
        if not state.device or state.vid ~= vid or state.pid ~= pid then
            if state.device then
                pcall(function() state.device:close() end)
            end

            local ok, dev = pcall(usb.open, vid, pid)
            if not ok then
                return { success = false, error = "无法打开设备: " .. tostring(dev) }
            end

            state.device = dev
            state.vid = vid
            state.pid = pid

            -- 自动分离内核驱动并声明接口
            pcall(function() dev:set_auto_detach_kernel_driver(true) end)
            pcall(function() dev:claim_interface(0) end)
        end

        -- 读取数据
        local ok, result = pcall(function()
            return state.device:read_bulk(props.endpoint, props.size, props.timeout)
        end)

        if ok then
            return { data = result.data, length = result.length, success = true, error = "" }
        else
            return { data = "", length = 0, success = false, error = tostring(result) }
        end
    end
}
```

#### 8.3 Control 传输 Block

```lua
-- scripts/usb/control_transfer.lua
return {
    meta = {
        id = "usb.control_transfer",
        name = "USB Control 传输",
        category = "USB",
        description = "发送 USB 控制传输请求",
        color = "#9C27B0"
    },

    properties = {
        { id = "vid", name = "VID", type = "string", default = "0000" },
        { id = "pid", name = "PID", type = "string", default = "0000" },
        { id = "direction", name = "方向", type = "string", default = "in" },
        { id = "req_type", name = "类型", type = "string", default = "vendor" },
        { id = "request", name = "请求码", type = "number", default = 0, min = 0, max = 255 },
        { id = "value", name = "wValue", type = "number", default = 0 },
        { id = "index", name = "wIndex", type = "number", default = 0 },
        { id = "size", name = "数据大小", type = "number", default = 64 }
    },

    inputs = {
        { id = "trigger", name = "触发", type = "event" },
        { id = "data", name = "发送数据", type = "string" }
    },

    outputs = {
        { id = "response", name = "响应", type = "string" },
        { id = "length", name = "长度", type = "number" },
        { id = "success", name = "成功", type = "boolean" }
    },

    execute = function(self, inputs)
        local props = self.properties or {}

        local vid = tonumber(props.vid, 16) or 0
        local pid = tonumber(props.pid, 16) or 0

        local ok, device = pcall(usb.open, vid, pid)
        if not ok then
            return { response = "", length = 0, success = false }
        end

        local rt = usb.request_type(props.direction or "in", props.req_type or "vendor", "device")

        if props.direction == "in" then
            local ok, result = pcall(function()
                return device:read_control({
                    request_type = rt,
                    request = props.request or 0,
                    value = props.value or 0,
                    index = props.index or 0,
                    size = props.size or 64,
                    timeout = 1000
                })
            end)
            device:close()

            if ok then
                return { response = result.data, length = result.length, success = true }
            else
                return { response = "", length = 0, success = false }
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
                    timeout = 1000
                })
            end)
            device:close()

            return { response = "", length = ok and n or 0, success = ok }
        end
    end
}
```

### 9. 错误处理

USB 操作可能因各种原因失败，建议使用 `pcall` 包装：

```lua
local ok, result = pcall(function()
    local device = usb.open(0x1234, 0x5678)
    device:claim_interface(0)
    return device:read_bulk(0x81, 64, 1000)
end)

if ok then
    print("读取成功: " .. result.length .. " bytes")
else
    print("错误: " .. tostring(result))
end
```

常见错误：

| 错误 | 原因 | 解决方案 |
|------|------|---------|
| Device not found | 设备未连接或 VID/PID 错误 | 检查设备连接和 ID |
| Access denied | 权限不足 | Linux: 添加 udev 规则; macOS: 检查驱动 |
| Resource busy | 设备被其他程序占用 | 关闭占用程序或分离内核驱动 |
| Timeout | 操作超时 | 增加超时时间或检查设备状态 |
| Pipe error | 端点错误或设备拒绝 | 检查端点地址和传输类型 |

### 10. 平台注意事项

#### Linux
- 需要 udev 规则或 root 权限访问 USB 设备
- 创建 `/etc/udev/rules.d/99-usb.rules`:
  ```
  SUBSYSTEM=="usb", ATTR{idVendor}=="1234", ATTR{idProduct}=="5678", MODE="0666"
  ```
- 重载: `sudo udevadm control --reload-rules`

#### macOS
- 某些设备可能被系统驱动占用
- 使用 `set_auto_detach_kernel_driver(true)` 自动分离

#### Windows
- 需要安装 WinUSB 或 libusb 驱动
- 使用 Zadig 工具替换驱动

