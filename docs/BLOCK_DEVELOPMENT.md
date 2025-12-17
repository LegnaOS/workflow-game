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
        color = "#4CAF50"          -- 十六进制颜色
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

## 工作流文件格式

| 扩展名 | 说明 | 用途 |
|--------|------|------|
| `.L` | 明文 JSON | 开发调试 |
| `.LZ` | AES 加密 | 防止篡改 |
| `.dist.L` | 明文只读 | 分发 |
| `.dist.LZ` | 加密只读 | 正式发布 |

加密使用 AES-256-GCM，密码通过 PBKDF2 派生。

