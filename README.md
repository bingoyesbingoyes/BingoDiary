<p align="center">
  <img src="src-tauri/icons/icon.png" width="128" height="128" alt="Bingo Diary Logo">
</p>

<h1 align="center">Bingo Diary</h1>

<p align="center">
  <strong>A lightweight, cross-platform diary application built with Tauri 2</strong>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#tech-stack">Tech Stack</a> •
  <a href="#installation">Installation</a> •
  <a href="#development">Development</a> •
  <a href="#architecture">Architecture</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Tauri-2.9-blue?logo=tauri" alt="Tauri">
  <img src="https://img.shields.io/badge/Vue-3.4-green?logo=vue.js" alt="Vue 3">
  <img src="https://img.shields.io/badge/Rust-1.70+-orange?logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/Platform-macOS%20|%20Windows%20|%20Linux%20|%20Android-lightgrey" alt="Platform">
</p>

---

## Overview | 概述

Bingo Diary is a modern, privacy-focused diary application that runs entirely offline. Built with Tauri 2 and Rust, it offers exceptional performance with minimal resource usage. Features include Markdown editing with LaTeX math support, Google Drive sync, tag management, schedule events with alarms, and beautiful glassmorphism UI.

Bingo Diary 是一款现代化、注重隐私的日记应用，完全离线运行。基于 Tauri 2 和 Rust 构建，提供卓越的性能和极低的资源占用。功能包括支持 LaTeX 数学公式的 Markdown 编辑、Google Drive 云同步、标签管理、带闹钟的日程事件，以及精美的毛玻璃 UI 设计。



---

## Features | 功能特性

### Core Features | 核心功能

| Feature | Description | 功能 | 描述 |
|---------|-------------|------|------|
| **Calendar View** | Navigate diary entries by date with visual indicators | **日历视图** | 通过日历导航日记条目，带可视化标记 |
| **Markdown Editor** | Full-featured editor with live preview toggle | **Markdown 编辑器** | 全功能编辑器，支持实时预览切换 |
| **LaTeX Math** | KaTeX-powered math formulas (`$...$` and `$$...$$`) | **LaTeX 公式** | KaTeX 驱动的数学公式支持 |
| **Image Embedding** | Drag-drop or paste images directly into entries | **图片嵌入** | 拖放或粘贴图片到日记中 |
| **Full-text Search** | Search across all diary entries instantly | **全文搜索** | 跨所有日记条目即时搜索 |

### Organization | 组织管理

| Feature | Description | 功能 | 描述 |
|---------|-------------|------|------|
| **Tag System** | Create tags with custom colors for categorization | **标签系统** | 创建带自定义颜色的标签进行分类 |
| **Tag Filtering** | Filter entries by single or multiple tags | **标签筛选** | 按单个或多个标签筛选条目 |
| **Schedule Events** | Add timed events with alarm notifications | **日程事件** | 添加带闹钟通知的定时事件 |

### Cloud & Export | 云端与导出

| Feature | Description | 功能 | 描述 |
|---------|-------------|------|------|
| **Google Drive Sync** | Bi-directional sync with OAuth2 authentication | **Google Drive 同步** | 双向同步，OAuth2 认证 |
| **Auto Sync** | Configurable sync intervals (5min to 1hr) | **自动同步** | 可配置的同步间隔（5分钟到1小时） |
| **PDF Export** | Export with rendered Markdown and math formulas | **PDF 导出** | 导出渲染后的 Markdown 和数学公式 |
| **JSON Backup** | Full diary backup and restore | **JSON 备份** | 完整日记备份与恢复 |
| **Markdown Export** | Export with embedded images | **Markdown 导出** | 导出包含嵌入图片的文件 |

### Customization | 个性化

| Feature | Description | 功能 | 描述 |
|---------|-------------|------|------|
| **Dark/Light Theme** | Toggle between themes with persistence | **深色/浅色主题** | 主题切换并持久化 |
| **Custom Backgrounds** | Presets, solid colors, or custom images | **自定义背景** | 预设、纯色或自定义图片 |
| **Glassmorphism UI** | Modern translucent panel design | **毛玻璃 UI** | 现代半透明面板设计 |
| **Multi-language** | English and Chinese support (i18n) | **多语言** | 英文和中文支持 |

### Security | 安全

| Feature | Description | 功能 | 描述 |
|---------|-------------|------|------|
| **App Password** | bcrypt-hashed startup password protection | **应用密码** | bcrypt 加密的启动密码保护 |
| **Lock Screen** | Password-protected access | **锁屏界面** | 密码保护访问 |

---

## Tech Stack | 技术栈

### Frontend | 前端

