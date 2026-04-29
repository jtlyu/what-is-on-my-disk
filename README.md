# Diskwise

> WizTree's speed + an AI that explains every folder + per-app cleanup scaffolds. Open source. Local first.

[![CI](https://github.com/OWNER/diskwise/actions/workflows/ci.yml/badge.svg)](https://github.com/OWNER/diskwise/actions/workflows/ci.yml)
[![Release](https://github.com/OWNER/diskwise/actions/workflows/release.yml/badge.svg)](https://github.com/OWNER/diskwise/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Diskwise scans your disk like WizTree, then **walks you through every large folder** and tells you what it is, whether it's safe to delete, and how to clean it. For known apps (WeChat, Chrome, Steam, npm, Docker, ...) it shows a dedicated panel with safe options. For everything else, an AI advisor inspects the folder and suggests an action.

```
       ┌────────────────────────────────────────────────────┐
       │  Scan (NTFS MFT / fts)        →   Treemap + Tree   │
       │  Auto-Walk largest folders    →   Card per folder  │
       │  ├─ Scaffold matched          →   Specialised UI   │
       │  └─ Unknown                   →   AI advisor card  │
       │  Approve  →  Recycle / Quarantine  →  Undo log     │
       └────────────────────────────────────────────────────┘
```

## Why Diskwise

- **Fast scan** – direct MFT read on NTFS, parallel `walkdir` elsewhere.
- **Auto-Walk** – stops on every folder above a size threshold; you press `Recycle` / `Skip` / `Always trust`.
- **Scaffolds** – TOML manifests describe each app's caches, retention prompts, and risks. Community-extensible.
- **AI advisor** – pluggable provider (OpenAI, Anthropic, Gemini, Ollama). Sends only path metadata, never file contents (configurable).
- **Safe by default** – every delete goes to the OS recycle bin, plus a 7-day quarantine and a JSONL undo log.

## Quickstart (development)

Requires Rust 1.80+, Node 20+, pnpm 9+, and Tauri prerequisites for your OS.

```bash
git clone https://github.com/OWNER/diskwise && cd diskwise
pnpm install
pnpm tauri dev
```

For a release build:

```bash
pnpm tauri build
```

## Quickstart (users)

Grab the latest installer for your OS from [Releases](https://github.com/OWNER/diskwise/releases):

- Windows: `Diskwise_x64-setup.exe` or `.msi`
- macOS: `Diskwise_universal.dmg`
- Linux: `.AppImage` or `.deb`

## Built-in scaffolds

| Category | Scaffold | Targets |
|---|---|---|
| Messaging | `wechat-pc` | `WeChat Files/*/FileStorage` (image/video/file caches) |
| Browser | `chrome`, `edge`, `firefox` | `Cache`, `Code Cache`, `GPUCache`, `Service Worker` |
| Dev | `npm`, `pnpm`, `yarn`, `pip`, `cargo` | package manager caches |
| Containers | `docker` | unused images, build cache, dangling volumes |
| IDE | `jetbrains` | `caches`, `logs`, `system` |
| Games | `steam` | `downloading`, `shadercache`, `workshop` |

[Scaffold authoring guide](docs/SCAFFOLD-AUTHORING.md) – PRs welcome.

## Project layout

```
diskwise/
  apps/desktop/         Tauri 2 desktop app
    src/                React + TS frontend
    src-tauri/          Tauri backend (Rust)
  crates/
    scanner/            Disk scanner (walkdir + Win MFT)
    scaffold/           TOML loader + glob matcher
    executor/           Recycle / quarantine / undo
    advisor/            LLM clients (OpenAI, Ollama, ...)
  scaffolds/            Built-in *.toml manifests
  docs/
  .github/
```

## Privacy

By default Diskwise never reads file contents. The AI advisor receives only:
`{path, size, file_count, top_extensions, sample_paths (names only)}`. You can switch to a local Ollama model so nothing leaves the machine. See [SECURITY.md](SECURITY.md).

## Status

`v0.1` MVP. Cross-platform `walkdir` scanner; Win MFT direct-read landing in `v0.2`.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). The fastest way to help: write a new scaffold for an app you use. It's a single TOML file.

## License

MIT. See [LICENSE](LICENSE).
