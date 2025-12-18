# WorkflowEngine

[English](README_EN.md) | [Русский](README_RU.md) | 中文

可视化节点游戏逻辑编辑器 + 独立播放器。用连线代替代码，用 Lua 脚本扩展功能。

## 这是什么

一个让你通过拖拽节点、连接端口来搭建游戏逻辑的工具，以及可以独立分发游戏的播放器。

核心思路：把游戏逻辑拆成一个个 Block（节点），每个 Block 是一段 Lua 脚本，Block 之间通过连线传递数据。你可以用它来做：

- 回合制战斗系统
- 放置/挂机游戏
- 技能/Buff 计算
- 状态机
- 任何能拆成数据流的逻辑

## 截图

<img width="1403" height="863" alt="image" src="https://github.com/user-attachments/assets/7201603f-72a7-4035-b66b-c1bc7106df32" />

https://github.com/user-attachments/assets/08793b5b-d584-44a1-b641-9e8912ce3061

## 下载

从 [Releases](https://github.com/LegnaOS/workflow-game/releases) 下载：

| 文件 | 说明 |
|------|------|
| `workflow_engine-*-macos-arm64.tar.gz` | macOS Apple Silicon 版 |
| `workflow_engine-*-macos-x64.tar.gz` | macOS Intel 版 |
| `workflow_engine-*-windows-x64.zip` | Windows 64位版 |

每个压缩包包含：
- `workflow_engine` - IDE 编辑器
- `workflow_player` - 独立播放器
- `scripts/` - 预设脚本库
- `workflows/` - 示例工作流

## 快速开始

### 使用 IDE

1. 下载并解压对应平台的压缩包
2. 运行 `workflow_engine`
3. 左侧双击 Block 添加到画布
4. 拖动端口创建连线
5. 右侧面板编辑属性
6. `Ctrl+S` 保存，`Ctrl+O` 打开

### 发布游戏

1. 在 IDE 中完成工作流设计
2. 点击工具栏「📦 发布」按钮
3. 输入游戏名称，选择保存目录
4. 自动生成：
   - `游戏名_publish/` 文件夹
   - `workflow_player` 播放器
   - `游戏名.lpack` 加密游戏包

### 运行游戏

1. 将 `workflow_player` 和 `.lpack` 文件放同一目录
2. 双击 `workflow_player`
3. 多个游戏时会显示选择界面

## 文件格式

| 扩展名 | 说明 | 用途 |
|--------|------|------|
| `.L` | 明文 JSON | 开发调试 |
| `.LZ` | AES 加密 | 源码保护 |
| `.lpack` | 加密游戏包 | 独立发布（含脚本） |

## 自定义 Block

Block 就是 Lua 脚本。放到 `scripts/` 目录下自动加载，支持热重载。

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

详细文档见 [docs/BLOCK_DEVELOPMENT.md](docs/BLOCK_DEVELOPMENT.md)

## 内置脚本

```
scripts/
├── lite/          # Lite RPG 放置游戏
│   ├── hero       # 英雄
│   ├── boss       # Boss
│   ├── weapon     # 武器
│   ├── armor      # 护甲
│   ├── skill      # 技能
│   └── gem_*      # 宝石（攻击/暴击/闪避）
├── game/          # 游戏核心
│   ├── character  # 角色
│   ├── monster    # 怪物
│   ├── attack     # 攻击计算
│   └── ...
├── logic/         # 逻辑控制
│   ├── branch     # 条件分支
│   ├── compare    # 比较
│   └── selector   # 选择器
├── math/          # 数学运算
│   ├── add        # 加法
│   ├── multiply   # 乘法
│   └── calc       # 表达式
├── input/         # 交互输入
│   ├── text_input # 文本框
│   ├── password   # 密码框
│   └── button     # 按钮
└── util/          # 工具
    ├── splitter   # 分流
    ├── merger     # 合并
    └── switch     # 开关
```

## 构建

需要 Rust 1.70+

```bash
# 开发
cargo run

# 编译 IDE 和播放器
cargo build --release

# 产物
target/release/workflow_engine  # IDE
target/release/workflow_player  # 播放器
```

## 项目结构

```
src/
├── main.rs           # IDE 入口
├── player.rs         # 播放器入口
├── app.rs            # 主应用逻辑
├── script/           # Lua 脚本引擎
│   ├── parser.rs     # 脚本解析
│   ├── registry.rs   # Block 注册表
│   └── loader.rs     # 编码处理
├── workflow/         # 工作流核心
│   ├── graph.rs      # 图结构
│   ├── block.rs      # Block 定义
│   ├── connection.rs # 连线
│   ├── package.rs    # 游戏包格式
│   └── storage.rs    # 文件存储
└── ui/               # 界面组件
    ├── canvas.rs     # 画布
    ├── block_widget.rs
    └── connection_widget.rs
```

## 技术栈

- **Rust** - 核心语言
- **egui/eframe** - 即时模式 GUI
- **mlua** - Lua 5.4 绑定
- **aes/cbc** - AES-128-CBC 加密
- **serde** - 序列化

## License

MIT

