# Tinker MC 0.2.0

This is the first real feature release after the desktop migration.

## What's new

- Persistent server-side Modrinth linking through `.tinker-mod-links.json`.
- Exact JAR hash identities (when available) are used for manual link persistence.
- Mod installs/updates select versions compatible with the active Minecraft version and loader instead of relying on Modrinth's project-global `latest_version` field.
- Removed the redundant “Already installed” button.
- Added round icons to the main tabs.
- Added a Players tab marked as Work in Progress.
- Added a structured Configuration editor with switches, numeric controls, text controls, and an advanced raw editor.
- Configuration's mod list only shows mods with recognized configuration files and explains the filtering with a warning.
- Added a Windows release subsystem setting so the installed application does not open an extra terminal window.
- Added signed Tauri updater support and GitHub Actions publishing of updater metadata.

## Known WIP

The Players tab's NBT Explorer integration is intentionally still a work in progress. It does not yet provide a remote player-data download/save-back workflow.
