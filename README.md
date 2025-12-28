# Hytale Rich Presence

<div align="center">
  <img src="static/icon.png" alt="Hytale Rich Presence Logo" width="128" height="128" />
  <br />
  <br />
  <p>
    <strong>Show off your Hytale hype on Discord with a beautiful Rich Presence.</strong>
  </p>
  <p>
    <a href="https://github.com/CustomEntity/hytale-rich-presence/releases">Download Latest Release</a>
    ·
    <a href="https://github.com/CustomEntity/hytale-rich-presence/issues">Report Bug</a>
    ·
    <a href="https://github.com/CustomEntity/hytale-rich-presence/pulls">Request Feature</a>
  </p>
</div>

## ✨ Features

- **Custom Activities**: Choose from predefined activities (Exploring, Building, PvP, etc.) or type your own custom status!
- **Party Mode**: Show that you are playing on a server with friends.
- **Server Integration**: Display the server name you are playing on.
- **Party Size**: Configurable party size and max players (e.g., "Playing on Orbis (3 of 10)").
- **Join Button**: Add a clickable "Join Server" button to your profile (optional).
- **Orbis Integration**: "Discover more on Orbis" button to link to the Orbis platform.
- **Beautiful UI**: A clean, modern interface inspired by Hytale's aesthetic.
- **Cross-Platform**: Available for Windows, macOS, and Linux.

## 🚀 Installation

1. Go to the [Releases](https://github.com/CustomEntity/hytale-rich-presence/releases) page.
2. Download the installer for your operating system:
   - **Windows**: `.exe` or `.msi`
   - **macOS**: `.dmg` or `.app`
   - **Linux**: `.deb` or `.AppImage`
3. Install and run the application.
4. Discord will automatically detect the Rich Presence!

## 🛠️ Usage

1. **Connect**: Click the "Connect to Discord" button.
2. **Select Activity**: Choose an activity from the grid or select "Custom..." to type your own.
3. **Party Mode**: Check "On a Server" to enable party features.
   - Enter the **Server Name**.
   - Set **Players** and **Max** count.
   - (Optional) Add a **Join Server URL**.
4. **Enjoy**: Your Discord profile now shows your Hytale status!

## 💻 Development

This project is built with [Tauri](https://tauri.app/), [Svelte](https://svelte.dev/), and [Vite](https://vitejs.dev/).

### Prerequisites

- Node.js (pnpm recommended)
- Rust (cargo)

### Setup

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

Distributed under the MIT License. See `LICENSE` for more information.

## 💖 Powered by

<a href="https://orbis.place">
  <img src="static/orbis-logo.png" alt="Powered by Orbis" height="30" />
</a>
