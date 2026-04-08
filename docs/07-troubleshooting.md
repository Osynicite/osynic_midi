# 故障排除指南 | Troubleshooting Guide

## 中文版本

### MIDI 设备相关问题

#### 问题：找不到 MIDI 设备
```bash
osynic-midi list-devices
# 输出：No MIDI input devices found!
```

**原因**：
1. 设备未连接
2. 设备未被系统识别
3. MIDI 驱动未安装

**解决方案**：
1. **检查物理连接**
   - 重新插拔 USB 线
   - 尝试不同的 USB 端口
   
2. **检查设备电源**
   - 确保 MIDI 设备已打开
   - 检查设备指示灯

3. **安装/更新驱动**
   - 访问设备制造商网站
   - 下载最新驱动程序
   - 重启操作系统

4. **使用虚拟 MIDI 端口**
   - 对于测试，可以使用 loopMIDI（Windows）或 IAC（Mac）
   - 检查 loopMIDI 是否启用

---

#### 问题：列出了错误的设备
**解决方案**：
1. 关闭所有其他 MIDI 应用程序
2. 断开并重新连接 MIDI 设备
3. 重启应用程序并重新运行 `list-devices`

---

### 配置文件问题

#### 问题：配置文件未显示在列表中
```bash
osynic-midi list-configs
# 输出：No configuration files found!
```

**原因**：
1. 文件不在 `configs/` 目录
2. 文件扩展名不是 `.json`
3. 文件权限不正确

**解决方案**：
1. **检查文件位置**
   ```
   osynic_midi/
   ├── configs/
   │   └── my_config.json  ✓ 正确位置
   │
   └── my_config.json      ✗ 错误位置
   ```

2. **检查文件扩展名**
   - 确保文件以 `.json` 结尾
   - 某些编辑器可能隐藏扩展名

3. **检查文件权限**
   ```bash
   # Linux/Mac
   chmod 644 configs/my_config.json
   ```

---

#### 问题：配置文件无法加载或解析失败
```
Error: Failed to load configuration: invalid JSON
```

**原因**：
1. JSON 格式无效
2. 缺少必需字段
3. 字段值类型错误

**解决方案**：
1. **使用 JSON 验证器**
   - https://jsonlint.com/
   - 粘贴你的配置内容验证

2. **检查常见错误**
   ```json
   // ✗ 错误：缺少逗号
   {
     "mapping_mode": "notes"
     "velocity_threshold": 0
   }
   
   // ✓ 正确
   {
     "mapping_mode": "notes",
     "velocity_threshold": 0
   }
   ```

3. **验证字段类型**
   ```json
   // ✗ 错误：velocity_threshold 应该是数字
   {
     "velocity_threshold": "0"
   }
   
   // ✓ 正确
   {
     "velocity_threshold": 0
   }
   ```

---

### 键盘输入问题

#### 问题：键盘输入完全不工作
```
✗ 按下 MIDI 键时没有反应
```

**原因**：
1. MIDI 映射未启动
2. 目标应用程序未获得焦点
3. 键盘模拟被操作系统阻止

**解决方案**：
1. **确认映射已启动**
   ```bash
   osynic-midi start
   # 应该看到 "MIDI mapping started..."
   ```

2. **确保目标应用程序有焦点**
   - 在启动映射后点击目标窗口
   - 某些应用可能需要以管理员身份运行

3. **以管理员身份运行**
   - 右键点击 `osynic-midi.exe`
   - 选择"以管理员身份运行"

4. **检查 Windows 安全软件**
   - 某些防病毒软件可能阻止键盘模拟
   - 将应用程序加入白名单

---

#### 问题：某些特定键无法工作
```
✗ 特定键总是不响应（但其他键可以）
```

**原因**：
1. 配置中缺少该键的映射
2. 输入的 MIDI 音符号不正确
3. 支持的键名拼写错误

**解决方案**：
1. **检查配置中是否存在映射**
   ```json
   "note_mappings": {
     "60": "A"  // MIDI 音符 60 映射到 'A' 键
   }
   ```

