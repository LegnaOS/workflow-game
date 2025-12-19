# WorkflowEngine

[中文](README.md) | [Русский](README_RU.md) | English

<p align="center">
  <strong>Visual Node Editor + Standalone Runtime</strong><br>
  Connect blocks instead of code, extend everything with Lua
</p>

---

## ✨ Features

- **Zero-code editing** - Drag nodes, connect ports, WYSIWYG
- **Lua script extension** - Each Block is a Lua script, hot-reloadable
- **Standalone publishing** - One-click export to encrypted game package with player
- **Cross-platform** - macOS (ARM/Intel) + Windows
- **USB device support** - Built-in complete USB communication API

## 📸 Screenshot

<img width="1403" height="863" alt="image" src="https://github.com/user-attachments/assets/7201603f-72a7-4035-b66b-c1bc7106df32" />

https://github.com/user-attachments/assets/08793b5b-d584-44a1-b641-9e8912ce3061

## 📦 Download

Get the latest version from [Releases](https://github.com/LegnaOS/workflow-game/releases):

| Platform | File |
|----------|------|
| macOS Apple Silicon | `workflow_engine-*-macos-arm64.tar.gz` |
| macOS Intel | `workflow_engine-*-macos-x64.tar.gz` |
| Windows x64 | `workflow_engine-*-windows-x64.zip` |

**Package contents:**
```
├── workflow_engine    # IDE editor
├── workflow_player    # Standalone player
├── scripts/           # Block script library
├── workflows/         # Example workflows
└── docs/              # Development docs
```

## 🚀 Quick Start

### Editing Workflows

```
1. Run workflow_engine
2. Double-click Blocks in left panel to add to canvas
3. Drag from port to another port to connect
4. Edit Block properties in right panel
5. Click "▶ Run" to preview
```

**Shortcuts:**
| Action | Shortcut |
|--------|----------|
| Save | `Ctrl/Cmd + S` |
| Open | `Ctrl/Cmd + O` |
| Undo | `Ctrl/Cmd + Z` |
| Redo | `Ctrl/Cmd + Shift + Z` |
| Delete | `Delete / Backspace` |
| Box select | Drag on empty area |
| Pan | `Space + drag` or middle-click drag |
| Zoom | Scroll wheel |

### Publishing Games

```
1. Click "📦 Publish" in toolbar
2. Enter game name
3. Choose save directory
```

**Output structure:**
```
GameName_publish/
├── workflow_player    # Player (standalone executable)
└── GameName.lpack     # Encrypted game package
```

Distribute the entire folder. Users double-click `workflow_player` to run.

## 📄 File Formats

| Extension | Format | Use Case |
|-----------|--------|----------|
| `.L` | Plain JSON | Development, version control |
| `.LZ` | AES-128 encrypted | Source protection |
| `.lpack` | Encrypted package | Standalone distribution (includes scripts) |

## 🧩 Custom Blocks

Blocks are Lua scripts. Drop into `scripts/` directory, auto-loaded with hot reload.

**Minimal example:**
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

**Full documentation:** [docs/BLOCK_DEVELOPMENT_EN.md](docs/BLOCK_DEVELOPMENT_EN.md)

## 📚 Built-in Script Library

```
scripts/
├── game/        # Game entities (character, monster, attack)
├── lite/        # Lite RPG (hero, boss, equipment, skills)
├── logic/       # Logic control (branch, compare, selector)
├── math/        # Math operations (add, multiply, expression)
├── input/       # Interactive input (textbox, button, password)
├── usb/         # USB devices (scan, read/write, control transfer)
├── event/       # Events (start, print)
├── util/        # Utilities (splitter, merger, switch)
└── debug/       # Debug (logger)
```

## 🔧 Building from Source

**Requirements:**
- Rust 1.70+
- Cross-compilation requires appropriate toolchains

```bash
# Development run
cargo run

# Release build
cargo build --release

# Output
target/release/workflow_engine  # IDE
target/release/workflow_player  # Player
```

**Multi-platform build script:**
```bash
./build.sh  # Builds macOS + Windows, outputs to dist/
```

## 🏗 Project Structure

```
src/
├── main.rs              # IDE entry
├── player.rs            # Player entry
├── app.rs               # Main app (2000+ lines core logic)
├── script/
│   ├── parser.rs        # Lua script parsing
│   ├── registry.rs      # Block registry
│   ├── executor.rs      # Execution engine
│   └── loader.rs        # Encoding detection (UTF-8/GBK)
├── workflow/
│   ├── graph.rs         # Workflow graph structure
│   ├── block.rs         # Block definition + dynamic ports
│   ├── connection.rs    # Connections
│   ├── package.rs       # .lpack game package
│   └── storage.rs       # File I/O + encryption
├── ui/
│   ├── canvas.rs        # Infinite canvas
│   ├── block_widget.rs  # Block rendering
│   └── connection_widget.rs  # Connection rendering
└── usb/
    ├── lua_bindings.rs  # USB Lua API
    └── types.rs         # USB type definitions
```

## 🛠 Tech Stack

| Component | Technology |
|-----------|------------|
| Language | Rust |
| GUI | egui / eframe |
| Scripting | mlua (Lua 5.4) |
| Encryption | AES-128-CBC |
| USB | rusb / libusb |
| Serialization | serde + serde_json |

## 📜 License

MIT
