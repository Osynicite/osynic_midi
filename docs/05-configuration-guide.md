# 配置指南 | Configuration Guide

## 中文版本

### 配置文件概述

OsynicMIDI 使用 JSON 配置文件定义 MIDI 音符到键盘按键的映射。所有配置文件应存储在项目根目录的 `configs/` 文件夹中。

### 文件格式

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
      "F": "H",
      "F#/Gb": "J",
      "G": "K",
      "G#/Ab": "L",
      "A": "Z",
      "A#/Bb": "X",
      "B": "C"
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

### 字段说明

#### 1. `mapping_mode` (必需)
默认的映射模式。可以是 `"notes"` 或 `"octaves"`。

- **`"notes"`**：按单个音符映射（使用 `note_mappings`）
- **`"octaves"`**：按八度和音高映射（使用 `octaves`）

可在运行时通过 CLI 参数 `-m` 覆盖。

```json
"mapping_mode": "notes"
```

#### 2. `velocity_threshold` (可选，默认: 0)
MIDI 力度阈值。低于此值的按键事件将被忽略。

- **范围**：0-127
- **0**：接受所有力度
- **例如 60**：只接受力度 ≥ 60 的按键

```json
"velocity_threshold": 0
```

#### 3. `octaves` (可选)
八度模式的音符映射。按八度分组，再按音高分组。

```json
"octaves": {
  "3": {
    "C": "A",
    "C#/Db": "S",
    "D": "D",
    ...
  },
  "4": {
    "C": "Q",
    ...
  }
}
```

**八度编号**：
- MIDI 音符 0-11：八度 0
- MIDI 音符 12-23：八度 1
- MIDI 音符 24-35：八度 2
- ...
- MIDI 音符 108-119：八度 9
- MIDI 音符 120-127：八度 10

**音高名称**：
```
C, C#/Db, D, D#/Eb, E, F, F#/Gb, G, G#/Ab, A, A#/Bb, B
```

#### 4. `note_mappings` (可选)
音符模式的音符到按键的直接映射。MIDI 音符号映射到键盘按键。

```json
"note_mappings": {
  "0": "Key_0",
  "1": "Key_1",
  "60": "Space",
  "61": "A",
  ...
  "127": "Key_127"
}
```

**键值**：0-127（MIDI 音符号）

### 支持的按键名称

#### 字母（A-Z）
```
"A", "B", "C", ..., "Z"
```

单个大写字母。

#### 数字（0-9）
```
"0", "1", "2", ..., "9"
```

数字键。

#### 符号
```
"," ",", ".": ".", "/": "/", ";": ";", "'": "'",
"[": "[", "]": "]", "\\": "\\", "-": "-", "=": "="
```

#### 特殊按键
```
"Space", "Left", "Right", "Tab", "RAlt"
```

### 创建你自己的配置

#### 步骤 1：新建 JSON 文件
在 `configs/` 目录中创建一个新的 `.json` 文件。例如：`configs/my_custom_config.json`

#### 步骤 2：基本结构
从基本模板开始：

```json
{
  "mapping_mode": "notes",
  "velocity_threshold": 0,
  "octaves": {},
  "note_mappings": {}
}
```

#### 步骤 3：填充映射
选择一种映射模式并填充相应的部分。

**示例 1：音符模式**
```json
{
  "mapping_mode": "notes",
  "velocity_threshold": 0,
  "octaves": {},
  "note_mappings": {
    "36": "Space",
    "37": "A",
    "38": "S",
    "39": "D",
    "40": "F"
  }
}
```

**示例 2：八度模式**
```json
{
  "mapping_mode": "octaves",
  "velocity_threshold": 0,
  "octaves": {
    "3": {
      "C": "A",
      "C#/Db": "S",
      "D": "D",
      "D#/Eb": "F",
      "E": "G"
    }
  },
  "note_mappings": {}
}
```

