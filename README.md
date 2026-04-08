<!-- markdownlint-disable MD033 MD041 MD045 -->
<p align="center">
    <img style="height:240px;width:240px" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="OsynicMIDI"/>
</p>

<p align="center">
  <h1 align="center">🎵 OsynicMIDI</h1>
  <p align="center"><strong>一个优雅的 Rust MIDI 到键盘映射工具库</strong></p>
  <p align="center">将 MIDI 设备（电子琴、控制器等）映射到键盘按键，轻松控制任何应用程序</p>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/Rust-1.85%2B-blue?style=flat-square"/></a>
  <a href="https://crates.io/crates/osynic_midi" target="_blank"><img src="https://img.shields.io/crates/v/osynic_midi?style=flat-square"/></a>
  <a href="https://docs.rs/osynic_midi" target="_blank"><img src="https://img.shields.io/docsrs/osynic_midi?style=flat-square"/></a>
  <a href="https://github.com/osynicite/osynic_midi" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green?style=flat-square"/></a>
  <a href="https://github.com/osynicite/osynic_midi" target="_blank"><img src="https://img.shields.io/github/stars/osynicite/osynic_midi?style=flat-square"/></a>
</p>

<hr>

<p align="center">
  <a href="#-快速开始">快速开始</a> •
  <a href="#-特性">特性</a> •
  <a href="#-安装">安装</a> •
  <a href="#-文档">文档</a> •
  <a href="#-示例">示例</a> •
  <a href="README_EN.md">English</a>
</p>

---

## 📖 关于项目

**OsynicMIDI** 是一个轻量级、高效的 MIDI 到键盘映射工具，使用 Rust 编写。它允许你用任何 MIDI 设备（如电子琴、鼓垫、MIDI 控制器）控制计算机上的键盘输入。

无论你是游戏玩家、音乐制作人，还是想要创意地使用 MIDI 设备，OsynicMIDI 都能为你提供简单而强大的解决方案。

### 为什么选择 OsynicMIDI？

✅ **简单易用** - 5秒快速启动，无需复杂配置  
✅ **强大灵活** - 支持多种映射模式和自定义配置  
✅ **现代界面** - 交互式菜单，使用箭头键和回车按钮  
✅ **库 + CLI** - 既可作为独立工具，也可嵌入你的 Rust 项目  
✅ **可靠稳定** - 经过充分测试，生产就绪  
✅ **开源免费** - MIT 许可，完全开源  

---

## ⚡ 快速开始

### 1️⃣ 安装

```bash
cargo install osynic_midi
```

或从源码编译：
```bash
git clone https://github.com/osynicite/osynic_midi
cd osynic_midi
cargo build --release
```

### 2️⃣ 运行

```bash
osynic-midi start
```

### 3️⃣ 选择

用箭头键 `↑` `↓` 选择配置和设备，按 `Enter` 确认！

就这么简单！现在你可以用 MIDI 设备弹出键盘按键了 🎉

---

## ✨ 特性

### 🎹 MIDI 映射
- **两种映射模式**
  - **音符模式**：每个 MIDI 音符映射到特定按键
  - **八度模式**：按八度和音高自动映射

### 🎯 智能设备管理
- 自动发现所有连接的 MIDI 设备
- 基于设备名称的可靠连接（不依赖不稳定的索引）
- 支持多个 MIDI 设备

### ⚙️ 灵活配置
- JSON 格式的可读配置文件
- 预设配置开箱即用
- 易于创建自定义映射
- 支持力度阈值过滤

### 🖥️ 现代化 CLI
- 交互式菜单选择（类似 Vite）
- 箭头键导航，直观易用
- 支持非交互式模式用于脚本和自动化
- 完整的命令行帮助系统

### 📚 全面文档
- 完整的 API 文档
- 详细的配置指南
- 丰富的使用示例
- 架构和设计文档

---

## 📦 安装

### 使用 Cargo（推荐）

```bash
cargo install osynic_midi
```

### 从源码编译

```bash
git clone https://github.com/osynicite/osynic_midi.git
cd osynic_midi
cargo build --release
```

编译后的二进制文件位于：
- **开发版本**：`target/debug/osynic-midi.exe`
- **发布版本**：`target/release/osynic-midi.exe`

### 系统要求

- **Rust**：1.85.0 或更高版本
- **操作系统**：Windows、Linux 或 macOS
- **MIDI 设备**：至少一个 MIDI 输入设备（可选，支持虚拟 MIDI 端口）

---

## 🚀 使用指南

### 基本命令

```bash
# 快速使用
osynic-midi

# 列出所有连接的 MIDI 设备
osynic-midi list-devices

# 列出所有可用的配置文件
osynic-midi list-configs

# 启动交互式 MIDI 映射
osynic-midi start

# 创建一个新的映射配置文件（交互式）
osynic-midi create

# 从本地文件加载配置文件
osynic-midi load-local

# 使用指定配置启动
osynic-midi start -c configs/my_config.json

# 指定映射模式（notes 或 octaves）
osynic-midi start -m notes

# 同时指定配置和模式（非交互式）
osynic-midi start -c configs/my_config.json -m notes

# 从本地路径加载配置和指定模式
osynic-midi load-local -c /path/to/config.json -m notes
```

### 交互式菜单

运行 `osynic-midi start` 后，你会看到三个选择菜单：

```
? Select configuration file:
❯ midi_config.json
  midi_config_10ka.json
  midi_config_10kab.json
  ...
```

**操作方式**：
- `↑` / `↓` - 上下移动
- `Enter` - 确认选择
- `Ctrl+C` - 取消

