# Block Development Guide

> A Block is the basic execution unit of the workflow engine. Each Block is a Lua script.
> Drop into `scripts/` directory, supports hot reload.

---

## Table of Contents

- [Quick Start](#quick-start)
- [Script Structure](#script-structure)
- [Data Types](#data-types)
- [Core Concepts](#core-concepts)
- [Interactive Widgets](#interactive-widgets)
- [Animation System](#animation-system)
- [USB Development](#usb-development)
- [Best Practices](#best-practices)

---

## Quick Start

**Minimal example** - Create `scripts/my/double.lua`:

```lua
return {
    meta = {
        id = "my.double",
        name = "Double",
        category = "My",
        color = "#FF5722"
    },
    inputs = {
        { id = "value", name = "Input", type = "number", default = 0 }
    },
    outputs = {
        { id = "result", name = "Result", type = "number" }
    },
    execute = function(self, inputs)
        return { result = inputs.value * 2 }
    end
}
```

Appears immediately in IDE left panel under "My" category.

---

## Script Structure

```lua
return {
    -- ═══════════════════════════════════════════════════════════
    -- Metadata (required)
    -- ═══════════════════════════════════════════════════════════
    meta = {
        id = "category.name",       -- Unique identifier (required)
        name = "Display Name",      -- Block title
        category = "Category",      -- Left panel category
        description = "Tooltip",    -- Hover description
        color = "#4CAF50",          -- Title bar color
        hideable = false,           -- Hide in preview mode (optional)
        widget = nil                -- Interactive widget type (optional)
    },

    -- ═══════════════════════════════════════════════════════════
    -- Properties (optional) - Editable in right panel
    -- ═══════════════════════════════════════════════════════════
    properties = {
        { id = "damage", name = "Damage", type = "number", default = 10, min = 0, max = 999 },
        { id = "name", name = "Name", type = "string", default = "Hero" },
        { id = "active", name = "Active", type = "boolean", default = true }
    },

    -- ═══════════════════════════════════════════════════════════
    -- Input ports (optional) - Left side yellow dots
    -- ═══════════════════════════════════════════════════════════
    inputs = {
        { id = "trigger", name = "Trigger", type = "event" },
        { id = "value", name = "Value", type = "number", default = 0 }
    },

    -- ═══════════════════════════════════════════════════════════
    -- Output ports (optional) - Right side blue dots
    -- ═══════════════════════════════════════════════════════════
    outputs = {
        { id = "result", name = "Result", type = "number" },
        { id = "done", name = "Done", type = "event" }
    },

    -- ═══════════════════════════════════════════════════════════
    -- Execute function (required) - Core logic
    -- ═══════════════════════════════════════════════════════════
    execute = function(self, inputs)
        -- self.properties  → Property values
        -- self.state       → Persistent state (across executions)
        -- inputs           → Input port values

        return {
            result = inputs.value * 2,
            done = true  -- event type: non-nil = triggered
        }
    end
}
```

---

## Data Types

| Type | Lua Type | Port Color | Description |
|------|----------|------------|-------------|
| `number` | number | Blue | Numeric value |
| `string` | string | Green | Text string |
| `boolean` | boolean | Orange | True/false |
| `event` | any/nil | Yellow | Event trigger (non-nil = triggered) |
| `any` | any | Gray | Any type |
| `table` | table | Purple | Table/array |

---

## Core Concepts

### State Management

`self.state` persists across executions:

```lua
execute = function(self, inputs)
    local state = self.state or { count = 0 }
    state.count = state.count + 1
    self.state = state
    return { count = state.count }
end
```

### Event Flow

Events control execution flow, only execute main logic when triggered:

```lua
execute = function(self, inputs)
    if not inputs.trigger then
        return { result = 0, done = nil }  -- nil = don't trigger downstream
    end
    -- Execute when triggered
    return { result = 42, done = true }
end
```

### Dynamic Output Ports

Returning fields not defined in `outputs` auto-creates dynamic ports:

```lua
execute = function(self, inputs)
    local result = { count = 3 }
    -- Dynamically generate dev1_name, dev2_name, dev3_name ports
    for i = 1, 3 do
        result["dev" .. i .. "_name"] = "Device " .. i
    end
    return result
end
```

### Debugging

```lua
execute = function(self, inputs)
    print("Input:", inputs.value)
    print("Property:", self.properties.damage)
    print("State:", self.state)
    return { result = 42 }
end
```

View in console (`Ctrl+`` ). Can also connect `debug/logger` Block.

---

## Interactive Widgets

Enable via `meta.widget`:

| widget | Description | state field |
|--------|-------------|-------------|
| `textinput` | Text box | `widget_text` |
| `password` | Password box | `widget_text` |
| `textarea` | Multi-line text | `widget_text` |
| `button` | Button | `widget_checked` |
| `checkbox` | Checkbox | `widget_checked` |
| `slider` | Slider | `widget_value` |

**Example: Text input**
```lua
return {
    meta = {
        id = "input.text",
        name = "Text Input",
        widget = "textinput",
        placeholder = "Enter..."
    },
    outputs = {
        { id = "value", name = "Text", type = "string" }
    },
    execute = function(self, inputs)
        return { value = self.state.widget_text or "" }
    end
}
```

**Example: Button**
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

### hideable Property

When `meta.hideable = true`, in preview mode:
- Has connections → Hidden
- No connections → Mini mode
- On hover → Temporarily expand

Suitable for: constant nodes, equipment attachments, skill nodes, etc.

---

## Animation System

Use `self.state._animation` for position offset animations:

```lua
self.state._animation = { x = 30, y = 0, speed = 300 }
```

| Parameter | Description |
|-----------|-------------|
| `x` | Horizontal offset (positive=right) |
| `y` | Vertical offset (positive=down) |
| `speed` | Speed (pixels/sec), 0=instant |

**Example: Attack lunge**
```lua
if inputs.attack then
    self.state._animation = { x = 30, y = 0, speed = 300 }
else
    self.state._animation = { x = 0, y = 0, speed = 200 }
end
```

---

## USB Development

Global `usb` table provides complete USB communication API.

### Device Enumeration

```lua
local devices = usb.devices()
for i, dev in ipairs(devices) do
    print(string.format("VID:%04X PID:%04X - %s",
        dev.vendor_id, dev.product_id, dev.product or "Unknown"))
end
```

**Device info fields:**
| Field | Type | Description |
|-------|------|-------------|
| `vendor_id` | number | VID |
| `product_id` | number | PID |
| `bus_number` | number | Bus number |
| `address` | number | Address |
| `speed` | string | "low"/"full"/"high"/"super" |
| `manufacturer` | string? | Manufacturer |
| `product` | string? | Product name |
| `serial_number` | string? | Serial number |

### Opening Devices

```lua
-- By VID/PID
local device = usb.open(0x1234, 0x5678)

-- By bus address
local device = usb.open_by_address(1, 5)
```

### Data Transfer

**Bulk transfer** (large data):
```lua
device:claim_interface(0)
local n = device:write_bulk(0x01, "Hello", 1000)  -- endpoint, data, timeout_ms
local result = device:read_bulk(0x81, 64, 1000)   -- endpoint, size, timeout_ms
-- result.data, result.length
```

**Interrupt transfer** (small data/low latency):
```lua
device:write_interrupt(0x02, "\x01\x02", 100)
local result = device:read_interrupt(0x82, 8, 100)
```

**Control transfer**:
```lua
local result = device:read_control({
    request_type = usb.request_type("in", "vendor", "device"),
    request = 0x01, value = 0, index = 0, size = 64, timeout = 1000
})
```

### Interface Management

```lua
device:set_auto_detach_kernel_driver(true)  -- Recommended
device:claim_interface(0)
-- ... transfer operations ...
device:release_interface(0)
```

### USB Block Example
```lua
return {
    meta = { id = "usb.scanner", name = "USB Scanner", category = "USB", color = "#9C27B0" },
    outputs = {
        { id = "devices", name = "Device List", type = "table" },
        { id = "count", name = "Count", type = "number" }
    },
    execute = function(self, inputs)
        local devices = usb.devices()
        return { devices = devices, count = #devices }
    end
}
```

### Error Handling

Wrap USB operations with `pcall`:
```lua
local ok, result = pcall(function()
    local device = usb.open(0x1234, 0x5678)
    device:claim_interface(0)
    return device:read_bulk(0x81, 64, 1000)
end)

if ok then print("OK: " .. result.length)
else print("Error: " .. tostring(result)) end
```

**Common errors:**
| Error | Solution |
|-------|----------|
| Device not found | Check VID/PID and connection |
| Access denied | Linux: udev rules; Windows: Zadig |
| Resource busy | Detach kernel driver |
| Timeout | Increase timeout |

### Platform Notes

**Linux** - Create `/etc/udev/rules.d/99-usb.rules`:
```
SUBSYSTEM=="usb", ATTR{idVendor}=="1234", MODE="0666"
```

**Windows** - Use [Zadig](https://zadig.akeo.ie/) to install WinUSB driver

**macOS** - Use `set_auto_detach_kernel_driver(true)`

---

## Best Practices

### Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| meta.id | `category.name` | `game.attack`, `util.counter` |
| port id | lowercase_underscore | `attack_power`, `is_valid` |
| property id | lowercase_underscore | `max_hp`, `crit_rate` |

### Code Style

```lua
-- ✅ Good: early return, less nesting
execute = function(self, inputs)
    if not inputs.trigger then return { result = 0 } end
    return { result = inputs.value * 2 }
end

-- ❌ Bad: excessive nesting
execute = function(self, inputs)
    if inputs.trigger then
        if inputs.value then
            return { result = inputs.value * 2 }
        end
    end
    return { result = 0 }
end
```

### Performance Tips

1. **Cache computed results** - Store unchanging data in `self.state`
2. **Avoid creating large tables in execute** - Reuse existing tables
3. **Reuse USB devices** - Cache opened devices in state
4. **Remove print calls** - Remove debug output in production

### File Encoding

Supports UTF-8 and GBK, auto-detected. UTF-8 recommended.

---

## Appendix: Directory Structure

```
scripts/
├── game/        # Game entities
├── lite/        # Lite RPG
├── logic/       # Logic control
├── math/        # Math operations
├── input/       # Interactive input
├── usb/         # USB devices
├── event/       # Events
├── util/        # Utilities
└── debug/       # Debug
```

## Appendix: File Formats

| Extension | Format | Use Case |
|-----------|--------|----------|
| `.L` | Plain JSON | Development |
| `.LZ` | AES encrypted | Source protection |
| `.lpack` | Encrypted package | Standalone distribution |