#### 步骤 4：验证 JSON
使用在线 JSON 验证器或你的文本编辑器来确保 JSON 格式有效。

#### 步骤 5：测试
```bash
osynic-midi list-configs
osynic-midi start -c configs/my_custom_config.json -m notes
```

### 配置最佳实践

1. **使用有意义的名称**
   - ✅ `midi_config_osu_mania.json`
   - ❌ `config1.json`

2. **注释你的配置**
   虽然 JSON 不原生支持注释，但你可以使用配置文件名和文档来说明配置的用途。

3. **备份工作配置**
   在修改配置前，创建一个备份副本。

4. **测试绑定**
   在真正使用前，用你的 MIDI 设备测试每个按键绑定。

5. **文档化力度阈值**
   如果使用力度阈值，在你的笔记中记录为什么选择了该值。

### 常见配置场景

#### 场景 1：Osu! Mania 风格映射
```json
{
  "mapping_mode": "notes",
  "velocity_threshold": 1,
  "octaves": {},
  "note_mappings": {
    "36": "A",
    "37": "S",
    "38": "D",
    "39": "F",
    "40": "G",
    "41": "H",
    "42": "J",
    "43": "K"
  }
}
```

#### 场景 2：琴键钢琴映射
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

#### 场景 3：低八度鼓垫
```json
{
  "mapping_mode": "octaves",
  "velocity_threshold": 10,
  "octaves": {
    "2": {
      "C": "Space",
      "D": "A",
      "E": "S",
      "G": "D"
    }
  },
  "note_mappings": {}
}
```

### 故障排除

#### 问题：某些按键不工作
- **检查**：JSON 中的按键名称是否正确拼写
- **检查**：音符号是否在配置中存在
- **检查**：力度是否低于阈值

#### 问题：所有按键都不工作
- 验证 JSON 格式是否有效
- 确保文件在 `configs/` 目录中
- 检查文件名（应该以 `.json` 结尾）

#### 问题：力度检测不工作
- 检查 `velocity_threshold` 是否正确设置
- 尝试设置为 0 以接受所有力度
- 检查你的 MIDI 设备是否发送力度信息

---

## English Version

### Configuration File Overview

OsynicMIDI uses JSON configuration files to define MIDI note-to-keyboard key mappings. All configuration files should be stored in the `configs/` folder in the project root.

### File Format

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
      "F": "H",
      "F#/Gb": "J",
      "G": "K",
      "G#/Ab": "L",
      "A": "Z",
      "A#/Bb": "X",
      "B": "C"
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

### Field Descriptions

#### 1. `mapping_mode` (Required)
The default mapping mode. Either `"notes"` or `"octaves"`.

- **`"notes"`**: Map individual notes (uses `note_mappings`)
- **`"octaves"`**: Map by octave and pitch (uses `octaves`)

Can be overridden at runtime with `-m` CLI flag.

```json
"mapping_mode": "notes"
```

#### 2. `velocity_threshold` (Optional, default: 0)
MIDI velocity threshold. Key events with velocity below this value are ignored.

- **Range**: 0-127
- **0**: Accept all velocities
- **Example 60**: Only accept velocities ≥ 60

```json
"velocity_threshold": 0
```

#### 3. `octaves` (Optional)
Note mappings for octave mode. Grouped by octave, then by pitch.

```json
"octaves": {
  "3": {
    "C": "A",
    "C#/Db": "S",
    "D": "D",
    ...
  },
  "4": {
    "C": "Q",
    ...
  }
}
```

**Octave numbers**:
- MIDI notes 0-11: Octave 0
- MIDI notes 12-23: Octave 1
- MIDI notes 24-35: Octave 2
- ...
- MIDI notes 108-119: Octave 9
- MIDI notes 120-127: Octave 10

**Pitch names**:
```
C, C#/Db, D, D#/Eb, E, F, F#/Gb, G, G#/Ab, A, A#/Bb, B
```

