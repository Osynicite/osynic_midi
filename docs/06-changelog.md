# 更新日志 | Changelog

## [0.1.0] - 2026-04-08

### 🎉 首次发布 | Initial Release

#### ✨ 新特性 | New Features
- **MIDI 到键盘映射** | MIDI to Keyboard Mapping
  - 支持两种映射模式：音符模式和八度模式
  - Supports two mapping modes: Notes and Octaves modes
  
- **交互式 CLI 界面** | Interactive CLI Interface
  - 使用箭头键选择（↑↓）和回车确认
  - Arrow key navigation (↑↓) with Enter confirmation
  - 现代化的用户体验，类似于 Vite 项目创建向导
  - Modern UX similar to Vite's project creation wizard
  
- **配置文件管理** | Configuration File Management
  - 自动发现 `configs/` 目录中的 JSON 配置文件
  - Auto-discovers JSON configuration files in `configs/` directory
  - 支持自定义配置和默认配置
  - Supports both custom and built-in configurations
  
- **MIDI 设备支持** | MIDI Device Support
  - 自动列出所有连接的 MIDI 输入设备
  - Auto-lists all connected MIDI input devices
  - 基于设备名称的可靠连接（而非索引）
  - Reliable device connection by name (not index)
  
- **模块化设计** | Modular Architecture
  - 清晰的库和 CLI 分离
  - Clear separation of library and CLI
  - 可作为库在其他 Rust 项目中使用
  - Can be used as a library in other Rust projects

#### 📦 依赖 | Dependencies
- `serde` & `serde_json` - JSON 序列化/反序列化
- `tokio` - 异步运行时
- `midir` - MIDI 设备支持
- `enigo` - 键盘模拟
- `inquire` - 交互式菜单UI
- `clap` - 命令行参数解析

#### 🔧 改进 | Improvements
- 修复 MIDI 设备连接问题（从索引基础改为名称基础）
- Fixed MIDI device connection by switching from index to name-based lookup
- 完善的错误处理和用户反馈
- Comprehensive error handling and user feedback
- 详细的文档和示例
- Detailed documentation and examples

#### 📚 文档 | Documentation
- `README.md` - 中文项目文档
- `README_EN.md` - 英文项目文档
- `docs/01-getting-started.md` - 快速开始指南
- `docs/02-cli-guide.md` - CLI 完整使用指南
- `docs/03-library-api.md` - 库 API 文档
- `docs/04-architecture.md` - 架构设计文档
- `docs/05-configuration-guide.md` - 配置文件指南
- `examples/lib_usage.rs` - 库使用示例

#### 🎯 系统要求 | System Requirements
- Rust 1.85.0 或更高版本
- Windows、Linux 或 macOS
- 至少一个 MIDI 输入设备

#### 🐛 已知问题 | Known Issues
无 | None

#### 📝 迁移指南 | Migration Guide
作为首次发布，无迁移指南。
As the initial release, no migration guide is applicable.

---

## 版本计划 | Planned Versions

### v0.2.0（计划中 | Planned）
- [ ] MIDI 输出支持
- [ ] 更多预设配置
- [ ] 配置验证工具
- [ ] 性能优化

### v0.3.0（计划中 | Planned）
- [ ] 图形用户界面 (GUI)
- [ ] 配置编辑器
- [ ] 配置同步功能

---

## 贡献 | Contributing

欢迎贡献！请参考 GitHub 中的贡献指南。

Contributions are welcome! Please refer to the contribution guidelines on GitHub.

---

## 许可证 | License

MIT License - 详见 LICENSE 文件