2. **验证键的名称**
   支持的键：
   - 字母：A-Z
   - 数字：0-9
   - 符号：`,`, `.`, `/`, `;`, `'`, `[`, `]`, `\`, `-`, `=`
   - 特殊键：`Space`, `Left`, `Right`, `Tab`, `RAlt`

3. **使用正确的 MIDI 音符号**
   - 中央 C = MIDI 音符 60
   - A0 = MIDI 音符 21
   - C8 = MIDI 音符 96

---

#### 问题：力度不影响触发
```
✗ 即使设置了 velocity_threshold，轻轻弹过的键仍然被触发
```

**原因**：
1. 配置中的 `velocity_threshold` 为 0
2. MIDI 设备强制设置力度
3. 配置未正确保存

**解决方案**：
1. **检查配置**
   ```json
   "velocity_threshold": 10  // 设置为非零值
   ```

2. **更新配置后重启**
   ```bash
   # 保存配置文件后
   osynic-midi start -c configs/my_config.json
   ```

3. **测试力度范围**
   - 尝试 `"velocity_threshold": 1` 看看是否有效
   - 逐步增加值直到达到所需效果

---

### 运行时问题

#### 问题：应用程序立即退出
```
✗ 启动后立即关闭，无错误消息
```

**原因**：
1. 命令参数错误
2. 必需的文件丢失
3. 配置路径不正确

**解决方案**：
1. **检查帮助文本**
   ```bash
   osynic-midi --help
   osynic-midi start --help
   ```

2. **使用绝对路径**
   ```bash
   # ✗ 可能有问题
   osynic-midi start -c configs/my_config.json
   
   # ✓ 更可靠
   osynic-midi start -c "C:\full\path\to\configs\my_config.json"
   ```

3. **验证文件存在**
   ```bash
   osynic-midi list-configs
   ```

---

#### 问题：性能问题或延迟
```
✗ 按键响应缓慢或有明显延迟
```

**原因**：
1. 系统资源不足
2. 其他应用程序占用资源
3. MIDI 设备连接问题

**解决方案**：
1. **关闭不必要的应用**
   - 关闭浏览器、大型应用等
   - 检查后台进程

2. **使用发布版本**
   ```bash
   cargo build --release
   # 发布版本速度通常更快
   ```

3. **检查 MIDI 设备连接**
   - 尝试使用不同的 USB 端口
   - 检查 USB 集线器质量

---

### 构建问题

#### 问题：编译失败
```
error[E0433]: cannot find crate in module tree
```

**原因**：
1. 依赖未安装
2. Rust 版本太低
3. 项目文件损坏

**解决方案**：
1. **清理并重建**
   ```bash
   cargo clean
   cargo build --release
   ```

2. **更新 Rust**
   ```bash
   rustup update
   ```

3. **检查依赖**
   ```bash
   cargo tree
   # 查看所有依赖项
   ```

---

#### 问题：缺少 MIDI 设备驱动库
```
error: library not found for -lcore_midi (macOS)
error: cannot find -lasound (Linux)
```

**解决方案（macOS）**：
```bash
brew install pkg-config
cargo build
```

**解决方案（Linux）**：
```bash
sudo apt-get install libasound2-dev
cargo build
```

---

### 获取帮助

#### 如果以上解决方案都不奏效：

1. **查看完整文档**
   - [完整 CLI 指南](./02-cli-guide.md)
   - [配置指南](./05-configuration-guide.md)

2. **检查示例**
   - `examples/lib_usage.rs` - 库使用示例
   - `examples/cli.rs` - CLI 使用示例

3. **提交问题**
   - 在 GitHub 提交 issue
   - 包含详细的错误信息和日志

---

## English Version

### MIDI Device Issues

#### Issue: No MIDI devices found
```bash
osynic-midi list-devices
# Output: No MIDI input devices found!
```

**Reasons**:
1. Device not connected
2. Device not recognized by system
3. MIDI drivers not installed

**Solutions**:
1. **Check Physical Connection**
   - Reconnect the USB cable
   - Try a different USB port

2. **Check Device Power**
   - Ensure the MIDI device is powered on
   - Check LED indicators

3. **Install/Update Drivers**
   - Visit device manufacturer's website
   - Download latest drivers
   - Restart operating system

4. **Use Virtual MIDI Port**
   - For testing, use loopMIDI (Windows) or IAC (Mac)
   - Ensure virtual port is enabled

---

#### Issue: Wrong device appears in list
**Solutions**:
1. Close all other MIDI applications
2. Disconnect and reconnect MIDI device
3. Restart the application and run `list-devices` again

---

### Configuration File Issues

#### Issue: Configuration file not listed
```bash
osynic-midi list-configs
# Output: No configuration files found!
```

**Reasons**:
1. File not in `configs/` directory
2. File extension is not `.json`
3. File permissions incorrect

**Solutions**:
1. **Check File Location**
   ```
   osynic_midi/
   ├── configs/
   │   └── my_config.json  ✓ Correct
   │
   └── my_config.json      ✗ Wrong
   ```

2. **Check File Extension**
   - Ensure file ends with `.json`
   - Some editors may hide extensions

3. **Check File Permissions**
   ```bash
   # Linux/Mac
   chmod 644 configs/my_config.json
   ```

---

#### Issue: Configuration file fails to load
```
Error: Failed to load configuration: invalid JSON
```

**Reasons**:
1. Invalid JSON format
2. Missing required fields
3. Wrong field value types

**Solutions**:
1. **Use JSON Validator**
   - https://jsonlint.com/
   - Paste your configuration to validate

2. **Check Common Errors**
   ```json
   // ✗ Wrong: Missing comma
   {
     "mapping_mode": "notes"
     "velocity_threshold": 0
   }
   
   // ✓ Correct
   {
     "mapping_mode": "notes",
     "velocity_threshold": 0
   }
   ```

3. **Verify Field Types**
   ```json
   // ✗ Wrong: velocity_threshold should be number
   {
     "velocity_threshold": "0"
   }
   
   // ✓ Correct
   {
     "velocity_threshold": 0
   }
   ```

---

### Keyboard Input Issues

#### Issue: Keyboard input doesn't work at all
```
✗ No response when pressing MIDI keys
```

**Reasons**:
1. MIDI mapping not started
2. Target application not in focus
3. Keyboard simulation blocked by OS

**Solutions**:
1. **Confirm Mapping is Running**
   ```bash
   osynic-midi start
   # Should see "MIDI mapping started..."
   ```

2. **Ensure Target Application in Focus**
   - Click target window after starting mapping
   - Some apps may need to run as admin

3. **Run as Administrator**
   - Right-click `osynic-midi.exe`
   - Select "Run as Administrator"

4. **Check Security Software**
   - Some antivirus may block keyboard simulation
   - Whitelist the application

---

#### Issue: Specific keys don't work
```
✗ Specific key always unresponsive (but others work)
```

**Reasons**:
1. No mapping for that key in config
2. Wrong MIDI note number
3. Key name spelling error

**Solutions**:
1. **Check Configuration**
   ```json
   "note_mappings": {
     "60": "A"  // MIDI note 60 maps to 'A'
   }
   ```

2. **Verify Key Names**
   Supported keys:
   - Letters: A-Z
   - Numbers: 0-9
   - Symbols: `,`, `.`, `/`, `;`, `'`, `[`, `]`, `\`, `-`, `=`
   - Special: `Space`, `Left`, `Right`, `Tab`, `RAlt`

