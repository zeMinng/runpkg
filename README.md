<p align="center">
  <a href="./README.zh.md">中文</a>
</p>

<p align="center">
  <img src="https://img.shields.io/npm/v/runpkg?style=flat-square&color=6366f1" alt="npm">
  <img src="https://img.shields.io/node/v/runpkg?style=flat-square&color=6366f1" alt="node">
  <img src="https://img.shields.io/github/license/zeMinng/runpkg?style=flat-square&color=6366f1" alt="license">
  <img src="https://img.shields.io/badge/status-pre--release-orange?style=flat-square" alt="status">
</p>

<h1 align="center">runpkg</h1>
<p align="center"><em>The interactive package.json cockpit for your terminal.</em></p>

---

**runpkg** replaces `npm run` with a beautiful, keyboard-driven TUI. Fuzzy-search scripts, run multiple tasks in parallel, upgrade dependencies with checkboxes, and catch missing packages — all in one command.

> ⚠️ runpkg is in early development. v0.1 focuses on the **Interactive Script Runner**.

## Features

| Module | What it does |
|---|---|
| 🚀 **Script Runner** `v0.1` | Smart menu with fuzzy search, multi-select (parallel / sequential), PM auto-detect, argument passthrough |
| 📦 **Dependency Manager** `v0.2` | Interactive upgrade UI, categorized version panorama, zombie & phantom dependency scanner |
| 🩺 **Health Check** `v0.3` | Node / PM version alignment, lockfile conflict guard, project overview card |
| 🎨 **Toolbox** `v0.3` | package.json field formatter, shell alias export |

## Install & Usage

```bash
# Install (once published)
npm install -g runpkg       # or pnpm / bun / yarn

# Open the dashboard in any project
runpkg

# Quick jumps
runpkg run                  # script runner
runpkg deps                 # dependency manager
runpkg doctor               # project health audit
```

> Requires **Node.js ≥ 18**.

## Roadmap

| Version | Module | Status |
|---|---|---|
| **v0.1** | Script Runner — menu, fuzzy search, multi-select, PM auto-detect | 🚧 In progress |
| **v0.2** | Dependency Manager — upgrade UI, version panorama, dead-dep scan | 📋 Planned |
| **v0.3** | Health Check + Toolbox — env audit, lockfile guard, pkg format | 📋 Planned |

## Contributing

Ideas, feedback, and PRs welcome. Open an [issue](https://github.com/zeMinng/runpkg/issues) before writing code — the roadmap is young and your input matters.

## License

[MIT](LICENSE) © [zeMinng](https://github.com/zeMinng)
