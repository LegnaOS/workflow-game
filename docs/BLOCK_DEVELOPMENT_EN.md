# Block Development Guide

## Overview

A Block is the basic execution unit of the workflow engine. Each Block is a Lua script defining:

- **Metadata** - ID, name, color
- **Ports** - inputs/outputs
- **Properties** - configurable parameters
- **Execute logic** - Lua function

Drop scripts into `scripts/` directory, engine auto-scans and loads with hot reload.

## Directory Structure

```
scripts/
├── game/           # Game entities
│   ├── character.lua
│   ├── monster.lua
│   └── attack.lua
├── input/          # Interactive input
│   ├── text_input.lua
│   ├── password_input.lua
│   └── button.lua
├── logic/          # Logic control
│   ├── branch.lua
│   └── compare.lua
├── math/           # Math operations
│   ├── add.lua
│   └── calc.lua
└── util/           # Utilities
    ├── splitter.lua
    └── merger.lua
```

## Block Script Format

```lua
return {
    -- Metadata (required)
    meta = {
        id = "category.name",      -- Unique ID (required)
        name = "Display Name",     -- UI name
        category = "Category",     -- Category name
        description = "Tooltip",   -- Hover description
        color = "#4CAF50"          -- Hex color
    },

    -- Properties (editable parameters)
    properties = {
        {
            id = "prop_id",
            name = "Property",
            type = "number",       -- number/string/boolean
            default = 10,
            min = 0,
            max = 100
        }
    },

    -- Input ports
    inputs = {
        {
            id = "input_id",
            name = "Input",
            type = "number",       -- number/string/boolean/event/any
            default = 0
        }
    },

    -- Output ports
    outputs = {
        {
            id = "output_id",
            name = "Output",
            type = "number",
            default = 0
        }
    },

    -- Execute function (core logic)
    execute = function(self, inputs)
        -- self.properties: access properties
        -- self.state: persistent state
        -- inputs: input port values
        
        local result = inputs.input_id * 2
        
        return {
            output_id = result
        }
    end
}
```

## Data Types

| Type | Lua Type | Description |
|------|----------|-------------|
| `number` | number | Numeric value |
| `string` | string | Text string |
| `boolean` | boolean | True/false |
| `event` | any/nil | Event trigger (non-nil = triggered) |
| `any` | any | Any type |

## State Management

Blocks can persist state across executions via `self.state`:

```lua
execute = function(self, inputs)
    local state = self.state or {}
    state.count = (state.count or 0) + 1
    self.state = state
    return { count_out = state.count }
end
```

## Events

Events control execution flow:

```lua
inputs = {
    { id = "trigger", name = "Trigger", type = "event" }
},

execute = function(self, inputs)
    if inputs.trigger then
        return { result = 42, event_out = true }
    end
    return { result = 0, event_out = nil }
end
```

## Example: Counter Block

```lua
return {
    meta = {
        id = "util.counter",
        name = "Counter",
        category = "Utility",
        color = "#2196F3"
    },

    properties = {
        { id = "step", name = "Step", type = "number", default = 1 },
        { id = "max", name = "Max", type = "number", default = 100 }
    },

    inputs = {
        { id = "increment", name = "Increment", type = "event" },
        { id = "reset", name = "Reset", type = "event" }
    },

    outputs = {
        { id = "value", name = "Value", type = "number", default = 0 },
        { id = "overflow", name = "Overflow", type = "event" }
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

## Hot Reload

Save script, engine auto-reloads. Errors shown in console.

## Block Animation System

Blocks can implement position offset animations (attack lunge, hit recoil, etc.) by setting `self.state._animation`.

```lua
execute = function(self, inputs)
    -- Set animation: x/y is offset (pixels), speed is movement speed (pixels/sec)
    if inputs.attack_trigger then
        -- Move right 30 pixels when attacking
        self.state._animation = { x = 30, y = 0, speed = 300 }
    else
        -- Return to original position
        self.state._animation = { x = 0, y = 0, speed = 200 }
    end

    return { ... }
end
```

### Animation Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `x` | number | Horizontal offset (positive = right, negative = left) |
| `y` | number | Vertical offset (positive = down, negative = up) |
| `speed` | number | Movement speed (pixels/sec), 0 = instant |

### Example: Character Attack Animation

```lua
-- Character lunges forward when attacking
if inputs.action_trigger then
    self.state._animation = { x = 30, y = 0, speed = 300 }
else
    self.state._animation = { x = 0, y = 0, speed = 200 }
end
```

### Example: Monster Hit Animation

```lua
-- Monster recoils when hit
if inputs.attack_event then
    self.state._animation = { x = -20, y = 0, speed = 400 }
elseif is_dead then
    -- Sink when dead
    self.state._animation = { x = 0, y = 30, speed = 100 }
else
    self.state._animation = { x = 0, y = 0, speed = 200 }
end
```

## Interactive Blocks

Blocks can contain interactive widgets (input fields, buttons, etc.) by setting the `widget` property in `meta`.

### Widget Types

| widget value | Description | Use Case |
|--------------|-------------|----------|
| `textinput` | Text input field | User text input |
| `password` | Password input | Password entry (masked) |
| `textarea` | Multi-line text | Long text input |
| `checkbox` | Checkbox | Toggle options |
| `slider` | Slider | Numeric adjustment |
| `button` | Button | Trigger events |

### Extended meta Fields

| Field | Type | Description |
|-------|------|-------------|
| `widget` | string | Widget type |
| `placeholder` | string | Placeholder/hint text |
| `options` | array | Dropdown options (for dropdown type) |

### Example: Text Input Block

```lua
return {
    meta = {
        id = "input.text_input",
        name = "Text Input",
        category = "Input",
        color = "#2196F3",
        widget = "textinput",           -- Enable text input widget
        placeholder = "Enter text..."   -- Placeholder
    },

    outputs = {
        { id = "value", name = "Text Value", type = "string", default = "" },
        { id = "length", name = "Text Length", type = "number", default = 0 }
    },

    execute = function(self, inputs)
        -- Widget value auto-synced to output value port
        local text = self.state.widget_text or ""
        return {
            value = text,
            length = string.len(text)
        }
    end
}
```

### Example: Password Input Block

```lua
return {
    meta = {
        id = "input.password",
        name = "Password Input",
        color = "#FF5722",
        widget = "password",
        placeholder = "Enter password..."
    },

    properties = {
        { id = "min_length", name = "Min Length", type = "number", default = 6 }
    },

    outputs = {
        { id = "value", name = "Password", type = "string", default = "" },
        { id = "is_valid", name = "Valid", type = "boolean", default = false }
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

### Example: Button Block

```lua
return {
    meta = {
        id = "input.button",
        name = "Button",
        color = "#4CAF50",
        widget = "button",
        placeholder = "Click"
    },

    outputs = {
        { id = "clicked", name = "Clicked", type = "event" },
        { id = "click_count", name = "Click Count", type = "number", default = 0 }
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

## Layer System

Workflows support multiple layers. Each layer is an independent region of the canvas. Switching layers auto-navigates the viewport.

- **New Layer**: Click "+" in the layer panel
- **Switch Layer**: Click layer name
- **Rename**: Double-click layer name
- **Delete**: Click "×" next to layer name

Layer info is persisted in workflow files.

## File Formats

| Extension | Description | Use Case |
|-----------|-------------|----------|
| `.L` | Plain JSON | Development |
| `.LZ` | AES encrypted | Tamper protection |
| `.dist.L` | Plain read-only | Distribution |
| `.dist.LZ` | Encrypted read-only | Release |

