# 快速开始指南 | Quick Start Guide

## 中文版本

### 简介
OsynicMIDI 是一个强大的 MIDI 到键盘映射工具库，使用 Rust 编写。它能将 MIDI 设备输入（如电子琴、MIDI 控制器）映射到键盘按键，让你可以用 MIDI 设备控制任何支持键盘输入的应用程序。

### 安装

#### 方式一：使用 cargo install（推荐）
```bash
cargo install osynic_midi
```

#### 方式二：克隆源码并编译
```bash
git clone https://github.com/osynicite/osynic_midi
cd osynic_midi
cargo build --release
```

编译后的二进制文件位于：
- **开发版本**：`target/debug/osynic-midi.exe`
- **发布版本**：`target/release/osynic-midi.exe`（推荐使用）

### 5秒快速开始

```bash
osynic-midi start
```

然后按照提示用方向键 `↑` `↓` 选择配置和设备，按 `Enter` 确认即可！

### 常用命令

```bash
# 列出所有可用的 MIDI 设备
osynic-midi list-devices

# 列出所有可用的配置文件
osynic-midi list-configs

# 启动交互式菜单
osynic-midi start

# 直接使用指定配置和模式
osynic-midi start -c configs/midi_config.json -m notes
```

### 工作流程

1. **检查设备**
   ```bash
   osynic-midi list-devices
   ```
   确保你的 MIDI 设备已连接并识别

2. **查看配置**
   ```bash
   osynic-midi list-configs
   ```
   选择适合你的配置文件，或创建自己的配置

3. **启动映射**
   ```bash
   osynic-midi start
   ```
   按照提示完成设置

4. **开始使用**
   现在你可以用 MIDI 设备弹出键盘按键了！

---

## English Version

### Introduction
OsynicMIDI is a powerful MIDI to keyboard mapping tool library written in Rust. It maps MIDI device input (such as electronic keyboards and MIDI controllers) to keyboard keys, allowing you to control any keyboard-input-supporting application with your MIDI device.

### Installation

#### Method 1: Using cargo install (Recommended)
```bash
cargo install osynic_midi
```

#### Method 2: Clone and build from source
```bash
git clone https://github.com/osynicite/osynic_midi
cd osynic_midi
cargo build --release
```

The compiled binary will be located at:
- **Debug build**: `target/debug/osynic-midi.exe`
- **Release build**: `target/release/osynic-midi.exe` (Recommended)

### Quick Start in 5 Seconds

```bash
osynic-midi start
```

Then follow the prompts using arrow keys `↑` `↓` to select configuration and device, press `Enter` to confirm!

### Common Commands

```bash
# List all available MIDI devices
osynic-midi list-devices

# List all available configuration files
osynic-midi list-configs

# Launch interactive menu
osynic-midi start

# Start with specific config and mode
osynic-midi start -c configs/midi_config.json -m notes
```

### Workflow

1. **Check Your Device**
   ```bash
   osynic-midi list-devices
   ```
   Ensure your MIDI device is connected and recognized

2. **View Available Configurations**
   ```bash
   osynic-midi list-configs
   ```
   Select a suitable configuration or create your own

3. **Start Mapping**
   ```bash
   osynic-midi start
   ```
   Follow the interactive prompts to set up

4. **Start Using**
   Now you can play keyboard keys with your MIDI device!
