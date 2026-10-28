<div align="center">

<img src="assets/banner.svg" alt="Stasis: freeze Windows Update for years, not weeks" width="100%">

[![Download](https://img.shields.io/github/v/release/Shariqtechie/Stasis?label=download&color=7fe1ff&style=for-the-badge)](https://github.com/Shariqtechie/Stasis/releases/latest)
[![Stars](https://img.shields.io/github/stars/Shariqtechie/Stasis?style=for-the-badge&color=8b7bff)](https://github.com/Shariqtechie/Stasis/stargazers)
[![Downloads](https://img.shields.io/github/downloads/Shariqtechie/Stasis/total?style=for-the-badge&color=5cf2c3)](https://github.com/Shariqtechie/Stasis/releases)
[![License](https://img.shields.io/github/license/Shariqtechie/Stasis?style=for-the-badge)](LICENSE)

**Windows 11 lets you pause updates for 5 weeks. Stasis pauses them for up to 5 years.**

[Download](https://github.com/Shariqtechie/Stasis/releases/latest/download/Stasis.exe) · [Website](https://shariqtechie.github.io/Stasis/) · [Report a bug](../../issues/new/choose)

</div>

> Stasis was formerly *Windows Update Pause Manager*. Same tool, rebuilt from scratch with a native app and a new look.

<!-- TODO: add a screen recording here: ![Stasis demo](assets/demo.gif) -->

## Why Stasis

| | Windows Settings | Stasis |
|---|---|---|
| Longest pause | 5 weeks | Up to 5 years |
| Pick your own end date | No | Yes |
| Backup before changes | No | Automatic |
| Undo | Resume only | Resume or restore any backup |
| Size | Built in | About 5 MB, native |

## Why not just disable Windows Update?

Many "disable Windows Update" scripts and tools stop services, block Microsoft servers, or rewrite system policies. People regularly report that this can break the Microsoft Store and app installs, and some say the only fix they found was reinstalling Windows.

Stasis does none of that. It only sets the same pause dates that Windows Settings uses.

- **No services are disabled or removed.** The Microsoft Store, Defender, and app installs keep working.
- **No system files are touched.** The only change is a few pause values in the registry, and each one is backed up first.
- **Easy to undo, even without Stasis.** Open **Settings → Windows Update** and click **Resume updates**. You don't need the app to go back to normal.

## Features

- **One click:** pick 7 days to 5 years, or set a custom end date.
- **Live countdown** showing how long your PC stays frozen.
- **Automatic backups** of your settings before every change, with one-click restore.
- **Shows in Windows Settings:** it writes the same pause values Windows already uses.
- **Private:** no network access, no telemetry, no account.
- **Tiny and native:** built with Rust and Tauri, with no Python runtime.

## Quick start

1. Download **[Stasis.exe](https://github.com/Shariqtechie/Stasis/releases/latest/download/Stasis.exe)** from the latest release.
2. Run it and allow the admin prompt. The pause values live in `HKEY_LOCAL_MACHINE`.
3. Choose a duration and click **Freeze updates**.
4. To go back to normal, click **Resume updates** in Stasis, or open **Settings → Windows Update** and click **Resume updates** there. Stasis doesn't need to be open.

Windows SmartScreen may warn about a new, unsigned app. Check the SHA256 file in the release against your download, or build from source below.

## Compatibility

| Edition | 23H2 | 24H2 | 25H2 |
|---|---|---|---|
| Windows 11 Home | 🟡 Testing | 🟡 Testing | 🟡 Testing |
| Windows 11 Pro | 🟡 Testing | 🟡 Testing | 🟡 Testing |
| Windows 11 Enterprise | 🟡 Testing | 🟡 Testing | 🟡 Testing |

Windows 10 reached end of support in October 2025, so it is low priority. Results go here as each edition is verified in a virtual machine: freeze, reboot, check Settings, resume. Tested something? [Tell us](../../issues/new/choose).

## Security note

Freezing updates also pauses security patches. Pick a duration you are comfortable with, and resume to catch up every few months. Stasis is a convenience tool, not a security product.

## Build from source

Needs Node 20+ and the [Rust toolchain](https://rustup.rs) on Windows.

```bash
npm install
npm run build        # outputs src-tauri/target/release/stasis.exe
```

Or let GitHub Actions do it: push a tag like `v1.2.0` and the release appears with the exe and its SHA256.

## Roadmap

- [ ] Resume reminder before your freeze ends
- [x] Frameless Mica window
- [x] About panel, plus extend-freeze controls
- [ ] Verified results for every Windows 11 edition
- [ ] winget and Scoop packages
- [ ] Signed releases

## Contributing

Bugs, ideas, and VM test results are all welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

## Star history

[![Star History](https://api.star-history.com/svg?repos=Shariqtechie/Stasis&type=Date)](https://star-history.com/#Shariqtechie/Stasis&Date)

## License

MIT. Not affiliated with or endorsed by Microsoft.
