# 库 API 文档 | Library API Documentation

## 中文版本

### 概述

`osynic_midi` Crate 是一个模块化的 MIDI 到键盘映射库，可以作为：
1. **CLI 工具** - `osynic-midi` 命令行应用
2. **库** - 嵌入其他 Rust 应用中进行编程式 MIDI 映射

### 架构

库组织为以下模块：

### 1. `core` - 核心 MIDI 映射逻辑

核心数据结构和键盘映射引擎。

#### 类型

```rust
pub enum MappingMode {
    Octaves,  // 按八度和音高映射
    Notes,    // 按单个音符映射
}

pub struct Config {
    pub mapping_mode: Option<MappingMode>,
    pub octaves: HashMap<String, HashMap<String, String>>,
    pub velocity_threshold: u8,
    pub note_mappings: HashMap<u8, String>,
}

pub enum KeyEvent {
    NoteOn(u8, u8),  // (音符, 力度)
    NoteOff(u8),     // 音符
}

pub struct KeyboardMapper {
    pub config: Config,
    pub enigo: Arc<Mutex<Enigo>>,
    pub mode: MappingMode,
}
```

#### 函数

```rust
impl Config {
    /// 从 JSON 文件加载配置
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>>

    /// 将 MIDI 音符编号转换为八度和音高
    pub fn note_to_pitch(note: u8) -> (u8, String)

    /// 获取 MIDI 音符对应的键盘按键
    pub fn get_key_for_note(&self, note: u8, mode: &MappingMode) -> Option<Key>
}

impl KeyboardMapper {
    /// 使用配置创建新的映射器
    pub fn new(config: Config, enigo: Arc<Mutex<Enigo>>, mode: MappingMode) -> Self

    /// 处理 MIDI 按键事件
    pub fn handle_event(&self, event: KeyEvent) -> Result<(), Box<dyn Error>>
}
```

### 2. `discovery` - 配置和设备发现

查找可用 MIDI 配置和设备的函数。

#### 函数

```rust
/// 列出 'configs' 目录中的所有配置文件
pub fn list_configs() -> Result<Vec<String>, Box<dyn Error>>

/// 列出所有可用的 MIDI 输入设备
pub fn list_midi_devices() -> Result<Vec<String>, Box<dyn Error>>

/// 显示格式化的配置列表
pub fn display_configs() -> Result<(), Box<dyn Error>>

/// 显示格式化的 MIDI 设备列表
pub fn display_devices() -> Result<(), Box<dyn Error>>
```

### 3. `interactive` - 交互式菜单选择

使用 `inquire` 库的交互式提示。

#### 函数

```rust
/// 提示用户选择配置文件
pub fn select_config() -> Result<String, Box<dyn Error>>

/// 提示用户选择 MIDI 输入设备（返回设备名称）
pub fn select_device() -> Result<String, Box<dyn Error>>

/// 提示用户选择映射模式
pub fn select_mode() -> Result<MappingMode, Box<dyn Error>>
```

### 4. `mapper` - MIDI 映射控制

启动映射过程的主要函数。

#### 函数

```rust
/// 启动 MIDI 到键盘的映射
pub async fn start_mapping(
    config_path: String,
    device_name: String,
    mode: MappingMode,
) -> Result<(), Box<dyn Error>>
```

## 使用示例

### 示例 1：基本配置加载

```rust
use osynic_midi::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load("configs/midi_config.json")?;
    println!("速度阈值: {}", config.velocity_threshold);
    println!("音符映射数: {}", config.note_mappings.len());
    Ok(())
}
```

### 示例 2：资源发现

```rust
use osynic_midi::{list_configs, list_midi_devices};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 列出可用配置
    let configs = list_configs()?;
    println!("找到 {} 个配置文件", configs.len());

    // 列出可用 MIDI 设备
    let devices = list_midi_devices()?;
    println!("找到 {} 个 MIDI 设备", devices.len());

    Ok(())
}
```

### 示例 3：交互式选择

```rust
use osynic_midi::interactive::{select_config, select_device, select_mode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 让用户选择配置
    let config_path = select_config()?;
    
    // 让用户选择设备
    let device_name = select_device()?;
    
    // 让用户选择模式
    let mode = select_mode()?;

    println!("已选择: {} 在设备 {}", config_path, device_name);
    Ok(())
}
```

### 示例 4：编程式 MIDI 映射

```rust
use osynic_midi::{
    Config, MappingMode, KeyboardMapper, KeyEvent,
};
use enigo::{Enigo, Settings};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = Config::load("configs/midi_config.json")?;
    
    // 创建映射器
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));
    let mapper = KeyboardMapper::new(
        config,
        enigo,
        MappingMode::Notes,
    );

    // 处理 MIDI 音符
    let event = KeyEvent::NoteOn(60, 100);  // 中央 C，力度 100
    mapper.handle_event(event)?;
    
    Ok(())
}
```

### 示例 5：完整映射流程

```rust
use osynic_midi::start_mapping;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 启动 MIDI 映射
    start_mapping(
        "configs/midi_config.json".to_string(),
        "X8III".to_string(),
        MappingMode::Notes,
    ).await?;

    Ok(())
}
```

