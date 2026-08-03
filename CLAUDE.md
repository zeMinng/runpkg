# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

`runpkg` 是一个用 Rust 编写的交互式 `package.json` 控制台/项目驾驶舱。允许用户在终端中以 TUI 界面浏览、搜索和执行 npm scripts，管理依赖，以及诊断项目健康状态。对标 `npm run` / `pnpm run`，但提供更丰富的交互体验。

## 常用命令

```bash
# 构建
cargo build

# 运行（不带参数 = 启动 TUI 交互模式）
cargo run

# 运行子命令
cargo run -- scripts    # 脚本运行器
cargo run -- deps       # 依赖管理
cargo run -- doctor     # 项目健康诊断

# 测试
cargo test

# 运行单个测试
cargo test <test_name>

# Release 构建
cargo build --release
```

## 架构概览

```
src/
├── main.rs                  # 入口：解析 CLI args → 收集运行时信息 → 加载 package.json → 分发到子命令/TUI
├── cli/
│   ├── mod.rs               # 导出 Args, Commands
│   └── args.rs              # clap derive 宏定义的 CLI 参数：Args(可选的 Commands) + Commands 枚举(Scripts/Deps/Doctor)
├── project/
│   ├── mod.rs
│   ├── package_json.rs      # PackageJson 结构体(serde Deserialize)，load() 从文件系统读取并解析
│   └── info.rs              # ProjectInfo → 从 PackageJson 转换，作为内部统一表示
├── system/
│   ├── mod.rs
│   ├── runtime.rs           # RuntimeInfo 结构体，collect_runtime_info() 聚合运行时元信息
│   └── node.rs              # 通过 spawn `node -v` 检测本地 Node.js 版本
└── constants/
    ├── mod.rs
    ├── app.rs               # BANNER(ASCII Art)、APP_NAME/VERSION/DESCRIPTION/AUTHORS（来自 env!("CARGO_PKG_*")）
    ├── paths.rs             # PACKAGE_JSON 文件名、LOCK_FILES 列表
    ├── tui.rs               # TUI 帧率(FRAME_RATE: 250ms)
    └── ui.rs                # UI icon/emoji 常量
```

## 关键依赖

| Crate | 用途 |
| :--- | :--- |
| `clap` (derive) | CLI 参数解析 |
| `ratatui` + `crossterm` | TUI 终端界面 |
| `serde` + `serde_json` | `package.json` 反序列化 |
| `anyhow` | 错误处理 |
| `tokio` (full) | 异步运行时（当前尚未使用） |
| `colored` | 终端彩色输出 |

## 设计约定

- **Rust edition 2024**：使用最新版 Rust 语法和特性
- **注释语言**：代码注释使用英文，业务/功能说明可使用中文
- **`PackageJson` → `ProjectInfo` 转换模式**：原始反序列化结构体通过 `From<PackageJson>` 转换为内部使用的 `ProjectInfo`，解耦外部格式与内部逻辑
- **常量集中管理**：所有魔术字符串（文件名、路径、图标、帧率等）统一放在 `src/constants/` 下

## 当前状态（v0.1 MVP）

主循环已搭建完成骨架，当前聚焦**模块一：智能脚本运行器 (Task Hub)**：
- CLI 子命令已定义（`scripts` / `deps` / `doctor`），但各子命令的具体逻辑待实现
- TUI 入口（无参数启动）待实现
- `package.json` 加载和 Node 版本检测已可用
