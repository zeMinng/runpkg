<div align="center">

# ⚡ runpkg

基于 **Rust** 打造的下一代交互式 `package.json` 控制台与项目驾驶舱 🦀🚀

[![Crates.io](https://img.shields.io/badge/Rust-edition_2024-6366f1?style=flat-square&logo=rust&logoColor=white)](https://crates.io/crates/runpkg)
[![GitHub License](https://img.shields.io/github/license/zeMinng/runpkg?style=flat-square&color=6366f1)](./LICENSE)
[![GitHub Stars](https://img.shields.io/github/stars/zeMinng/runpkg?style=flat-square&color=6366f1)](https://github.com/zeMinng/runpkg/stargazers)

<p align="center">
  🌐 <a href="./README.md">English</a> | <b>简体中文</b>
</p>

</div>

## ✨ 为什么选择 runpkg？

`runpkg` 是一个用 Rust 编写的原生 CLI 驾驶舱。毫秒级响应，帮你轻松调度项目脚本与监控项目健康度，摆脱繁重的 Node.js 运行时依赖。

> [!NOTE]  
> 🚧 **早期开发中**：v0.0.1 当前聚焦于 **脚本运行器 (Task Hub)** 核心功能，演示截图与 Demo 即将上线！

## ✨ 特性亮点

| 模块 | 核心能力 |
| :--- | :--- |
| 🦀 **Rust 原生极速** | 毫秒级零延迟启动，内存占用极低，摆脱 Node.js 运行时依赖 |
| 🎯 **智能 Task Hub** | 自动提取脚本与注释说明，支持模糊搜索、多选并发/串行调度及参数透传 |
| 🧠 **包管理器自适应** | 自动检测并智能调用 `pnpm` / `bun` / `yarn` / `npm` |
| 📦 **依赖健康中心** | 可视化对比升级版本，Major 大版本风险预警，智能扫描“僵尸/隐形依赖” |
| 🩺 **环境与锁文件卫士** | Node/包管理器版本对齐检查，自动侦测并清理冲突的多余 Lockfile |

## ⚡ 快速安装

从 [GitHub Releases](https://github.com/zeMinng/runpkg/releases) 下载最新预编译二进制，或使用一行命令安装：

### Windows (PowerShell)

```powershell
powershell -c "irm https://raw.githubusercontent.com/zeMinng/runpkg/main/scripts/install.ps1 | iex"
```

### macOS / Linux

```bash
curl -fsSL https://raw.githubusercontent.com/zeMinng/runpkg/main/scripts/install.sh | sh
```

> [!TIP]
> 💡 安装完成后无需全局配置 Node.js npm 包，直接在终端输入 runpkg 即可启动！

## 贡献

想法与 PR 都欢迎——不妨先<a href="https://github.com/zeMinng/runpkg/issues">提个 issue</a>聊聊。

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m '添加一些 AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

##

<div align="center">
  <p>
    🦀 <b>Rust</b> 驱动 · Crafted with ❤️ by <a href="https://github.com/zeMinng">zeMinng</a><br />
    Released under the <a href="./LICENSE">MIT License</a> © 2026–present
  </p>
</div>
