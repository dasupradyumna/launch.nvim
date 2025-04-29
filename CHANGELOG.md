# CHANGELOG

## v1.0.0 – First Stable Release 🎉

This release marks the first stable version of the plugin with a fully interactive and event-driven
state-machine based task launcher UI.

### Features
- **Launcher UI with Modes**: Introduces **select**, **edit**, and **view** modes, each with
    fully tailored UI interactions, with all default keymaps removed
- **Task Config Editing**: Add, delete, and modify config fields `name`, `cmd`, `args`, `env`, `cwd`
    and `disp` directly from the UI, in edit mode
- **Contextual Help**: Dedicated help overlays with available keybindings in every window
- **Active Task Viewer**: View and relaunch currently active tasks, described by their titles which
    include config name along with start timestamp
- **Undo/Redo Support**: Intuitive undo/redo history for edit and select modes
- **Versioned Runtime Configs**: Runtime configs are stored in neovim data directory with version
    metadata to support future schema upgrades

### Fixes
- Fixed crashes related to config copy actions, mode switching, and out-of-bounds access
- Prevented buffer/window leaks and ensured safe handling of deleted or missing fields
- Made undo blocks mode-local and prevented redo/undo glitches