---

## English Version

### Overview

The `osynic_midi` crate is a modular MIDI to keyboard mapping library that can be used as:
1. **A CLI tool** - `osynic-midi` command-line application
2. **A library** - Embedded in other Rust applications for programmatic MIDI mapping

### Architecture

The library is organized into the following modules:

### 1. `core` - Core MIDI Mapping Logic

Core data structures and the keyboard mapping engine.

#### Types

```rust
pub enum MappingMode {
    Octaves,  // Maps by octave and pitch
    Notes,    // Maps individual notes
}

pub struct Config {
    pub mapping_mode: Option<MappingMode>,
    pub octaves: HashMap<String, HashMap<String, String>>,
    pub velocity_threshold: u8,
    pub note_mappings: HashMap<u8, String>,
}

pub enum KeyEvent {
    NoteOn(u8, u8),  // (note, velocity)
    NoteOff(u8),     // note
}

pub struct KeyboardMapper {
    pub config: Config,
    pub enigo: Arc<Mutex<Enigo>>,
    pub mode: MappingMode,
}
```

#### Functions

```rust
impl Config {
    /// Load configuration from JSON file
    pub fn load(path: &str) -> Result<Self, Box<dyn Error>>

    /// Convert MIDI note number to octave and pitch
    pub fn note_to_pitch(note: u8) -> (u8, String)

    /// Get keyboard key for a MIDI note
    pub fn get_key_for_note(&self, note: u8, mode: &MappingMode) -> Option<Key>
}

impl KeyboardMapper {
    /// Create a new mapper with configuration
    pub fn new(config: Config, enigo: Arc<Mutex<Enigo>>, mode: MappingMode) -> Self

    /// Handle a MIDI key event
    pub fn handle_event(&self, event: KeyEvent) -> Result<(), Box<dyn Error>>
}
```

### 2. `discovery` - Configuration and Device Discovery

Functions for finding available MIDI configurations and devices.

#### Functions

```rust
/// List all configuration files in the 'configs' directory
pub fn list_configs() -> Result<Vec<String>, Box<dyn Error>>

/// List all available MIDI input devices
pub fn list_midi_devices() -> Result<Vec<String>, Box<dyn Error>>

/// Display formatted list of available configurations
pub fn display_configs() -> Result<(), Box<dyn Error>>

/// Display formatted list of available MIDI devices
pub fn display_devices() -> Result<(), Box<dyn Error>>
```

### 3. `interactive` - Interactive Menu Selection

Interactive prompts using the `inquire` library.

#### Functions

```rust
/// Prompt user to select a configuration file
pub fn select_config() -> Result<String, Box<dyn Error>>

/// Prompt user to select a MIDI input device (returns device name)
pub fn select_device() -> Result<String, Box<dyn Error>>

/// Prompt user to select mapping mode
pub fn select_mode() -> Result<MappingMode, Box<dyn Error>>
```

### 4. `mapper` - MIDI Mapping Control

Main MIDI mapping function for starting the mapping process.

#### Functions

```rust
/// Start MIDI to keyboard mapping
pub async fn start_mapping(
    config_path: String,
    device_name: String,
    mode: MappingMode,
) -> Result<(), Box<dyn Error>>
```

## Usage Examples

### Example 1: Basic Configuration Loading

```rust
use osynic_midi::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load("configs/midi_config.json")?;
    println!("Velocity threshold: {}", config.velocity_threshold);
    println!("Note mappings: {}", config.note_mappings.len());
    Ok(())
}
```

### Example 2: Discovering Resources

```rust
use osynic_midi::{list_configs, list_midi_devices};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // List available configurations
    let configs = list_configs()?;
    println!("Found {} configuration files", configs.len());

    // List available MIDI devices
    let devices = list_midi_devices()?;
    println!("Found {} MIDI devices", devices.len());

    Ok(())
}
```

### Example 3: Interactive Selection

```rust
use osynic_midi::interactive::{select_config, select_device, select_mode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let user select configuration
    let config_path = select_config()?;
    
    // Let user select device
    let device_name = select_device()?;
    
    // Let user select mode
    let mode = select_mode()?;

    println!("Selected: {} on device {}", config_path, device_name);
    Ok(())
}
```

### Example 4: Programmatic MIDI Mapping

```rust
use osynic_midi::{
    Config, MappingMode, KeyboardMapper, KeyEvent,
};
use enigo::{Enigo, Settings};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::load("configs/midi_config.json")?;
    
    // Create mapper
    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default())?));
    let mapper = KeyboardMapper::new(
        config,
        enigo,
        MappingMode::Notes,
    );

    // Handle a MIDI note
    let event = KeyEvent::NoteOn(60, 100);  // Middle C, velocity 100
    mapper.handle_event(event)?;
    
    Ok(())
}
```

### Example 5: Complete Mapping Workflow

```rust
use osynic_midi::start_mapping;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start MIDI mapping
    start_mapping(
        "configs/midi_config.json".to_string(),
        "X8III".to_string(),
        MappingMode::Notes,
    ).await?;

    Ok(())
}
```
