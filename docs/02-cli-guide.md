# CLI 使用指南 | CLI Usage Guide

## 中文版本

### 概述
`osynic-midi` 命令行工具允许你从终端控制 MIDI 到键盘的映射。支持交互式菜单和直接参数两种使用方式。

### 编译

#### 开发版本
```bash
cargo build
```
输出：`target/debug/osynic-midi.exe`

#### 发布版本（推荐用于生产环境）
```bash
cargo build --release
```
输出：`target/release/osynic-midi.exe`

### 子命令详解

#### 1. 列出 MIDI 设备
```bash
osynic-midi list-devices
```

列出系统中所有连接的 MIDI 输入设备。

**示例输出**：
```
Available MIDI input devices:
  0: X8III
  1: MIDIIN2 (X8III)
  2: loopMIDI Port
  3: loopMIDI Port 1
```

#### 2. 列出配置文件
```bash
osynic-midi list-configs
```

列出 `configs` 目录中所有可用的 MIDI 映射配置文件。

**示例输出**：
```
Available configuration files:
  0: midi_config.json
  1: midi_config_10ka.json
  2: midi_config_10kab.json
  ...
```

#### 3. 启动映射（交互式）
```bash
osynic-midi start
```

使用交互式菜单启动 MIDI 映射。将按顺序引导你：
1. 选择配置文件
2. 选择 MIDI 输入设备
3. 选择映射模式（音符或八度）

**菜单示例**：
```
? Select configuration file:
❯ midi_config.json
  midi_config_10ka.json
  midi_config_10kab.json
```

使用方法：
- `↑` 向上移动
- `↓` 向下移动
- `Enter` 确认选择
- `Ctrl+C` 中止

#### 4. 使用指定配置启动
```bash
osynic-midi start --config configs/midi_config_10ka.json
```

或简写：
```bash
osynic-midi start -c configs/midi_config_10ka.json
```

#### 5. 指定映射模式
```bash
osynic-midi start --mode notes
# 或
osynic-midi start --mode octaves
```

或简写：
```bash
osynic-midi start -m notes
osynic-midi start -m octaves
```

#### 6. 组合：配置 + 模式
```bash
osynic-midi start -c configs/midi_config.json -m notes
```

这种非交互式方式适合脚本和自动化任务。

### 映射模式说明

#### 音符模式 (Notes)
将单个 MIDI 音符映射到特定的键盘按键。基于配置文件中的 `note_mappings` 部分。

**适用场景**：当你希望为每个音符指定特定的按键时。

#### 八度模式 (Octaves)
基于八度和音高名称映射 MIDI 音符。按八度和音高（C、C#/Db、D 等）分组。

**适用场景**：当你有八度基础的键盘布局时。

### 配置文件格式

配置文件采用 JSON 格式，存储在 `configs` 目录中。

#### 示例结构：
```json
{
  "mapping_mode": "notes",
  "velocity_threshold": 0,
  "octaves": {
    "3": {
      "C": "A",
      "C#/Db": "S",
      "D": "D",
      "D#/Eb": "F",
      "E": "G",
      ...
    }
  },
  "note_mappings": {
    "0": "Key_0",
    "1": "Key_1",
    ...
    "127": "Key_127"
  }
}
```

#### 字段说明：

- **mapping_mode**：默认模式（"notes" 或 "octaves"），可由 CLI 参数覆盖
- **velocity_threshold**：MIDI 力度阈值（0-127），低于此值的按键将被忽略
- **octaves**：音符映射字典，按八度分组，再按音高分组
- **note_mappings**：音符编号（0-127）到键盘按键的直接映射

### 支持的按键名称

配置文件中支持以下按键名称：

#### 字母：
A-Z（单个大写字母）

#### 符号：
- `,` - 逗号
- `.` - 句号
- `/` - 正斜杠
- `;` - 分号
- `'` - 撇号
- `[` - 左方括号
- `]` - 右方括号
- `\` - 反斜杠
- `-` - 连字符
- `=` - 等号

#### 特殊按键：
- `Space` - 空格键
- `Left` - 左箭头键
- `Right` - 右箭头键
- `RAlt` - 右 Alt 键

### 工作流示例

#### 示例 1：快速启动（使用默认配置和音符模式）
```bash
osynic-midi start -c configs/midi_config.json -m notes
```

#### 示例 2：交互式设置
```bash
osynic-midi start
```

按照提示逐步完成配置。

#### 示例 3：启动前检查设备
```bash
osynic-midi list-devices
osynic-midi list-configs
osynic-midi start
```

### 操作期间的键盘快捷键

- **Enter**：退出 MIDI 映射应用程序

### 故障排除

#### 常见问题：

1. **找不到 MIDI 设备**
   - 安装 MIDI 设备驱动程序
   - 检查设备是否正确连接
   - 尝试重新启动应用程序

2. **配置文件无法加载**
   - 确保配置文件位于 `configs` 目录
   - 检查 JSON 格式是否有效
   - 检查文件权限

3. **键盘输入无法工作**
   - 某些应用程序可能限制键盘输入
   - 尝试以管理员权限运行
   - 检查操作系统安全设置

---

## English Version

### Overview
The `osynic-midi` command-line tool allows you to control MIDI-to-keyboard mapping from the terminal. It supports both interactive menu mode and direct parameter mode.

### Compilation

#### Debug build
```bash
cargo build
```
Output: `target/debug/osynic-midi.exe`

#### Release build (Recommended for production)
```bash
cargo build --release
```
Output: `target/release/osynic-midi.exe`

### Commands

#### 1. List MIDI Devices
```bash
osynic-midi list-devices
```

Lists all MIDI input devices currently connected to your system.

**Example output**:
```
Available MIDI input devices:
  0: X8III
  1: MIDIIN2 (X8III)
  2: loopMIDI Port
  3: loopMIDI Port 1
