# WorkflowEngine

[English](README_EN.md) | [Русский](README_RU.md) | 中文

基于可视化节点的游戏逻辑编辑器。用连线代替代码，用 Lua 脚本扩展功能。

## 这是什么

一个让你通过拖拽节点、连接端口来搭建游戏逻辑的工具。

核心思路：把游戏逻辑拆成一个个 Block（节点），每个 Block 是一段 Lua 脚本，Block 之间通过连线传递数据。你可以用它来做：

- 回合制战斗系统
- 技能/Buff 计算
- 状态机
- 任何能拆成数据流的逻辑

## 截图

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│  回合控制器  │─────▶│    角色     │─────▶│    攻击     │
│             │      │  HP: 100    │      │  伤害: 25   │
│  [触发]     │      │  攻击: 20   │      │             │
└─────────────┘      └─────────────┘      └─────────────┘
                            │
                            ▼
                     ┌─────────────┐
                     │    怪物     │
                     │  HP: 50     │
                     └─────────────┘
```

## 快速开始

```bash
# 克隆
git clone https://github.com/LegnaOS/workflow-game.git
cd workflow-game

# 编译运行
cargo run --release

# 或者直接下载 Release
```

打开后：
1. 左侧是 Block 列表，双击添加到画布
2. 拖动端口创建连线
3. 右侧面板编辑 Block 属性
4. Ctrl+S 保存，Ctrl+O 打开

## 文件格式

| 扩展名 | 说明 |
|--------|------|
| `.L` | 明文 JSON，可直接编辑 |
| `.LZ` | 加密格式，需要密码 |
| `.dist.L` | 分发版，只读 |
| `.dist.LZ` | 加密分发版 |

## 自定义 Block

Block 就是 Lua 脚本。放到 `scripts/` 目录下自动加载，支持热重载。

最简示例：

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

## 内置 Block

```
scripts/
├── game/          # 游戏
│   ├── character  # 角色（属性、状态）
│   ├── monster    # 怪物
│   ├── attack     # 攻击计算
│   ├── fireball   # 火球术
│   └── ...
├── logic/         # 逻辑
│   ├── branch     # 条件分支
│   ├── compare    # 比较
│   └── selector   # 选择器
├── math/          # 数学
│   ├── add        # 加法
│   ├── multiply   # 乘法
│   └── calc       # 表达式计算
├── util/          # 工具
│   ├── splitter   # 分流器
│   ├── merger     # 合并器
│   └── switch     # 开关
└── event/         # 事件
    └── on_start   # 启动事件
```

## 构建

需要 Rust 1.70+

```bash
# 开发
cargo run

# 发布
./build.sh all

# 单平台
./build.sh mac
./build.sh mac-intel
./build.sh windows
```

输出在 `dist/` 目录。

## 项目结构

```
src/
├── main.rs           # 入口、字体加载
├── app.rs            # 主应用逻辑
├── script/           # Lua 脚本引擎
│   ├── loader.rs     # 编码处理（UTF-8/GBK）
│   ├── registry.rs   # Block 注册表
│   └── executor.rs   # 执行器
├── workflow/         # 工作流核心
│   ├── graph.rs      # 图结构
│   ├── block.rs      # Block 定义
│   ├── connection.rs # 连线
│   └── storage.rs    # 文件存储
└── ui/               # 界面组件
    ├── canvas.rs     # 画布
    └── block_widget.rs
```

## 技术栈

- **Rust** - 核心
- **egui/eframe** - GUI
- **mlua** - Lua 5.4 绑定
- **serde** - 序列化

## License

MIT