3. **Use Correct MIDI Note Numbers**
   - Middle C = MIDI note 60
   - A0 = MIDI note 21
   - C8 = MIDI note 96

---

#### Issue: Velocity threshold not working
```
✗ Light key presses still trigger even with velocity_threshold set
```

**Reasons**:
1. `velocity_threshold` is 0 in config
2. MIDI device enforces velocity
3. Config not saved properly

**Solutions**:
1. **Check Configuration**
   ```json
   "velocity_threshold": 10  // Set to non-zero
   ```

2. **Restart After Saving**
   ```bash
   # After saving config file
   osynic-midi start -c configs/my_config.json
   ```

3. **Test Velocity Range**
   - Try `"velocity_threshold": 1` to see if it works
   - Gradually increase until desired effect

---

### Runtime Issues

#### Issue: Application exits immediately
```
✗ Closes right after startup with no error
```

**Reasons**:
1. Wrong command arguments
2. Missing required files
3. Incorrect config path

**Solutions**:
1. **Check Help Text**
   ```bash
   osynic-midi --help
   osynic-midi start --help
   ```

2. **Use Absolute Paths**
   ```bash
   # ✗ May have issues
   osynic-midi start -c configs/my_config.json
   
   # ✓ More reliable
   osynic-midi start -c "C:\full\path\to\configs\my_config.json"
   ```

3. **Verify Files Exist**
   ```bash
   osynic-midi list-configs
   ```

---

#### Issue: Performance lag or latency
```
✗ Slow key response or noticeable delay
```

**Reasons**:
1. Low system resources
2. Other apps consuming resources
3. MIDI device connection issue

**Solutions**:
1. **Close Unnecessary Applications**
   - Close browsers, large apps
   - Check background processes

2. **Use Release Build**
   ```bash
   cargo build --release
   # Release build is usually much faster
   ```

3. **Check MIDI Device Connection**
   - Try different USB port
   - Check USB hub quality

---

### Build Issues

#### Issue: Compilation fails
```
error[E0433]: cannot find crate in module tree
```

**Reasons**:
1. Dependencies not installed
2. Rust version too old
3. Project files corrupted

**Solutions**:
1. **Clean and Rebuild**
   ```bash
   cargo clean
   cargo build --release
   ```

2. **Update Rust**
   ```bash
   rustup update
   ```

3. **Check Dependencies**
   ```bash
   cargo tree
   # View all dependencies
   ```

---

#### Issue: Missing MIDI device library
```
error: library not found for -lcore_midi (macOS)
error: cannot find -lasound (Linux)
```

**Solution (macOS)**:
```bash
brew install pkg-config
cargo build
```

**Solution (Linux)**:
```bash
sudo apt-get install libasound2-dev
cargo build
```

---

### Getting Help

#### If none of the above work:

1. **Check Full Documentation**
   - [Complete CLI Guide](./02-cli-guide.md)
   - [Configuration Guide](./05-configuration-guide.md)

2. **Check Examples**
   - `examples/lib_usage.rs` - Library usage
   - `examples/cli.rs` - CLI usage

3. **Submit an Issue**
   - File an issue on GitHub
   - Include detailed error messages and logs
