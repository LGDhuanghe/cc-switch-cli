# 🚀 CC-Switch CLI 快速上手指南

欢迎使用 CC-Switch CLI Edition！

---

## 📦 构建项目

```bash
cd src-tauri
cargo build --release

# 或使用 debug 模式
cargo build
```

二进制文件位置：
- Debug: `./target/debug/cc-switch`
- Release: `./target/release/cc-switch`

---

## 🎯 两种使用模式

### 1. 命令行模式（快速操作）

适合：熟悉命令行，需要快速执行操作

```bash
# Provider 管理
cc-switch provider list
cc-switch provider current
cc-switch provider switch <id>

# MCP 管理
cc-switch mcp list
cc-switch mcp sync
cc-switch mcp validate node

# Config 管理
cc-switch config path
cc-switch config validate
cc-switch config backup

# Prompts 管理
cc-switch prompts list
cc-switch prompts current
cc-switch prompts activate <id>
```

### 2. 交互式模式（美观直观）⭐ 推荐新手

适合：首次使用，喜欢可视化界面

```bash
# 启动交互式模式
cc-switch interactive

# 或指定应用
cc-switch interactive --app claude
cc-switch interactive --app codex
cc-switch interactive --app gemini
```

**交互式模式特点**：
- 🎨 美观的菜单界面
- ⌨️  方向键导航
- ✅ 实时数据展示
- 🔄 即时操作反馈
- 📋 一键查看状态

---

## 🌟 常用场景

### 场景 1: 切换 Provider

**命令行方式**：
```bash
# 1. 查看所有 providers
cc-switch provider list

# 2. 切换到指定 provider
cc-switch provider switch <id>
```

**交互式方式**：
```bash
cc-switch interactive
# 选择：🔌 Manage Providers
# 选择：🔄 Switch Provider
# 从列表选择想要的 provider
```

### 场景 2: 管理 MCP 服务器

**命令行方式**：
```bash
# 查看所有 MCP 服务器
cc-switch mcp list

# 启用服务器
cc-switch mcp enable <id> --app claude

# 同步所有服务器
cc-switch mcp sync
```

**交互式方式**：
```bash
cc-switch interactive
# 选择：🛠️  Manage MCP Servers
# 查看所有服务器状态
# 选择：🔄 Sync All Servers
```

### 场景 3: 切换 Prompt

**命令行方式**：
```bash
# 查看所有 prompts
cc-switch prompts list

# 激活 prompt
cc-switch prompts activate <id>
```

**交互式方式**：
```bash
cc-switch interactive
# 选择：💬 Manage Prompts
# 选择：🔄 Switch Active Prompt
# 从列表选择想要的 prompt
```

### 场景 4: 查看当前配置

**命令行方式**：
```bash
cc-switch provider current
cc-switch prompts current
cc-switch config validate
```

**交互式方式**：
```bash
cc-switch interactive
# 选择：👁️  View Current Configuration
# 一键查看所有配置信息
```

---

## 🎨 交互式模式导航

### 主菜单

```
═══════════════════════════════════════════════════════════
    🎯 CC-Switch Interactive Mode
═══════════════════════════════════════════════════════════
📱 Application: claude
────────────────────────────────────────────────────────────

What would you like to do? (Current: claude)
❯ 🔌 Manage Providers        ← 管理 API Providers
  🛠️  Manage MCP Servers      ← 管理 MCP 服务器
  💬 Manage Prompts           ← 管理 Prompt 预设
  👁️  View Current Config     ← 查看当前配置
  🔄 Switch Application       ← 切换应用 (Claude/Codex/Gemini)
  🚪 Exit                     ← 退出
```

### 导航技巧

- ⬆️⬇️ 方向键：上下选择
- ↵ Enter：确认选择
- Ctrl+C：退出/返回
- ⬅️  箭头：返回上级菜单

---

## 💡 实用技巧

