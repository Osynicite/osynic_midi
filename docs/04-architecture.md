# 架构文档 | Architecture Documentation

## 中文版本

### 项目概览

项目结构为一个 Rust 库加上 CLI 包装器，允许：
1. **直接库使用** - 在自己的 Rust 项目中使用库
2. **独立 CLI** - 使用 `osynic-midi` 命令行工具

### 目录结构

```
osynic_midi/
├── src/
│   ├── main.rs              # CLI 入口点（薄包装）
│   ├── lib.rs               # 库根（公开 API）
│   ├── core.rs              # 核心 MIDI 映射逻辑
│   ├── discovery.rs         # 配置和设备发现
│   ├── interactive.rs       # 交互式菜单选择
│   ├── mapper.rs            # MIDI 映射控制
│   ├── config.rs            # 配置处理
│   ├── error.rs             # 错误类型
│   ├── keyboard.rs          # 键盘映射
│   └── midi.rs              # MIDI 处理
├── examples/
│   ├── ajm.rs               # 原始示例（已重构）
│   └── lib_usage.rs         # 库使用示例
├── configs/                 # MIDI 配置文件
├── docs/                    # 文档目录
├── Cargo.toml               # 项目清单
└── LICENSE
```

### 模块组织

#### `lib.rs` - 库根
**目的**：为外部用户暴露公开 API

```rust
pub mod core;
pub mod discovery;
pub mod interactive;
pub mod mapper;

// 重新导出常用类型
pub use core::{Config, MappingMode, KeyboardMapper, KeyEvent};
pub use discovery::{list_configs, list_midi_devices, ...};
pub use interactive::{select_config, select_device, select_mode};
pub use mapper::start_mapping;
```

**导出内容**：
- MIDI 映射的所有核心类型
- 发现配置和设备的函数
- 菜单选择的交互函数
- 启动映射的异步函数

#### `core.rs` - 核心逻辑
**目的**：独立于 CLI 的核心 MIDI 映射功能

**类型**：
- `MappingMode` 枚举 - 音符或八度
- `Config` 结构 - JSON 配置
- `KeyEvent` 枚举 - MIDI 音符事件
- `KeyboardMapper` 结构 - MIDI 到键盘的映射

**关键特性**：
- 配置加载和验证
- 音符到音高转换
- 按键映射解析
- MIDI 事件处理

**依赖**：enigo（键盘）、serde（JSON）

#### `discovery.rs` - 发现
**目的**：查找可用的配置和设备

**函数**：
- `list_configs()` - 发现 JSON 配置文件
- `list_midi_devices()` - 枚举 MIDI 端口
- `display_configs()` - 格式化显示配置
- `display_devices()` - 格式化显示设备

**用途**：CLI 和库用户都需要

#### `interactive.rs` - 交互式选择
**目的**：用户友好的菜单选择

**函数**：
- `select_config()` - 选择配置文件
- `select_device()` - 选择 MIDI 设备
- `select_mode()` - 选择映射模式

**技术**：使用 `inquire` 库进行箭头键导航

**用途**：主要用于 CLI，也可用于库应用

#### `mapper.rs` - 映射控制
**目的**：启动并运行 MIDI 映射

**函数**：
- `start_mapping()` - 异步函数运行映射

**特性**：
- MIDI 设备连接
- 事件通道设置
- 异步事件处理
- 优雅关闭

**异步模型**：生成 tokio 任务进行事件处理

#### `main.rs` - CLI 入口点
**目的**：命令行界面

**模式**：库函数的薄包装

**流程**：
```
使用: osynic-midi <命令>
│
├─ list-devices  ─→ discovery::display_devices()
├─ list-configs  ─→ discovery::display_configs()
├─ start        ─→ start_cli_mapping()
│  │
│  ├─ interactive::select_config()
│  ├─ interactive::select_device()
│  ├─ interactive::select_mode()
│  └─ mapper::start_mapping()
│
└─ 默认          ─→ start_cli_mapping()
```

