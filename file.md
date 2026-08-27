## 最重要的架构调整

```
src/
├── main.rs
│
├── app/
│   ├── mod.rs
│   ├── app.rs
│   ├── state.rs
│   └── action.rs
│
├── cli/
│   ├── mod.rs
│   └── args.rs
│
├── domain/
│   ├── mod.rs
│   ├── project.rs
│   ├── script.rs
│   ├── dependency.rs
│   ├── runtime.rs
│   └── package_manager.rs
│
├── project/
│   ├── mod.rs
│   ├── loader.rs
│   └── package_json.rs
│
├── system/
│   ├── mod.rs
│   ├── node.rs
│   ├── pm.rs
│   └── runtime.rs
│
├── command/
│   ├── mod.rs
│   ├── script.rs
│   ├── dependency.rs
│   └── package_manager.rs
│
├── tui/
│   ├── mod.rs
│   ├── terminal.rs
│   ├── event.rs
│   ├── theme.rs
│   │
│   ├── components/
│   │   ├── header.rs
│   │   ├── sidebar.rs
│   │   ├── status_bar.rs
│   │   ├── script_list.rs
│   │   ├── dependency_list.rs
│   │   └── output.rs
│   │
│   └── screens/
│       ├── dashboard.rs
│       ├── scripts.rs
│       ├── dependencies.rs
│       └── doctor.rs
│
└── constants/
```


## 切换逻辑

核心是**双重焦点机制（Focus Trapping）**：左（侧边栏）与右（内容区）各有一个"键盘焦点归属"，同一时刻只有一边响应 `↑/↓`，从而避免两侧都是上下选单时的按键冲突。

**状态模型**

- `Focus::{Sidebar, Content}`：当前键盘焦点在哪一边。
- `Screen::{Dashboard, Scripts, Dependencies, Doctor}`：当前激活的 screen（由侧边栏选中项决定）。
- 每个 screen 维护自己的 cursor（如 `script_cursor` / `dep_cursor`）。

**统一按键表**

| 按键 | 焦点在 Sidebar | 焦点在 Content |
| :--- | :--- | :--- |
| `↑` / `k` | 上一个 screen | 上一个列表项 |
| `↓` / `j` | 下一个 screen | 下一个列表项 |
| `→` / `l` / `Tab` | 进入内容区（仅 Scripts / Dependencies） | — |
| `←` / `h` / `Esc` | — | 退回 Sidebar |
| `Enter` | 进入内容区（同 `→`） | 执行动作（如运行脚本） |
| `1`–`4` | 跳转 screen | 跳转 screen 并回 Sidebar |
| `r` / `q` | 刷新 / 退出 | 刷新 / 退出 |

**规则要点**

- 侧边栏 `↑/↓` 只负责切 screen，右侧内容**实时跟随刷新**（移动即预览，不设独立的"预览态"，避免多余状态）。
- `←/→` 专用于"焦点进出"，不再承担切屏；切屏只靠 `↑/↓` 与数字键。
- `Enter` 是双语义：Sidebar 下 = 进入内容区，Content 下 = 执行动作；`→` / `Tab` 只"深入"不"执行"。
- 只有 Scripts / Dependencies 有可交互列表，Dashboard / Doctor 为只读，故 `Enter` / `→` 在它们身上不切换焦点。

**视觉反馈（Focus Trapping 的关键）**

- **活动边框（Active Border）**：拥有焦点的面板边框高亮（亮青），无焦点面板边框变暗（暗灰）。
- **光标差异（Cursor Indicator）**：
  - Sidebar 聚焦：当前 screen 显示 `▶`；内容区边框暗、选中行降级为 `▸` 或不高亮。
  - Content 聚焦：内容区边框亮、选中行高亮；侧边栏边框暗、当前项降级为 `▸`。
- **状态栏 hints** 随 focus 切换文案，明确当前 `↑/↓` 作用对象。