```

#### 2. List Configuration Files
```bash
osynic-midi list-configs
```

Displays all available MIDI mapping configuration files in the `configs` directory.

**Example output**:
```
Available configuration files:
  0: midi_config.json
  1: midi_config_10ka.json
  2: midi_config_10kab.json
  ...
```

#### 3. Start Mapping (Interactive)
```bash
osynic-midi start
```

Start MIDI mapping with interactive prompts to select device, configuration, and mapping mode.

**Menu example**:
```
? Select configuration file:
❯ midi_config.json
  midi_config_10ka.json
  midi_config_10kab.json
```

Controls:
- `↑` Move up
- `↓` Move down
- `Enter` Confirm selection
- `Ctrl+C` Abort

#### 4. Start with Specific Configuration
```bash
osynic-midi start --config configs/midi_config_10ka.json
```

Or shorthand:
```bash
osynic-midi start -c configs/midi_config_10ka.json
```

#### 5. Specify Mapping Mode
```bash
osynic-midi start --mode notes
# or
osynic-midi start --mode octaves
```

Or shorthand:
```bash
osynic-midi start -m notes
osynic-midi start -m octaves
```

#### 6. Combined: Config + Mode
```bash
osynic-midi start -c configs/midi_config.json -m notes
```

This non-interactive mode is perfect for scripts and automation.

### Mapping Modes

#### Notes Mode
Maps individual MIDI notes to specific keyboard keys based on the `note_mappings` configuration.

**Use case**: When you want specific keys for each note number.

#### Octaves Mode
Maps MIDI notes based on octave and pitch name (C, C#/Db, D, etc.).

**Use case**: When you have octave-based keyboard layouts.

### Configuration File Format

Configuration files are JSON-based and located in the `configs` directory.

#### Example structure:
```json
{
  "mapping_mode": "notes",
  "velocity_threshold": 0,
  "octaves": {
    "3": {
      "C": "A",
      "C#/Db": "S",
      "D": "D",
      ...
    }
  },
  "note_mappings": {
    "0": "Key_0",
    "1": "Key_1",
    ...
    "127": "Key_127"
  }
}
```

#### Field explanations:

- **mapping_mode**: Default mode ("notes" or "octaves"), can be overridden by CLI argument
- **velocity_threshold**: MIDI velocity threshold (0-127), keys below are ignored
- **octaves**: Note mapping dictionary structured by octave and pitch
- **note_mappings**: Direct note (0-127) to keyboard key mapping

### Supported Key Names

The following key names are supported in configuration files:

#### Letters:
A-Z (single uppercase letter)

#### Symbols:
- `,` - Comma
- `.` - Period
- `/` - Forward slash
- `;` - Semicolon
- `'` - Apostrophe
- `[` - Left bracket
- `]` - Right bracket
- `\` - Backslash
- `-` - Hyphen
- `=` - Equals

#### Special Keys:
- `Space` - Spacebar
- `Left` - Left arrow key
- `Right` - Right arrow key
- `RAlt` - Right Alt key

### Workflow Examples

#### Example 1: Quick start with default config and notes mode
```bash
osynic-midi start -c configs/midi_config.json -m notes
```

#### Example 2: Interactive setup
```bash
osynic-midi start
```

Follow the prompts for step-by-step configuration.

#### Example 3: Check devices before starting
```bash
osynic-midi list-devices
osynic-midi list-configs
osynic-midi start
```

### Keyboard Shortcuts During Operation

- **Enter**: Exit the MIDI mapping application

### Troubleshooting

#### Common Issues:

1. **No MIDI devices found**
   - Install MIDI device drivers
   - Check if devices are properly connected
   - Try restarting the application

2. **Cannot load configuration file**
   - Ensure the configuration file is in the `configs` directory
   - Verify JSON format is valid
   - Check file permissions

3. **Keyboard input doesn't work**
   - Some applications may restrict keyboard input
   - Try running with administrator privileges
   - Check operating system security settings
