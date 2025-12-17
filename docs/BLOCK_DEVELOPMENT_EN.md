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

## File Formats

| Extension | Description | Use Case |
|-----------|-------------|----------|
| `.L` | Plain JSON | Development |
| `.LZ` | AES encrypted | Tamper protection |
| `.dist.L` | Plain read-only | Distribution |
| `.dist.LZ` | Encrypted read-only | Release |