### 1. 查看帮助

```bash
# 主帮助
cc-switch --help

# 子命令帮助
cc-switch provider --help
cc-switch mcp --help
cc-switch prompts --help
```

### 2. 指定应用

大多数命令支持 `--app` 参数：

```bash
cc-switch provider list --app claude
cc-switch mcp list --app codex
cc-switch prompts list --app gemini
```

### 3. 配置管理

```bash
# 查看配置路径
cc-switch config path

# 验证配置
cc-switch config validate

# 创建备份
cc-switch config backup

# 导出配置
cc-switch config export /path/to/backup.json

# 导入配置
cc-switch config import /path/to/backup.json
```

### 4. Shell 补全

生成 shell 补全脚本：

```bash
# Bash
cc-switch completions bash > cc-switch.bash
source cc-switch.bash

# Zsh
cc-switch completions zsh > _cc-switch
# 将文件放到 $fpath 中的目录

# Fish
cc-switch completions fish > cc-switch.fish
source cc-switch.fish
```

---

## 🔧 常见问题

### Q: 如何开始？

A: 推荐新手使用交互式模式：

```bash
cc-switch interactive
```

然后跟随菜单导航即可。

### Q: 命令行和交互式模式有什么区别？

A:
- **命令行模式**: 快速、可脚本化，适合熟练用户
- **交互式模式**: 直观、美观，适合新手或探索功能

两种模式可以随时切换使用！

### Q: 如何切换应用（Claude/Codex/Gemini）？

A:
```bash
# 命令行方式：使用 --app 参数
cc-switch provider list --app claude

# 交互式方式：主菜单选择 "🔄 Switch Application"
cc-switch interactive
```

### Q: 配置文件在哪里？

A:
```bash
cc-switch config path
# 输出：~/.cc-switch/config.json
```

### Q: 如何备份配置？

A:
```bash
# 自动备份（保留最近 10 个）
cc-switch config backup

# 手动导出
cc-switch config export ~/my-backup.json
```

---

## 📚 更多资源

### 文档

- `README.md` - 项目说明和快速开始
- `CLAUDE.md` - 开发者文档
- `CHANGELOG.md` - 版本更新记录

### 命令速查

```bash
# Provider
provider list/current/switch/delete

# MCP
mcp list/enable/disable/delete/sync/import/validate

# Config
config show/path/export/import/backup/restore/validate/reset

# Prompts
prompts list/current/activate/show/delete

# Interactive
interactive
```

---

## 🎯 推荐工作流

### 首次使用

1. **启动交互式模式**
   ```bash
   cc-switch interactive
   ```

2. **查看当前配置**
   - 选择 "👁️  View Current Configuration"
   - 了解当前状态

3. **探索功能**
   - 逐个尝试各个菜单
   - 熟悉界面布局

### 日常使用

1. **快速操作用命令行**
   ```bash
   cc-switch provider switch <id>
   cc-switch mcp sync
   ```

2. **复杂操作用交互式**
   ```bash
   cc-switch interactive
   # 导航到需要的功能
   ```

3. **定期备份**
   ```bash
   cc-switch config backup
   ```

---

## ⚡ 快捷命令

```bash
# 查看版本
cc-switch --version

# 查看当前 Provider
cc-switch provider current

# 查看当前 Prompt
cc-switch prompts current

# 验证配置
cc-switch config validate

# 同步 MCP
cc-switch mcp sync

# 交互式模式（最简单！）
cc-switch interactive
```

---

## 🎉 开始使用

**最简单的开始方式**：

```bash
# 1. 构建项目
cd src-tauri && cargo build

# 2. 启动交互式模式
./target/debug/cc-switch interactive

# 3. 享受美观直观的 CLI 体验！
```

---

生成时间：2025-11-23
版本：CC-Switch v3.7.1 CLI Edition
状态：✅ 可用

**祝使用愉快！** 🚀
