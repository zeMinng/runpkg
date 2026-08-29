# 打造交互式 CLI 工具

## Turn 1

### 🤖 Assistant

我们可以将产品定位为一个 **“下一代交互式 package.json 控制台 / 项目驾驶舱 (Interactive package.json Workspace Manager)”**。
以下是规划的**全功能图谱**。你可以采用 **“主干 MVP + 扩展模块”** 的方式逐步迭代发布：

## 🚀 模块一：交互式脚本运行器 (Interactive Task Hub)
卖点：比 npm run 或 pnpm run 颜值高 10 倍、功能强 10 倍的脚本菜单。
- **智能脚本识别与菜单**：
  - 解析 `scripts` 字段，以极简漂亮的终端列表展示。
  - **脚本注释提取**：自动读取 `package.json` 中的 `scripts-info` 或代码上方/内部的注释，在选项旁显示 hint（如 `dev`: "启动本地开发服务器"）。
- **包管理器自动识别 (PM Auto-Detect)**：
  - 自动识别 `pnpm` / `bun` / `yarn` / `npm`（通过 lockfile 或 `packageManager` 字段），免去用户思考用哪个命令的烦恼。
- **实时模糊搜索与筛选**：输入任意字母（如 `test`）秒级过滤脚本。
- **多脚本组合运行**：
  - 支持按 `Space` 键多选（如同时选中 `lint` 和 `type-check`）。
  - 支持选择**并发 (Parallel)** 或 **串行 (Sequential)** 执行。
- **参数透传**：支持在交互选择后，追加传递自定义命令行参数。

## 📦 模块二：依赖健康度与更新中心 (Dependency Manager)
卖点：可视化、高颜值的交互式 npm outdated。
- **依赖全景图谱**：
  - 将 `dependencies`、`devDependencies`、`peerDependencies` 分分类别，清晰展示当前版本、锁定版本和最新版本。
- **交互式一键升级**：
  - **按需勾选**：上下键选择要更新的包，按空格选中，一键自动升级。
  - **安全升级策略**：可切换“平滑升级（符合 `^` 或 `~` 约束）”或“强行升级到 Latest”。
  - **Break Change 预警**：大版本（Major Version）变更自动标红并弹窗提示风险。
- **冗余与僵尸依赖检测 (Zombie Dependency Scanner)**：
  - 扫描项目文件，找出 `package.json` 里写了但**代码中从未 import 过**的“死依赖”。
  - 找出代码中 `import` 了，但**忘在 package.json 里声明**的“隐形依赖”。

## 🩺 模块三：环境与健康诊断 (Project Sanity & Environment)
卖点：团队协作避坑神器，避免“在我电脑上能跑，在你电脑上跑不起来”。
- Node / 包管理器版本对齐检查：
  - 对比当前机器的 Node/pnpm/npm 版本与 `package.json` 中 `engines` / `packageManager` 字段的要求，不匹配时直观报错并提示安装指引。
- **锁文件一致性校验 (Lockfile Guard)**：
  - 检测根目录下是否存在多个冲突的 Lockfile（如既有 `pnpm-lock.yaml` 又有 `package-lock.json`），并提供“一键清理”选项。
- **项目元信息速览**：
  - 一键显示项目轻量简报（代码行数估算、协议 License、Git 分支与未提交文件状态）。

## 🎨 模块四：零配置工具箱与快捷导出 (Utilities)

- **交互式 package.json 修复与格式化**：
  - 自动将字段按官方规范重排序（如 `name` -> `version` -> `scripts` -> `dependencies`）。
  - 补全缺失的常用字段（如 `repository`、`author`、`license`）。
- **Terminal Alias 一键导出**：
  - 可以根据项目常用命令，生成系统的 Shell Alias 快捷键。

## 📊 功能优先级路线图建议 (Roadmap)

| 阶段 | 对应功能模块 | 目标 |
| --- | --- | --- |
| v0.1 (MVP) | 模块一（智能脚本运行 + 包管理器自动识别） | 快速发布，抢占种子用户 |
| v0.2 (升级版) | 模块二（依赖全景图 + 交互式升级） | 形成核心竞争力，引发自媒体传播 |
| v0.3 (全能版) | 模块三 + 模块四（健康诊断 + 僵尸依赖检测） | 稳固开源生态，冲击 1k+ Star |
