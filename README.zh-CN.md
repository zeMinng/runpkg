<p align="center">
  <a href="./README.md">English</a>
</p>

<p align="center">
  <img src="https://img.shields.io/npm/v/runpkg?style=flat-square&color=6366f1" alt="npm">
  <img src="https://img.shields.io/node/v/runpkg?style=flat-square&color=6366f1" alt="node">
  <img src="https://img.shields.io/github/license/zeMinng/runpkg?style=flat-square&color=6366f1" alt="license">
  <img src="https://img.shields.io/badge/status-pre--release-orange?style=flat-square" alt="status">
</p>

<h1 align="center">runpkg</h1>
<p align="center"><em>终端里的 package.json 交互式驾驶舱。</em></p>

---

**runpkg** 用一套高颜值、纯键盘操作的终端界面取代 `npm run`。模糊搜索脚本、多任务并行执行、复选框升级依赖、揪出遗漏的包——一个命令搞定。

> ⚠️ runpkg 仍在早期开发阶段，v0.1 聚焦于**交互式脚本运行器**。

## 功能

| 模块 | 说明 |
|---|---|
| 🚀 **脚本运行器** `v0.1` | 智能菜单 + 模糊搜索、多选执行（并发/串行）、PM 自动识别、参数透传 |
| 📦 **依赖管理** `v0.2` | 交互式升级界面、分类版本全景图、僵尸依赖 & 幽灵依赖扫描 |
| 🩺 **健康诊断** `v0.3` | Node / PM 版本对齐检查、Lockfile 冲突守卫、项目信息速览 |
| 🎨 **工具箱** `v0.3` | package.json 字段格式化、Shell 别名一键导出 |

## 安装与使用

```bash
# 安装（发布后可用）
npm install -g runpkg       # 或 pnpm / bun / yarn

# 在任意项目中打开控制台
runpkg

# 快捷入口
runpkg run                  # 脚本运行器
runpkg deps                 # 依赖管理
runpkg doctor               # 项目健康检查
```

> 需要 **Node.js ≥ 18**。

## 路线图

| 版本 | 模块 | 状态 |
|---|---|---|
| **v0.1** | 脚本运行器——菜单、模糊搜索、多选、PM 自动识别 | 🚧 开发中 |
| **v0.2** | 依赖管理——升级面板、版本全景、死依赖扫描 | 📋 规划中 |
| **v0.3** | 健康诊断 + 工具箱——环境审计、Lockfile 守卫、格式化 | 📋 规划中 |

## 参与贡献

想法、反馈、代码都欢迎。写代码之前建议先提 [issue](https://github.com/zeMinng/runpkg/issues) 讨论——项目还很年轻，你的意见很重要。

## 开源协议

[MIT](LICENSE) © [zeMinng](https://github.com/zeMinng)
