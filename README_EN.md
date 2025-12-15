# Antigravity Tools 🚀

<div align="center">
  <img src="public/icon.png" alt="Antigravity Logo" width="120" height="120" style="border-radius: 24px; box-shadow: 0 10px 30px rgba(0,0,0,0.15);">

  <h3>Professional Account Management for AI Services</h3>
  <p>Manage your Gemini / Claude accounts with ease. Unlimited Possibilities.</p>
  
  <p>
    <a href="https://github.com/lbjlaq/Antigravity-Manager">
      <img src="https://img.shields.io/badge/Version-2.0.0-blue?style=flat-square" alt="Version">
    </a>
    <img src="https://img.shields.io/badge/Tauri-v2-orange?style=flat-square" alt="Tauri">
    <img src="https://img.shields.io/badge/React-18-61DAFB?style=flat-square" alt="React">
    <img src="https://img.shields.io/badge/License-MIT-green?style=flat-square" alt="License">
  </p>

  <p>
    <a href="#-downloads">📥 Download</a> • 
    <a href="#-features">✨ Features</a> • 
    <a href="#-comparison">🆚 Comparison</a>
  </p>
  
  <p>
    <a href="./README.md">🇨🇳 简体中文</a> | 
    <strong>🇺🇸 English</strong>
  </p>
</div>

---

<div align="center">
  <img src="docs/images/accounts-dark.png" alt="Antigravity Dark Mode" style="border-radius: 12px; box-shadow: 0 20px 40px rgba(0,0,0,0.2); width: 100%; max-width: 800px;">
  <p><i>(Deep Dark Mode: Increased productivity)</i></p>
</div>

## 🎨 Gallery

<div align="center">

| **Light Mode** | **Dark Mode** |
| :---: | :---: |
| <img src="docs/images/dashboard-light.png" width="100%" style="border-radius: 8px;"> | <img src="docs/images/accounts-dark.png" width="100%" style="border-radius: 8px;"> |
| **Dashboard** | **Accounts** |

| <img src="docs/images/accounts-light.png" width="100%" style="border-radius: 8px;"> | <img src="docs/images/settings-dark.png" width="100%" style="border-radius: 8px;"> |
| **List View** | **Settings** |

</div>

---

**Antigravity Tools** is a **modern account management tool** built for AI developers and power users.

As the 2.0 rewrite of [Antigravity Manager](https://github.com/lbjlaq/Antigravity-Manager), it leverages the high-performance **[Tauri v2](https://v2.tauri.app/)** + **[React](https://react.dev/)** stack, evolving from a heavy Python GUI into a lightweight, blazing-fast native application.

It helps you effortlessly manage dozens of **Google Gemini** and **Claude 3.5** accounts, monitoring Quotas in real-time, and intelligently switching accounts when quotas are exhausted, enabling an "unlimited" AI experience.

> ⚠️ **Note**: The project repository URL remains unchanged at [lbjlaq/Antigravity-Manager](https://github.com/lbjlaq/Antigravity-Manager).
>
> **Looking for v1.0?**
> The source code for v1.0 (Python/Flet) has been archived in the [v1 branch](https://github.com/lbjlaq/Antigravity-Manager/tree/v1). Switch branches to view or maintain the legacy version.

## 🆚 Why v2.0? (Comparison)

| Feature | 🐢 v1.0 (Legacy) | 🚀 v2.0 (New) | Improvement |
| :--- | :--- | :--- | :--- |
| **Core Tech** | Python + Flet | **Rust (Tauri)** + **React** | **Performance Leap** |
| **Bundle Size** | ~80 MB | **~10 MB** | **87% Smaller** |
| **Startup Time** | Slow (Interpreted) | **Instant** (Native Binary) | **Blazing Fast** |
| **Memory Usage** | High (>200MB) | **Tiny** (<50MB) | **Efficient** |
| **UI/UX** | Basic Material | **Modern Glassmorphism** | **Beautiful** |
| **Security** | Plaintext/Obfuscated | **Encrypted SQLite** | **Secure** |
| **Extensibility** | Hard (Python Deps) | **Easy** (Web Stack) | **Rich Ecosystem** |

## ✨ Features

### 📊 Dashboard
- **Overview**: Real-time display of total accounts and average quota per model. Health status at a glance.
- **Smart Recommendation**: Automatically filters the "Best Account" with the most available quota, supporting one-click switching to always use optimal resources.
- **Monitoring**: Real-time highlighting of low-quota accounts to prevent interruptions.

### 👥 Account Management
- **Import Methods**:
    - 🔥 **OAuth**: Open browser for Google login authorization to auto-fetch tokens (Recommended).
    - 📋 **Manual**: Directly paste Refresh Tokens.
    - 📂 **V1 Migration**: Automatically scan and batch import legacy data from v1 (`~/.antigravity-agent`).
    - 🔄 **Local Sync**: Auto-read and import currently logged-in accounts from local IDE databases (Cursor/Windsurf).
- **Batch Actions**: Bulk refresh quotas, export backups (JSON), and batch delete.
- **Search**: Fast keyword search by email to easily manage dozens of accounts.

### 🔄 Quota Sync
- **Auto Refresh**: Configurable background polling for the latest quota info.
- **Token Keep-alive**: Built-in automatic token refreshing to ensure connections remain valid.
- **Precise Display**: Clearly shows remaining percentages and reset times for Gemini / Claude models.

### 🛠️ System Integration
- **Tray Icon**: Minimizes to the system tray for silent background operation without taking up taskbar space.
- **Quick Actions**: Tray menu supports one-click quota viewing and quick switching to the next available account.
- **Secure Storage**: Fully local encrypted SQLite storage. All token data is kept locally on your machine and never uploaded to the cloud.

### ⚙️ Settings
- **Internationalization**: Native support for **English** / **Simplified Chinese** switching.
- **Theme**: Perfect adaptation for System Dark/Light modes.
- **Data Management**: Custom export paths and one-click log cache cleaning.

## 🛠️ Tech Stack

Built with a cutting-edge modern stack ensuring high performance and maintainability:

| Module | Tech Choice | Description |
| :--- | :--- | :--- |
| **Frontend** | React 18 + TypeScript | UI Construction & Logic |
| **UI Framework** | TailwindCSS + DaisyUI | Modern Atomic CSS & Components |
| **Backend** | Tauri v2 (Rust) | High-performance System Interaction |
| **Database** | SQLite (rusqlite) | Local Persistent Storage |
| **State** | Zustand | Lightweight Global State Management |
| **Network** | Reqwest (Async) | Async Network Requests |

## 📦 Installation & Run

### 📥 Download

Visit the [Releases Page](https://github.com/lbjlaq/Antigravity-Manager/releases) to download the installer for your system:

- **macOS**: Supports Intel (`.dmg`) and Apple Silicon (`.dmg`)
- **Windows**: `.exe` Installer
- **Linux**: `.deb` or `.AppImage` *(Theoretical support, untested, feedback welcome)*

### 💻 Development

If you're a developer and want to contribute:

```bash
# 1. Clone project
git clone https://github.com/lbjlaq/antigravity-tools.git

# 2. Install dependencies
npm install

# 3. Start dev server (Frontend + Backend)
npm run tauri dev
```

### 🏗️ Build

```bash
# Build Universal macOS App (Intel & Apple Silicon)
npm run build:universal
```

## 👤 Author

**Ctrler**

- 💬 WeChat: `Ctrler`
- 🐙 GitHub: [@lbjlaq](https://github.com/lbjlaq)

## 📄 License

Copyright © 2025 Antigravity. All rights reserved.
MIT License
