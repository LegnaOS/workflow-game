# WorkflowEngine

A visual node-based game logic editor. Connect blocks instead of writing code, extend with Lua scripts.

## What is this

A tool for building game logic by dragging nodes and connecting ports.

Core idea: break down game logic into Blocks (nodes), each Block is a Lua script, Blocks pass data through connections. Use it for:

- Turn-based battle systems
- Skill/Buff calculations
- State machines
- Any logic that can be represented as data flow

## Quick Start

```bash
# Clone
git clone https://github.com/LegnaOS/workflow-game.git
cd workflow-game

# Build and run
cargo run --release

# Or download from Releases
```

After launching:
1. Left panel shows Block list, double-click to add to canvas
2. Drag ports to create connections
3. Right panel for editing Block properties
4. Ctrl+S to save, Ctrl+O to open

## File Formats

| Extension | Description |
|-----------|-------------|
| `.L` | Plain JSON, editable |
| `.LZ` | Encrypted, requires password |
| `.dist.L` | Distribution, read-only |
| `.dist.LZ` | Encrypted distribution |

## Custom Blocks

Blocks are Lua scripts. Drop them in `scripts/` directory, auto-loaded with hot reload.

Minimal example:

```lua
return {
    meta = {
        id = "my.double",
        name = "Double",
        category = "Math",
        color = "#FF5722"
    },
    inputs = {
        { id = "value", name = "Input", type = "number", default = 0 }
    },
    outputs = {
        { id = "result", name = "Result", type = "number", default = 0 }
    },
    execute = function(self, inputs)
        return { result = inputs.value * 2 }
    end
}
```

See [docs/BLOCK_DEVELOPMENT_EN.md](docs/BLOCK_DEVELOPMENT_EN.md) for details.

## Built-in Blocks

```
scripts/
├── game/          # Game
│   ├── character  # Character (stats, state)
│   ├── monster    # Monster
│   ├── attack     # Attack calculation
│   └── fireball   # Fireball skill
├── logic/         # Logic
│   ├── branch     # Conditional branch
│   ├── compare    # Comparison
│   └── selector   # Selector
├── math/          # Math
│   ├── add        # Addition
│   ├── multiply   # Multiplication
│   └── calc       # Expression
└── util/          # Utility
    ├── splitter   # Splitter
    ├── merger     # Merger
    └── switch     # Switch
```

## Building

Requires Rust 1.70+

```bash
# Development
cargo run

# Release
./build.sh all

# Single platform
./build.sh mac
./build.sh mac-intel
./build.sh windows
```

Output in `dist/` directory.

## Project Structure

```
src/
├── main.rs           # Entry, font loading
├── app.rs            # Main app logic
├── script/           # Lua engine
│   ├── loader.rs     # Encoding (UTF-8/GBK)
│   ├── registry.rs   # Block registry
│   └── executor.rs   # Executor
├── workflow/         # Core workflow
│   ├── graph.rs      # Graph structure
│   ├── block.rs      # Block definition
│   ├── connection.rs # Connections
│   └── storage.rs    # File storage
└── ui/               # UI components
    ├── canvas.rs     # Canvas
    └── block_widget.rs
```

## Tech Stack

- **Rust** - Core
- **egui/eframe** - GUI
- **mlua** - Lua 5.4 bindings
- **serde** - Serialization

## License

MIT

