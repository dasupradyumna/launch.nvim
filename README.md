# :rocket: launch.nvim

A **task launcher** plugin for [neovim](https://github.com/neovim/neovim) allowing the user to
***dynamically*** configure tasks per project (i.e *current working directory*), inspired by the
framework in *Visual Studio Code*. 

<Add a quick video here>

<Note about the older Lua version of the plugin>

### Contents

- [Showcase](#showcase)
- [Features](#features)
- [Usage](#usage)
    - [Installation](#installation)
    - [Configuration](#configuration)
    - [Commands](#commands)
- [Contributing](#contributing)
- [License](#license)

## Showcase

## Features

- Launcher UI with **select**, **edit**, and **view** modes, each with fully tailored UI interactions, with all default keymaps removed

## Usage

### Installation

#### [lazy.nvim](https://github.com/folke/lazy.nvim)

```lua
-- LazySpec (plugin specification)
-- return {
    'dasupradyumna/launch.nvim'
-- }
```

#### [packer.nvim](https://github.com/wbthomason/packer.nvim)

```lua
-- inside setup function
-- packer.startup(function(use)
use 'dasupradyumna/launch.nvim'
-- end)
```

#### [vim-plug](https://github.com/junegunn/vim-plug)

```vim
Plug 'dasupradyumna/launch.nvim'
```

### Configuration

### Commands

- **LaunchConfig** : Opens a launcher UI displaying all available task configurations, which allows
    the user to select a configuration to perform actions on (i.e. *launch*, *edit*). Refer to the
    [Wiki](https://github.com/dasupradyumna/launch.nvim/wiki/Launcher-Interface) for more details on
    the various launcher modes and their behaviors.

- **LaunchListActiveTasks** : Opens a UI window displaying all active tasks, which allows the user to
    view or relaunch them.

## Contributing

Any ideas for new *features* and *quality-of-life changes* that you wish to see in this plugin or
its documentation are welcome. Please feel free to open an issue or even start a discussion
regarding your requirement. And as always, all PRs are welcome! (*preferably* after the new feature
has been discussed in a post)

## License

**launch.nvim** is licensed under the *GNU General Public License 3.0*.

### More plugins by Author

[midnight.nvim](https://github.com/dasupradyumna/midnight.nvim) :crescent_moon:
A modern black neovim theme written in Lua *(colorscheme used in the demo)*