#### 4. `note_mappings` (Optional)
Direct note-to-key mapping for notes mode. MIDI note numbers map to keyboard keys.

```json
"note_mappings": {
  "0": "Key_0",
  "1": "Key_1",
  "60": "Space",
  "61": "A",
  ...
  "127": "Key_127"
}
```

**Key values**: 0-127 (MIDI note numbers)

### Supported Key Names

#### Letters (A-Z)
```
"A", "B", "C", ..., "Z"
```

Single uppercase letters.

#### Numbers (0-9)
```
"0", "1", "2", ..., "9"
```

Number keys.

#### Symbols
```
",", ".", "/", ";", "'", "[", "]", "\\", "-", "="
```

#### Special Keys
```
"Space", "Left", "Right", "Tab", "RAlt"
```

### Creating Your Own Configuration

#### Step 1: Create a New JSON File
Create a new `.json` file in the `configs/` directory. For example: `configs/my_custom_config.json`

#### Step 2: Basic Structure
Start with a basic template:

```json
{
  "mapping_mode": "notes",
  "velocity_threshold": 0,
  "octaves": {},
  "note_mappings": {}
}
```

#### Step 3: Fill in Mappings
Choose a mapping mode and populate the corresponding section.

**Example 1: Notes mode**
```json
{
  "mapping_mode": "notes",
  "velocity_threshold": 0,
  "octaves": {},
  "note_mappings": {
    "36": "Space",
    "37": "A",
    "38": "S",
    "39": "D",
    "40": "F"
  }
}
```

**Example 2: Octaves mode**
```json
{
  "mapping_mode": "octaves",
  "velocity_threshold": 0,
  "octaves": {
    "3": {
      "C": "A",
      "C#/Db": "S",
      "D": "D",
      "D#/Eb": "F",
      "E": "G"
    }
  },
  "note_mappings": {}
}
```

#### Step 4: Validate JSON
Use an online JSON validator or your text editor to ensure the JSON is valid.

#### Step 5: Test
```bash
osynic-midi list-configs
osynic-midi start -c configs/my_custom_config.json -m notes
```

### Configuration Best Practices

1. **Use Meaningful Names**
   - ✅ `midi_config_osu_mania.json`
   - ❌ `config1.json`

2. **Document Your Configuration**
   Use the filename and project documentation to explain what each configuration is for.

3. **Backup Working Configurations**
   Before modifying a configuration, create a backup copy.

4. **Test Your Bindings**
   Test each key binding with your MIDI device before actual use.

5. **Document Velocity Thresholds**
   If using velocity thresholds, note why you chose that threshold.

### Common Configuration Scenarios

#### Scenario 1: Osu! Mania Style Mapping
```json
{
  "mapping_mode": "notes",
  "velocity_threshold": 1,
  "octaves": {},
  "note_mappings": {
    "36": "A",
    "37": "S",
    "38": "D",
    "39": "F",
    "40": "G",
    "41": "H",
    "42": "J",
    "43": "K"
  }
}
```

#### Scenario 2: Piano Keyboard Mapping
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

#### Scenario 3: Low Octave Drum Pads
```json
{
  "mapping_mode": "octaves",
  "velocity_threshold": 10,
  "octaves": {
    "2": {
      "C": "Space",
      "D": "A",
      "E": "S",
      "G": "D"
    }
  },
  "note_mappings": {}
}
```

### Troubleshooting

#### Issue: Some keys don't work
- **Check**: Is the key name in JSON spelled correctly?
- **Check**: Does the note exist in the configuration?
- **Check**: Is the velocity below the threshold?

#### Issue: No keys work at all
- Verify JSON format is valid
- Ensure file is in the `configs/` directory
- Check filename (should end with `.json`)

#### Issue: Velocity detection doesn't work
- Check if `velocity_threshold` is set correctly
- Try setting it to 0 to accept all velocities
- Verify your MIDI device sends velocity information
