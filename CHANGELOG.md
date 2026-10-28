# Changelog

All notable changes to Stasis are documented here. The release workflow turns the Unreleased section into the release notes.

## [Unreleased]

### Added
- Native Windows app rebuilt from scratch with Rust and Tauri, replacing the Python tool
- Glass UI with a live countdown ring that scales to the chosen freeze length
- Freeze presets from 7 days to 5 years, plus a custom end date picker
- Automatic backup before every change, with one-click restore and readable dates
- Windows 11 edition and build detection
- Frameless window with the Mica effect and a custom title bar
- Smart controls: freeze buttons hide while a Stasis freeze is active, with Extend freeze to push the date further
- Detects pauses made in Windows Settings and says so
- About panel with star, share, and bug report links
- Release workflow with a version box, SHA256 checksum, and release notes taken from this changelog
- Landing page, README, issue templates, security policy, and VM test sheet

### Changed
- Project renamed from Windows Update Pause Manager to Stasis
- Backups now live in %APPDATA%\Stasis\backups instead of the working folder

### Fixed
- The custom date option was missing from the old app
- Backups could land in System32 when run as administrator
- Square glow around the progress ring
- Social preview image text being cut off
