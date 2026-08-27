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

# 运行子命令（scripts / deps / doctor）
cargo run -- scripts

# 测试
cargo test

# 运行单个测试
cargo test <test_name>

# Release 构建
cargo build --release
```

## 入口与分发

`main.rs` 只是薄壳：解析 CLI 参数后调用库 crate 的入口 `runpkg::run(args)`（`src/lib.rs`）。真正逻辑都在库 crate 内，因此代码中模块之间的引用以 `crate::...` 为根，而非 `bin` 路径。

`lib.rs::run` 按是否有子命令分发：

- `Some(command)` → `run_cli`：当前 `scripts` / `deps` / `doctor` 三个子命令**仅是占位桩**，只打印运行时信息与解析出的 `ProjectInfo`，尚未实现真正的 CLI 逻辑。
- `None` → `run_tui`：并发加载运行时信息与项目数据（`tokio::join!`），构造 `App` 后进入 TUI 事件循环。

## 核心状态模型（App / Action / State）

这是整个应用最关键的跨文件模式，位于 `src/app/`：

- `Action`（`action.rs`）：输入事件的纯枚举（导航、确认、切屏、`RunScript(String)` 等）。
- `AppState`（`state.rs`）：`{ project: Option<ProjectInfo>, runtime: Option<RuntimeInfo> }` 及 `Screen` / `Focus` / `OutputState` 等 UI 状态。
- `App`（`app.rs`）：持有全部可变状态，`App::update(&mut self, action: Action)` 是**唯一的**状态变更入口，采用类似 Elm 的单向数据流。

TUI 事件循环（`tui/mod.rs::run`）遵循固定节奏：每帧先 `start_pending_script()` / `drain_script_output()`，再绘制，再 `next_event()` 取一个按键 → `map_key` 转成 `Action` → `app.update(action)`。**不要**在渲染函数里改状态，渲染只读 `&App`。

`App` 内部有 `receiver: Option<UnboundedReceiver<ScriptEvent>>` 与 `pending_script: Option<String>` 两个中转字段：`Action::RunScript` 只设置 `pending_script`，真正的进程 spawn 推迟到下一帧循环开头执行，保证不阻塞事件循环。

## TUI 渲染分层

- `tui/components/`：可复用的「零件」widget（header / sidebar / script_list / dependency_list / output / status_bar），每个 `render(frame, area, ...)` 只做纯渲染。
- `tui/screens/`：每个 `Screen` 对应一个画面（dashboard / scripts / dependencies / doctor），负责布局拆分并**编排** components。`screens/mod.rs::render_content` 按 `app.screen` 分发。
- 全局骨架在 `tui/mod.rs::render`：纵向 3 段（header 高 3 / body / status_bar 高 2），body 横向 2 栏（sidebar 宽 24 / content）。
- `tui/theme.rs`：集中管理调色板与所有 `Style`，不要在各组件里硬编码颜色。

键盘绑定集中在 `tui/mod.rs::map_key`：`q` 退出、`Esc/←` 返回菜单、`↑↓`(或 `jk`) 导航、`→/Tab/Enter` 进入、`1-4` 跳屏、`Enter` 在 Scripts 列表内运行脚本、`r` 刷新。

## 脚本执行管线

从「按 Enter」到「输出显示」跨 `app/` 与 `system/` 两层，按序：

1. `App::confirm()` 命中 `Focus::Content` + `Screen::Scripts` → 设 `pending_script`。
2. 下一帧 `App::start_pending_script()`：用 `pm::preferred(project, runtime)` 选包管理器，`pm::build_run_command(&pm, &name)` 生成 `(program, args)`，再 `runner::spawn(program, args, tx)`。
3. `runner::spawn`（`system/runner.rs`）spawn 子进程，把 stdout/stderr 逐行通过 `UnboundedSender<ScriptEvent>` 推给 App；进程结束后发 `ScriptEvent::Finished(code)`。
4. `App::drain_script_output()` 用 `try_recv()` 把缓冲的 `Line`/`Finished` 刷进 `OutputState`（最多保留 500 行）。

## package.json 加载与转换

- `package_json.rs` 的 `PackageJson` 是 serde 反序列化的**外部格式**，脚本与三类依赖都用 `IndexMap`（保留 `package.json` 里的声明顺序，列表按索引取值）。
- `info.rs` 的 `ProjectInfo` 通过 `From<PackageJson>` 转换，是**内部统一表示**（依赖扁平化为 `DepEntry { name, version }`）。
- 业务代码只应依赖 `ProjectInfo`；`load_from(project_path)` 返回 `Option<ProjectInfo>`（失败静默降级为 `None`，UI 显示 "No package.json loaded"）。

## 包管理器检测与命令构建

`system/pm.rs` 是唯一处理跨平台差异的地方：

- 优先级固定为 `pnpm > bun > yarn > npm`；`detect_available()` 逐个 `--version` 探测 PATH。
- `preferred()` 解析顺序：`package.json` 的 `packageManager` 字段 → PATH 上第一个可用 → 兜底 `npm`。
- **Windows 特殊处理**：npm/pnpm/yarn 是 `.cmd` shim，必须经 `cmd /c <pm>.cmd run <script>` 执行；`bun` 是原生 `bun.exe` 直接 spawn。差异通过 `#[cfg(target_os = "windows")]` 分支隔离在 `build_command` / `build_run_command` 内。

## 设计约定

- **Rust edition 2024**。
- **注释语言**：代码注释用英文，业务/功能说明可用中文。
- **`PackageJson` → `ProjectInfo` 转换**：解耦外部格式与内部逻辑（见上文）。
- **常量集中管理**：所有魔术字符串（文件名、锁文件列表、图标、帧率、banner、应用元信息）统一放在 `src/constants/` 下，业务代码通过 `crate::constants::...` 引用。

## 关键依赖

| Crate | 用途 |
| :--- | :--- |
| `clap` (derive) | CLI 参数解析 |
| `ratatui` + `crossterm` | TUI 终端界面 |
| `serde` + `serde_json` | `package.json` 反序列化 |
| `indexmap` (serde) | 保序映射，存储 scripts / 各类依赖 |
| `tokio` (full) | 异步运行时、子进程 spawn、mpsc 通道、超时 |
| `anyhow` | 错误处理 |
| `colored` | 终端彩色输出 |

## 当前状态（v0.0.1）

- **TUI 交互模式已完整实现**（四屏：Dashboard / Scripts / Dependencies / Doctor），是当前功能主体。
- **CLI 子命令（`scripts` / `deps` / `doctor`）是占位桩**，尚未实现独立逻辑。
- `package.json` 加载、Node 版本检测、包管理器检测、脚本执行管线已可用。
- 尚未实现的功能（对应 README 特性表）：模糊搜索、多选并发/串行调度、依赖升级对比、僵尸/幽灵依赖扫描、锁文件冲突清理等。
