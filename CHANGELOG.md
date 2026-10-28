# Changelog

All notable changes to Stasis are documented here. When you run the release workflow with a version, the Unreleased section becomes that version and is used as the release notes.

## [Unreleased]

### Added
- Stasis is a native Windows app built from scratch with Rust and Tauri, replacing the Python tool
- Dark glass UI with a live countdown ring that scales to the chosen freeze length
- Frameless window with the Mica effect and a custom title bar with tooltips
- Freeze presets from 7 days to 5 years, plus a custom date up to 10 years out
- Smart controls: freeze buttons hide while a Stasis freeze is active, with Extend freeze to push the date further
- Extend freeze disables options that would pass the 10-year limit and limits custom dates to a valid range
- Detects pauses made in Windows Settings and says so
- Automatic backup before every change, with one-click restore and readable dates
- Windows 11 edition and build detection
- About panel with star, share, and bug report links, with a smooth fade
- Release workflow with a version box, SHA256 checksum, and release notes taken from this changelog
- Landing page, redesigned README with an animated demo and a theme-aware star chart, issue templates, security policy, and VM test sheet

### Changed
- About panel redesigned with an app mark, a star button, and share, copy, and bug report tiles
- Release workflow actions upgraded to Node 24 versions
- Project renamed from Windows Update Pause Manager to Stasis
- Backups now live in %APPDATA%\Stasis\backups instead of the working folder
- Buttons are disabled while an action is running

### Fixed
- Windows 11 Settings now shows the pause date after a freeze on a PC that never paused from Settings, because Stasis writes the same pause start time Settings does and removes it on Resume
- Share on X opened File Explorer instead of the browser
- The Link copied confirmation was hidden behind the About panel
- The custom date option was missing from the old app
- Backups could land in System32 when run as administrator
- Pauses longer than 5 weeks are recognized as Stasis freezes, since Windows Settings cannot make them
- A freeze ending within 24 hours no longer shows as not paused, and the day count rounds up
- The freeze button glow no longer bleeds into the Backups card
- The About button icon looked pixelated

## [1.0.0]
- Initial release of Windows Update Pause Manager (Python and Tkinter)