### 数据流

#### 初始化
```
加载配置 (core::Config::load)
    ↓
创建映射器 (core::KeyboardMapper::new)
    ↓
访问 Enigo（用于键盘控制）
```

#### MIDI 事件
```
MIDI 设备
    ↓
MidiInput 回调
    ↓
创建 KeyEvent (core::KeyEvent)
    ↓
发送到通道 (tokio mpsc)
    ↓
异步处理程序任务
    ↓
mapper.handle_event()
    ↓
通过 Enigo 按键按下/释放
```

#### 配置
```
discovery::list_configs()
    ↓
读取 configs/ 目录
    ↓
筛选 .json 文件
    ↓
排序并返回名称
    ↓
interactive::select_config()
    ↓
在 inquire 菜单中显示
    ↓
core::Config::load()
    ↓
解析 JSON 和验证
```

### 关注点分离

#### 库 (`lib.rs` 导出)
- **core**：MIDI 映射逻辑
- **discovery**：查找资源
- **interactive**：菜单选择
- **mapper**：控制映射

#### CLI (`main.rs`)
- 解析命令行参数（clap）
- 链接库函数
- 向用户显示结果
- 妥善处理错误

### 依赖关系

```
main.rs
  ├─ lib.rs (核心导出)
  │   ├─ core.rs
  │   ├─ discovery.rs
  │   ├─ interactive.rs
  │   │   └─ inquire (UI)
  │   └─ mapper.rs
  │       ├─ tokio (异步)
  │       ├─ midir (MIDI)
  │       └─ enigo (键盘)
  │
  └─ clap (CLI 参数解析)
```

### 扩展性

要添加新功能：

1. **新的映射模式**：扩展 `core.rs` 中的 `MappingMode`
2. **新的发现方法**：在 `discovery.rs` 中添加函数
3. **新的选择类型**：在 `interactive.rs` 中添加交互函数
4. **新的 CLI 命令**：在 `main.rs` 中添加子命令

所有更改都应遵循模块化原则，保持库和 CLI 的分离。

---

## English Version

### Project Overview

The project is structured as a Rust library with a CLI wrapper, allowing:
1. **Direct library usage** - Use the library in your own Rust projects
2. **Standalone CLI** - Use `osynic-midi` as a command-line tool

### Directory Structure

```
osynic_midi/
├── src/
│   ├── main.rs              # CLI entry point (thin wrapper)
│   ├── lib.rs               # Library root (public API)
│   ├── core.rs              # Core MIDI mapping logic
│   ├── discovery.rs         # Config & device discovery
│   ├── interactive.rs       # Interactive menu selection
│   ├── mapper.rs            # MIDI mapping control
│   ├── config.rs            # Configuration
│   ├── error.rs             # Error types
│   ├── keyboard.rs          # Keyboard mapping
│   └── midi.rs              # MIDI handling
├── examples/
│   ├── ajm.rs               # Original example (refactored)
│   └── lib_usage.rs         # Library usage example
├── configs/                 # MIDI configuration files
├── docs/                    # Documentation
├── Cargo.toml               # Project manifest
└── LICENSE
```

### Module Organization

#### `lib.rs` - Library Root
**Purpose**: Expose public API for external users

```rust
pub mod core;
pub mod discovery;
pub mod interactive;
pub mod mapper;

// Re-export commonly used types
pub use core::{Config, MappingMode, KeyboardMapper, KeyEvent};
pub use discovery::{list_configs, list_midi_devices, ...};
pub use interactive::{select_config, select_device, select_mode};
pub use mapper::start_mapping;
```

**What's exported**:
- All core types for MIDI mapping
- Discovery functions for finding configs/devices
- Interactive functions for menu selection
- Main async function for starting mapping

#### `core.rs` - Core Logic
**Purpose**: Core MIDI mapping functionality independent of CLI