| Technology | Version | Purpose | 用途 |
|------------|---------|---------|------|
| [Vue 3](https://vuejs.org/) | ^3.4.18 | Reactive UI framework | 响应式 UI 框架 |
| [Vite](https://vitejs.dev/) | ^7.3.0 | Next-generation build tool | 新一代构建工具 |
| [Tailwind CSS](https://tailwindcss.com/) | ^3.4.19 | Utility-first CSS framework | 原子化 CSS 框架 |
| [v-calendar](https://vcalendar.io/) | ^3.1.2 | Date picker component | 日期选择组件 |
| [marked](https://marked.js.org/) | ^17.0.1 | Markdown parser | Markdown 解析器 |
| [KaTeX](https://katex.org/) | ^0.16.27 | LaTeX math rendering | LaTeX 数学渲染 |
| [SASS](https://sass-lang.com/) | ^1.97.1 | CSS preprocessor | CSS 预处理器 |

### Backend | 后端

| Crate | Version | Purpose | 用途 |
|-------|---------|---------|------|
| [tauri](https://v2.tauri.app/) | 2.9.5 | Cross-platform app framework | 跨平台应用框架 |
| [serde](https://serde.rs/) | 1.0 | Serialization/deserialization | 序列化/反序列化 |
| [chrono](https://docs.rs/chrono/) | 0.4 | Date/time handling | 日期时间处理 |
| [pulldown-cmark](https://docs.rs/pulldown-cmark/) | 0.9 | Markdown parsing | Markdown 解析 |
| [katex](https://docs.rs/katex/) | 0.4 | Server-side LaTeX rendering | 服务端 LaTeX 渲染 |
| [bcrypt](https://docs.rs/bcrypt/) | 0.15 | Password hashing | 密码哈希 |
| [headless_chrome](https://docs.rs/headless_chrome/) | 1.0 | PDF generation | PDF 生成 |
| [reqwest](https://docs.rs/reqwest/) | 0.11 | HTTP client for sync | HTTP 客户端 |
| [oauth2](https://docs.rs/oauth2/) | 4.4 | Google OAuth2 | Google OAuth2 认证 |
| [tokio](https://tokio.rs/) | 1.0 | Async runtime | 异步运行时 |

### Tauri Plugins | Tauri 插件

- `tauri-plugin-fs` - File system access | 文件系统访问
- `tauri-plugin-dialog` - Native dialogs | 原生对话框
- `tauri-plugin-shell` - Shell command execution | Shell 命令执行
- `tauri-plugin-os` - OS detection | 操作系统检测
- `tauri-plugin-notification` - System notifications | 系统通知
- `tauri-plugin-clipboard-manager` - Clipboard access | 剪贴板访问
- `tauri-plugin-log` - Application logging | 应用日志

---

## Installation | 安装

- [Node.js](https://nodejs.org/) >= 18
- [Rust](https://rustup.rs/) >= 1.70
- [Tauri CLI](https://v2.tauri.app/start/prerequisites/)

```bash
# Clone repository | 克隆仓库
git clone https://github.com/BingoYes/BingoDiary.git
cd BingoDiary

# Install dependencies | 安装依赖
npm install

# Run in development mode | 开发模式运行
npm run dev

# Build for production | 生产构建
npm run build
```

#### Android Build | Android 构建

```bash
# Initialize Android project | 初始化 Android 项目
npx tauri android init

# Run on device/emulator | 在设备/模拟器上运行
npx tauri android dev

# Build APK | 构建 APK
npx tauri android build
```

---

## Usage | 使用指南

### Getting Started | 快速开始

1. **Create Entry | 创建条目**: Select a date from the calendar, write your content in Markdown

   选择日历中的日期，用 Markdown 编写内容

2. **Save | 保存**: Press `Cmd+S` (macOS) or `Ctrl+S` (Windows/Linux), or use auto-save on mobile

   按 `Cmd+S` (macOS) 或 `Ctrl+S` (Windows/Linux)，移动端自动保存

3. **Add Images | 添加图片**: Drag-drop or paste images directly into the editor

   直接拖放或粘贴图片到编辑器

4. **Math Formulas | 数学公式**: Use `$E=mc^2$` for inline or `$$\sum_{i=1}^n x_i$$` for block formulas

   使用 `$E=mc^2$` 行内公式或 `$$\sum_{i=1}^n x_i$$` 块级公式

### Keyboard Shortcuts | 快捷键

| Shortcut | Action | 快捷键 | 操作 |
|----------|--------|--------|------|
| `Cmd/Ctrl + S` | Save entry | `Cmd/Ctrl + S` | 保存条目 |
| `Cmd/Ctrl + F` | Search | `Cmd/Ctrl + F` | 搜索 |
| `Cmd/Ctrl + E` | Toggle edit/preview | `Cmd/Ctrl + E` | 切换编辑/预览 |

### Google Drive Sync Setup | Google Drive 同步设置

1. Open Settings > Sync | 打开设置 > 同步
2. Click "Connect to Google Drive" | 点击"连接 Google Drive"
3. Authorize the application | 授权应用
4. Enable auto-sync and set interval | 启用自动同步并设置间隔

---

## Architecture | 架构设计

```
┌─────────────────────────────────────────────────────────────┐
│                    Frontend (Vue 3 + Vite)                  │
│  ┌──────────────┐  ┌─────────────┐  ┌───────────────────┐  │
│  │   Index.vue  │  │ Components  │  │    i18n (EN/ZH)   │  │
│  │  (Main View) │  │  (Modular)  │  │                   │  │
│  └──────┬───────┘  └──────┬──────┘  └─────────┬─────────┘  │
│         │                 │                    │            │
│         └─────────────────┴────────────────────┘            │
│                           │                                 │
│               ┌───────────▼───────────┐                     │
│               │  @tauri-apps/api/core │                     │
│               │   invoke(), listen()  │                     │
│               └───────────┬───────────┘                     │
└───────────────────────────┼─────────────────────────────────┘
                            │ IPC (Inter-Process Communication)
┌───────────────────────────┼─────────────────────────────────┐
│                     Backend (Rust + Tauri 2)                │
│               ┌───────────▼───────────┐                     │
│               │     commands.rs       │  ← API Layer        │
│               └───────────┬───────────┘                     │
│      ┌────────────────────┼────────────────────┐            │
│      │          ┌─────────┴─────────┐          │            │
│      ▼          ▼                   ▼          ▼            │
│  ┌───────┐  ┌────────┐  ┌───────┐  ┌─────┐  ┌───────┐      │
│  │diary  │  │config  │  │ tags  │  │ pdf │  │ sync/ │      │
│  │.rs    │  │.rs     │  │ .rs   │  │ .rs │  │       │      │
│  └───┬───┘  └───┬────┘  └───┬───┘  └──┬──┘  └───┬───┘      │
│      │          │           │         │         │           │
│      └──────────┴───────────┴─────────┴─────────┘           │
│                             │                               │
│               ┌─────────────▼─────────────┐                 │
│               │       File System         │                 │
│               │  ~/Library/Application    │                 │
│               │  Support/com.bingoyes.    │                 │
│               │  diary/                   │                 │
│               │  ├── diary/    (entries)  │                 │
│               │  ├── images/   (assets)   │                 │
│               │  ├── config.json          │                 │
│               │  └── tags.json            │                 │
│               └───────────────────────────┘                 │
└─────────────────────────────────────────────────────────────┘
```

### Project Structure | 项目结构

```
BingoDiary/
├── src/                          # Frontend source | 前端源码
│   ├── main.js                   # Vue app entry | Vue 应用入口
│   ├── App.vue                   # Root component | 根组件
│   ├── pages/
│   │   └── Index.vue             # Main view | 主视图
│   ├── components/               # UI components | UI 组件
│   │   ├── MarkdownEditor.vue    # Editor with preview | 编辑器
│   │   ├── SettingsModal.vue     # Settings dialog | 设置对话框
│   │   ├── TagManager.vue        # Tag CRUD | 标签管理
│   │   ├── SyncSettings.vue      # Google Drive config | 同步配置
│   │   ├── EventModal.vue        # Schedule events | 日程事件
│   │   └── LockScreen.vue        # Password lock | 密码锁屏
│   ├── css/                      # Stylesheets | 样式表
│   └── i18n/                     # Translations | 翻译
├── src-tauri/                    # Backend source | 后端源码
│   ├── src/
│   │   ├── lib.rs                # Plugin registration | 插件注册
│   │   ├── commands.rs           # Tauri commands | Tauri 命令
│   │   ├── diary.rs              # Diary operations | 日记操作
│   │   ├── config.rs             # Configuration | 配置管理
│   │   ├── tags.rs               # Tag management | 标签管理
│   │   ├── pdf.rs                # PDF export | PDF 导出
│   │   └── sync/                 # Google Drive sync | 云同步
│   │       ├── auth.rs           # OAuth2 flow | OAuth2 流程
│   │       ├── drive.rs          # Drive API client | Drive API
│   │       └── engine.rs         # Sync logic | 同步逻辑
│   ├── Cargo.toml                # Rust dependencies | Rust 依赖
│   └── tauri.conf.json           # Tauri config | Tauri 配置
├── package.json                  # Node dependencies | Node 依赖
├── vite.config.js                # Vite config | Vite 配置
└── tailwind.config.js            # Tailwind config | Tailwind 配置
```

---

## Performance Comparison | 性能对比

Compared to the original Electron version | 与原 Electron 版本对比:

| Metric | Electron | Tauri | Improvement |
|--------|----------|-------|-------------|
| **Package Size** | ~150 MB | ~8 MB | **95% smaller** |
| **Startup Time** | 2-3s | 0.5-1s | **70% faster** |
| **Memory Usage** | ~200 MB | ~60 MB | **70% less** |
| **CPU Idle** | 2-5% | <1% | **80% less** |

---



## License | 许可证

[MIT License](LICENSE)

---

## Author | 作者

**bingoyes** - [bingoyesbingoyes@gmail.com](mailto:bingoyesbingoyes@gmail.com)

---

<p align="center">
  Made with ❤️ using <a href="https://v2.tauri.app/">Tauri 2</a>
</p>
