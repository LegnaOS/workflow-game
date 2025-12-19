# WorkflowEngine

[English](README_EN.md) | [Русский](README_RU.md) | 中文

<p align="center">
  <strong>可视化节点编辑器 + 独立运行时</strong><br>
  用连线代替代码，用 Lua 扩展一切
</p>

---

## ✨ 特性

- **零代码编辑** - 拖拽节点、连接端口，所见即所得
- **Lua 脚本扩展** - 每个 Block 就是一个 Lua 脚本，热重载
- **独立发布** - 一键打包成加密游戏包，含播放器分发
- **跨平台** - macOS (ARM/Intel) + Windows
- **USB 设备支持** - 内置完整 USB 通信 API

## 📸 截图

<img width="1403" height="863" alt="image" src="https://github.com/user-attachments/assets/7201603f-72a7-4035-b66b-c1bc7106df32" />

https://github.com/user-attachments/assets/08793b5b-d584-44a1-b641-9e8912ce3061

## 📦 下载

从 [Releases](https://github.com/LegnaOS/workflow-game/releases) 获取最新版本：

| 平台 | 文件 |
|------|------|
| macOS Apple Silicon | `workflow_engine-*-macos-arm64.tar.gz` |
| macOS Intel | `workflow_engine-*-macos-x64.tar.gz` |
| Windows x64 | `workflow_engine-*-windows-x64.zip` |

**压缩包内容：**
```
├── workflow_engine    # IDE 编辑器
├── workflow_player    # 独立播放器
├── scripts/           # Block 脚本库
├── workflows/         # 示例工作流
└── docs/              # 开发文档
```

## 🚀 快速开始

### 编辑工作流

```
1. 运行 workflow_engine
2. 左侧面板双击 Block 添加到画布
3. 从端口拖出连线到另一个端口
4. 右侧面板编辑 Block 属性
5. 点击「▶ 运行」预览效果
```

**快捷键：**
| 操作 | 快捷键 |
|------|--------|
| 保存 | `Ctrl/Cmd + S` |
| 打开 | `Ctrl/Cmd + O` |
| 撤销 | `Ctrl/Cmd + Z` |
| 重做 | `Ctrl/Cmd + Shift + Z` |
| 删除 | `Delete / Backspace` |
| 框选 | 拖动空白区域 |
| 平移 | `Space + 拖动` 或 中键拖动 |
| 缩放 | 滚轮 |

### 发布游戏

```
1. 点击工具栏「📦 发布」
2. 输入游戏名称
3. 选择保存目录
```

**生成结构：**
```
游戏名_publish/
├── workflow_player    # 播放器（可独立运行）
└── 游戏名.lpack       # 加密游戏包
```

将整个文件夹分发给用户，双击 `workflow_player` 即可运行。

## 📄 文件格式

| 扩展名 | 格式 | 用途 |
|--------|------|------|
| `.L` | 明文 JSON | 开发调试，可版本控制 |
| `.LZ` | AES-128 加密 | 源码保护 |
| `.lpack` | 加密游戏包 | 独立发布（含脚本） |

## 🧩 自定义 Block

Block 是 Lua 脚本，放到 `scripts/` 目录自动加载，修改后热重载。

**最小示例：**
```lua
return {
    meta = {
        id = "my.double",
        name = "翻倍",
        category = "数学",
        color = "#FF5722"
    },
    inputs = {
        { id = "value", name = "输入", type = "number", default = 0 }
    },
    outputs = {
        { id = "result", name = "结果", type = "number", default = 0 }
    },
    execute = function(self, inputs)
        return { result = inputs.value * 2 }
    end
}
```

**详细文档：** [docs/BLOCK_DEVELOPMENT.md](docs/BLOCK_DEVELOPMENT.md)

## 📚 内置脚本库

```
scripts/
├── game/        # 游戏实体（角色、怪物、攻击）
├── lite/        # Lite RPG（英雄、Boss、装备、技能）
├── logic/       # 逻辑控制（分支、比较、选择器）
├── math/        # 数学运算（加减乘除、表达式）
├── input/       # 交互输入（文本框、按钮、密码）
├── usb/         # USB 设备（扫描、读写、控制传输）
├── event/       # 事件（启动、打印）
├── util/        # 工具（分流、合并、开关）
└── debug/       # 调试（日志）
```

## 🔧 从源码构建

**环境要求：**
- Rust 1.70+
- 跨平台编译需要对应工具链

```bash
# 开发运行
cargo run

# Release 构建
cargo build --release

# 输出
target/release/workflow_engine  # IDE
target/release/workflow_player  # 播放器
```

**多平台打包脚本：**
```bash
./build.sh  # 构建 macOS + Windows，输出到 dist/
```

## 🏗 项目结构

```
src/
├── main.rs              # IDE 入口
├── player.rs            # 播放器入口
├── app.rs               # 主应用（2000+ 行核心逻辑）
├── script/
│   ├── parser.rs        # Lua 脚本解析
│   ├── registry.rs      # Block 注册表
│   ├── executor.rs      # 执行引擎
│   └── loader.rs        # 编码检测 (UTF-8/GBK)
├── workflow/
│   ├── graph.rs         # 工作流图结构
│   ├── block.rs         # Block 定义 + 动态端口
│   ├── connection.rs    # 连线
│   ├── package.rs       # .lpack 游戏包
│   └── storage.rs       # 文件读写 + 加密
├── ui/
│   ├── canvas.rs        # 无限画布
│   ├── block_widget.rs  # Block 渲染
│   └── connection_widget.rs  # 连线渲染
└── usb/
    ├── lua_bindings.rs  # USB Lua API
    └── types.rs         # USB 类型定义
```

## 🛠 技术栈

| 组件 | 技术 |
|------|------|
| 语言 | Rust |
| GUI | egui / eframe |
| 脚本 | mlua (Lua 5.4) |
| 加密 | AES-128-CBC |
| USB | rusb / libusb |
| 序列化 | serde + serde_json |

## 📜 License

MIT