**Types**:
- `MappingMode` enum - Notes or Octaves
- `Config` struct - Configuration from JSON
- `KeyEvent` enum - MIDI note events
- `KeyboardMapper` struct - Maps MIDI to keyboard

**Key features**:
- Configuration loading and validation
- Note-to-pitch conversion
- Key mapping resolution
- MIDI event handling

**Dependencies**: enigo (keyboard), serde (JSON)

#### `discovery.rs` - Discovery
**Purpose**: Find and list available options

**Functions**:
- `list_configs()` - Find JSON config files
- `list_midi_devices()` - Enumerate MIDI ports
- `display_configs()` - Pretty-print configs
- `display_devices()` - Pretty-print devices

**Use case**: Both CLI and library users need this

#### `interactive.rs` - Interactive Selection
**Purpose**: User-friendly menu selection

**Functions**:
- `select_config()` - Pick a config file
- `select_device()` - Pick a MIDI device
- `select_mode()` - Pick mapping mode

**Technology**: Uses `inquire` library for arrow-key navigation

**Use case**: CLI primarily, but can be used in library apps too

#### `mapper.rs` - Mapping Control
**Purpose**: Start and run MIDI mapping

**Functions**:
- `start_mapping()` - Async function to run mapping

**Features**:
- MIDI device connection
- Event channel setup
- Async event handler
- Graceful shutdown

**Async model**: Spawns tokio task for event processing

#### `main.rs` - CLI Entry Point
**Purpose**: Command-line interface for users

**Pattern**: Thin wrapper around library functions

**Flow**:
```
Usage: osynic-midi <COMMAND>
│
├─ list-devices  ─→ discovery::display_devices()
├─ list-configs  ─→ discovery::display_configs()
├─ start        ─→ start_cli_mapping()
│  │
│  ├─ interactive::select_config()
│  ├─ interactive::select_device()
│  ├─ interactive::select_mode()
│  └─ mapper::start_mapping()
│
└─ Default      ─→ start_cli_mapping()
```

### Data Flow

#### Initialization
```
Load Config (core::Config::load)
    ↓
Create Mapper (core::KeyboardMapper::new)
    ↓
Access Enigo (for keyboard control)
```

#### MIDI Event
```
MIDI Device
    ↓
MidiInput callback
    ↓
Create KeyEvent (core::KeyEvent)
    ↓
Send to channel (tokio mpsc)
    ↓
Async handler task
    ↓
mapper.handle_event()
    ↓
Key press/release via Enigo
```

#### Configuration
```
discovery::list_configs()
    ↓
Read configs/ directory
    ↓
Filter .json files
    ↓
Sort and return names
    ↓
interactive::select_config()
    ↓
Show in inquire menu
    ↓
core::Config::load()
    ↓
Parse JSON and validate
```

### Separation of Concerns

#### Library (`lib.rs` exports)
- **core**: MIDI mapping logic
- **discovery**: Finding resources
- **interactive**: Menu selection
- **mapper**: Control mapping

#### CLI (`main.rs`)
- Parse command-line arguments (clap)
- Chain library functions
- Display results to user
- Handle errors gracefully

### Dependencies

```
main.rs
  ├─ lib.rs (core exports)
  │   ├─ core.rs
  │   ├─ discovery.rs
  │   ├─ interactive.rs
  │   │   └─ inquire (UI)
  │   └─ mapper.rs
  │       ├─ tokio (async)
  │       ├─ midir (MIDI)
  │       └─ enigo (keyboard)
  │
  └─ clap (CLI argument parsing)
```

### Extensibility

To add new features:

1. **New mapping modes**: Extend `MappingMode` in `core.rs`
2. **New discovery methods**: Add functions in `discovery.rs`
3. **New selection types**: Add interactive functions in `interactive.rs`
4. **New CLI commands**: Add subcommands in `main.rs`

All changes should follow modular principles, maintaining separation of library and CLI.
