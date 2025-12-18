# WorkflowEngine

Visual node-based game logic editor + standalone player. Connect blocks instead of writing code, extend with Lua scripts.

## What is this

A tool for building game logic by dragging nodes and connecting ports, plus a standalone player for distributing games.

Core idea: break down game logic into Blocks (nodes), each Block is a Lua script, Blocks pass data through connections. Use it for:

- Turn-based battle systems
- Idle/clicker games
- Skill/Buff calculations
- State machines
- Any logic that can be represented as data flow

## Screenshot

<img width="1403" height="863" alt="image" src="https://github.com/user-attachments/assets/7201603f-72a7-4035-b66b-c1bc7106df32" />

https://github.com/user-attachments/assets/08793b5b-d584-44a1-b641-9e8912ce3061

## Download

Get from [Releases](https://github.com/LegnaOS/workflow-game/releases):

| File | Description |
|------|-------------|
| `workflow_engine-*-macos-arm64.tar.gz` | macOS Apple Silicon |
| `workflow_engine-*-macos-x64.tar.gz` | macOS Intel |
| `workflow_engine-*-windows-x64.zip` | Windows 64-bit |

Each package contains:
- `workflow_engine` - IDE editor
- `workflow_player` - Standalone player
- `scripts/` - Preset scripts
- `workflows/` - Example workflows

## Quick Start

### Using IDE

1. Download and extract package for your platform
2. Run `workflow_engine`
3. Double-click blocks in left panel to add
4. Drag ports to create connections
5. Edit properties in right panel
6. `Ctrl+S` save, `Ctrl+O` open

### Publish Game

1. Design your workflow in IDE
2. Click "📦 Publish" in toolbar
3. Enter game name and choose directory
4. Auto-generates:
   - `GameName_publish/` folder
   - `workflow_player` player
   - `GameName.lpack` encrypted game package

### Run Game

1. Put `workflow_player` and `.lpack` in same directory
2. Double-click `workflow_player`
3. Shows selection UI when multiple games present

## File Formats

| Extension | Description | Use Case |
|-----------|-------------|----------|
| `.L` | Plain JSON | Development |
| `.LZ` | AES encrypted | Source protection |
| `.lpack` | Encrypted package | Standalone distribution |

## Custom Blocks

Blocks are Lua scripts. Drop in `scripts/` directory, auto-loaded with hot reload.

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

## Built-in Scripts

```
scripts/
├── lite/          # Lite RPG Idle Game
│   ├── hero       # Hero
│   ├── boss       # Boss
│   ├── weapon     # Weapon
│   ├── armor      # Armor
│   ├── skill      # Skill
│   └── gem_*      # Gems (attack/crit/dodge)
├── game/          # Game Core
│   ├── character  # Character
│   ├── monster    # Monster
│   ├── attack     # Attack calculation
│   └── ...
├── logic/         # Logic Control
│   ├── branch     # Conditional branch
│   ├── compare    # Comparison
│   └── selector   # Selector
├── math/          # Math
│   ├── add        # Addition
│   ├── multiply   # Multiplication
│   └── calc       # Expression
├── input/         # Interactive Input
│   ├── text_input # Text box
│   ├── password   # Password box
│   └── button     # Button
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

# Build IDE and Player
cargo build --release

# Output
target/release/workflow_engine  # IDE
target/release/workflow_player  # Player
```

## Project Structure

```
src/
├── main.rs           # IDE entry
├── player.rs         # Player entry
├── app.rs            # Main app logic
├── script/           # Lua engine
│   ├── parser.rs     # Script parsing
│   ├── registry.rs   # Block registry
│   └── loader.rs     # Encoding handling
├── workflow/         # Core workflow
│   ├── graph.rs      # Graph structure
│   ├── block.rs      # Block definition
│   ├── connection.rs # Connections
│   ├── package.rs    # Game package format
│   └── storage.rs    # File storage
└── ui/               # UI components
    ├── canvas.rs     # Canvas
    ├── block_widget.rs
    └── connection_widget.rs
```

## Tech Stack

- **Rust** - Core language
- **egui/eframe** - Immediate mode GUI
- **mlua** - Lua 5.4 bindings
- **aes/cbc** - AES-128-CBC encryption
- **serde** - Serialization

## License

MIT

