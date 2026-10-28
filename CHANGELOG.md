# Changelog

All notable changes to Stasis are documented here. When you run the release workflow with a version, the Unreleased section becomes that version and is used as the release notes.

## [Unreleased]

### Added
- Frameless window with the Mica effect and a custom title bar with tooltips
- Smart controls: freeze buttons hide while a Stasis freeze is active, with Extend freeze to push the date further
- Extend freeze starts with nothing selected, disables options that would go past the limit, and limits custom dates to a valid range
- Detects pauses made in Windows Settings and says so
- About panel with star, share, and bug report links
- Backups shown as readable dates
- 10-year limit on how far a freeze can be set
- Release workflow with a version box and release notes taken from this changelog
- Landing page, redesigned README with a theme-aware star chart, issue templates, security policy, and VM test sheet

### Changed
- The progress ring now scales to the chosen freeze length
- The title bar dims while the About panel is open
- Buttons are disabled while an action is running

### Fixed
- Freezes made with older builds are recognized: Windows Settings allows 5 weeks at most, so any longer pause is treated as a Stasis freeze
- A freeze ending within 24 hours no longer shows as not paused, and the day count rounds up
- Square glow around the progress ring
- The About button icon looked pixelated
- Social preview image text being cut off

## [1.0.0] - 2026-10-28

### Added
- Native Windows app rebuilt from scratch with Rust and Tauri, replacing the Python tool
- Glass UI with a live countdown ring
- Freeze presets from 7 days to 5 years, plus a custom end date picker
- Automatic backup before every change, with one-click restore
- Windows 11 edition and build detection

### Changed
- Project renamed from Windows Update Pause Manager to Stasis
- Backups now live in %APPDATA%\Stasis\backups instead of the working folder

### Fixed
- The custom date option was missing from the old app
- Backups could land in System32 when run as administrator