### 工作流程

1. **检查设备**
   ```bash
   osynic-midi list-devices
   ```
   确保你的 MIDI 设备已识别

2. **选择配置方式**
   
   **方式 A：使用现有配置（推荐新手）**
   ```bash
   osynic-midi list-configs
   osynic-midi start
   ```
   
   **方式 B：创建新配置（推荐自定义）**
   ```bash
   osynic-midi create
   ```
   按照提示逐步创建你的映射配置
   
   **方式 C：加载本地配置文件**
   ```bash
   osynic-midi load-local
   ```
   指定配置文件的完整路径（可以在任何位置）

3. **启动映射**
   ```bash
   osynic-midi start
   ```
   按照交互式提示完成设置

4. **开始使用**
   现在你可以使用 MIDI 设备弹出键盘按键了！

---

## 📚 文档

完整文档位于 `docs/` 目录：

| 文档                                       | 内容                     |
| ------------------------------------------ | ------------------------ |
| [快速开始](docs/01-getting-started.md)     | 5分钟入门指南            |
| [CLI 完整指南](docs/02-cli-guide.md)       | 命令行工具的详细使用说明 |
| [库 API 文档](docs/03-library-api.md)      | Rust 库的完整 API 参考   |
| [架构设计](docs/04-architecture.md)        | 项目结构和设计说明       |
| [配置指南](docs/05-configuration-guide.md) | JSON 配置文件详细说明    |
| [更新日志](docs/06-changelog.md)           | 版本历史和更新内容       |
| [故障排除](docs/07-troubleshooting.md)     | 常见问题和解决方案       |

---

## 💡 示例

### 示例 1：快速启动
```bash
osynic-midi start -c configs/midi_config.json -m notes
```

### 示例 2：库中使用
```rust
use osynic_midi::{Config, MappingMode, start_mapping};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载配置
    let config = Config::load("configs/midi_config.json")?;
    
    // 启动 MIDI 映射
    start_mapping(
        "configs/midi_config.json".to_string(),
        "X8III".to_string(),
        MappingMode::Notes,
    ).await?;
    
    Ok(())
}
```

详见 `examples/` 目录获取更多示例。

---

## 🎯 使用场景

### 🎮 游戏玩家
- 用 MIDI 键盘控制游戏（Osu!mania、节奏游戏等）
- 创建专业游戏外设映射

### 🎵 音乐制作人
- 用 MIDI 控制器控制 DAW
- 创建自定义快捷键映射

### 💻 开发者
- 作为库集成到 Rust 项目中
- 创建专业的 MIDI 应用程序

### 🏠 爱好者
- 创意使用 MIDI 设备
- 自动化和控制应用程序

---

## 🔧 配置示例

### Osu!Mania 风格

```json
{
  "mapping_mode": "notes",
  "velocity_threshold": 1,
  "octaves": {},
  "note_mappings": {
    "36": "A", "37": "S", "38": "D", "39": "F",
    "40": "G", "41": "H", "42": "J", "43": "K"
  }
}
```

### 钢琴键盘式

```json
{
  "mapping_mode": "octaves",
  "velocity_threshold": 0,
  "octaves": {
    "3": {
      "C": "A", "C#/Db": "W", "D": "S", "D#/Eb": "E",
      "E": "D", "F": "R", "F#/Gb": "F", "G": "T",
      "G#/Ab": "G", "A": "Y", "A#/Bb": "H", "B": "U"
    }
  },
  "note_mappings": {}
}
```

更多配置见 [配置指南](docs/05-configuration-guide.md)。

---

## 🤝 贡献

欢迎贡献！无论是下列哪一种方式，我们都很感谢：

- 🐛 报告 Bug
- 💡 提出新想法
- 📝 改进文档
- 🔧 提交代码修复

请通过 GitHub Issues 或 Pull Requests 与我们联系。

---

## ❓ 常见问题

### Q: 支持哪些操作系统？
A: Windows、Linux 和 macOS 都支持。

### Q: 可以不用 MIDI 设备使用吗？
A: 可以使用虚拟 MIDI 端口（如 Windows 的 loopMIDI）进行测试。

### Q: 支持多个 MIDI 设备吗？
A: 目前一次只能使用一个设备，但可以快速切换。

### Q: 如何创建自定义配置？
A: JSON 格式非常简单。参考 [配置指南](docs/05-configuration-guide.md)。

### Q: 可以在脚本中使用吗？
A: 可以！使用 `-c` 和 `-m` 参数运行非交互式模式。

更多问题见 [故障排除指南](docs/07-troubleshooting.md)。

---

## 📊 项目状态

- ✅ 稳定版本
- ✅ 生产就绪
- ✅ 完整文档
- ✅ 活跃维护

---

## 📄 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

---

## 🙏 致谢

感谢以下项目的支持：
- [midir](https://github.com/Boddlnagg/midir) - MIDI 支持
- [enigo](https://github.com/enigo-rs/enigo) - 键盘模拟
- [tokio](https://github.com/tokio-rs/tokio) - 异步运行时
- [inquire](https://github.com/mikaelmello/inquire) - 交互式菜单

---

## 📮 联系方式

- 📧 Email: [zoneherobrine@gmail.com](mailto:zoneherobrine@gmail.com)
- 🐙 GitHub: [@osynicite](https://github.com/osynicite)
- 🌐 项目链接: [osynic_midi](https://github.com/osynicite/osynic_midi)

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/osynicite">Hako Chest</a>
  <br><br>
  <a href="README_EN.md">🌍 English Version</a>
</p>
