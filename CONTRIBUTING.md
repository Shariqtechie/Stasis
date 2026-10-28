# Contributing

Thanks for helping Stasis. Small, focused changes are easiest to review.

## Quick ways to help
- **Test an edition:** freeze, reboot, check Windows Settings, resume. Open a bug report with your edition and version, even if it worked.
- **Suggest a feature** with the feature template.

## Development
```bash
npm install
npm run tauri dev    # run as administrator
```
- `src/index.html` is the whole UI.
- `src-tauri/src/main.rs` holds the registry logic.

## Pull requests
Keep one change per PR, say what you tested and on which Windows edition, and use short commit messages like `fix: handle missing pause keys`.
