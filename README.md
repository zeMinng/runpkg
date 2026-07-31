<div align="center">

# ⚡ runpkg

A next-generation interactive `package.json` console & project cockpit — built in **Rust** 🦀🚀

[![Crates.io](https://img.shields.io/badge/Rust-edition_2024-6366f1?style=flat-square&logo=rust&logoColor=white)](https://crates.io/crates/runpkg)
[![GitHub License](https://img.shields.io/github/license/zeMinng/runpkg?style=flat-square&color=6366f1)](./LICENSE)
[![GitHub Stars](https://img.shields.io/github/stars/zeMinng/runpkg?style=flat-square&color=6366f1)](https://github.com/zeMinng/runpkg/stargazers)

<p align="center">
  🌐 <b>English</b> | <a href="./README.zh-CN.md">简体中文</a>
</p>

</div>

## ✨ Why runpkg?

`runpkg` is a native CLI cockpit written in Rust. Millisecond responsiveness lets you effortlessly dispatch project scripts and monitor project health — no heavy Node.js runtime dependency.

> [!NOTE]
> 🚧 **Early stage**: v0.1 currently focuses on the **Script Runner (Task Hub)** core. Screenshots & demos coming soon!

## ✨ Features

| Module | Capability |
| :--- | :--- |
| 🦀 **Rust-Native Speed** | Millisecond zero-latency startup, minimal memory footprint, zero Node.js runtime dependency |
| 🎯 **Smart Task Hub** | Auto-extracts scripts & inline comments, supports fuzzy search, multi-select concurrent/sequential scheduling & argument passthrough |
| 🧠 **Package Manager Auto-Detect** | Automatically detects and intelligently delegates to `pnpm` / `bun` / `yarn` / `npm` |
| 📦 **Dependency Health Center** | Visual version upgrade comparison, Major-version risk warnings, smart "zombie & phantom dependency" scanner |
| 🩺 **Environment & Lockfile Guard** | Node / PM version alignment check, auto-detect & clean conflicting duplicate lockfiles |

## ⚡ Quick Install

Thanks to Rust's single-binary build, you can install directly via multiple native channels:

### Via Cargo (Recommended)

```bash
cargo install runpkg
```

### Via Homebrew (macOS / Linux)

```bash
brew install your-username/tap/runpkg
```

### Quick Install Script (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/your-username/runpkg/main/install.sh | sh
```

> [!TIP]
> 💡 No global Node.js npm package setup needed — just type `runpkg` (or the shorthand `rp`) in your terminal!

## Contributing

Ideas & PRs are welcome — feel free to <a href="https://github.com/zeMinng/runpkg/issues">open an issue</a> first.

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

<div align="center">
  <p>
    <sub>🦀 <b>Rust</b>-powered · Crafted with ❤️ by <a href="https://github.com/zeMinng">zeMinng</a></sub><br />
    <sub>Released under the <a href="./LICENSE">MIT License</a> © 2025–present</sub>
  </p>
</div>
