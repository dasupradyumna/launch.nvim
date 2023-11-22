# :rocket: launch.nvim

A **task launcher** plugin for [neovim](https://github.com/neovim/neovim) which allows
***dynamically*** configuring tasks per project (i.e *current working directory*), inspired by the
framework in *Visual Studio Code*. It also provides (optional) support for debugging via
[nvim-dap](https://github.com/mfussenegger/nvim-dap), a debug adapter protocol implementation for
neovim.

### Contents

- [Demo](#demo)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Setup](#setup)
  - [Commands](#commands)
- [Schemas](#schemas)
  - [Task Configuration](#task-configuration)
  - [Debug Configuration](#debug-configuration)
  - [User Variables](#user-variables)
- [Contributing](#contributing)

## Demo

This short 3 minute demo video covers all major features and commands offered by **launch.nvim**,
starting from an empty project and a new configuration file.  
*(This demo shows the config file using the old path; the new path does not pollute the project root
anymore)*

<https://github.com/dasupradyumna/launch.nvim/assets/45595032/8574ffe8-d37f-4d43-b83f-e9f611f6a2f2>

Although debugger-related configurations and commands are not covered in the above demo, they have
*exactly the same* UI and behavior as the task-related commands.

## Features

- Create custom tasks for **every working directory** using the file opened by
    *`LaunchOpenConfigFile`* command  
    *All configurations in this file are **hot-reloaded** upon saving changes*
- Configured tasks can be launched in a **tabpage** or a **floating window**, managed by the plugin
- Closing the plugin-managed *tabpage* or *floating window* **will not kill** the current task(s);
    they will continue to run in the background
- **Already launched tasks** (*active tasks*) can be reopened in a *tabpage* or a *floating window*,
    as long as the process is *still running* or the terminal buffer is *not closed*
- Create debugger configurations and launch debug sessions using **nvim-dap** plugin *(can be
    disabled)*  
    *UI elements for **nvim-dap** are not handled by **launch.nvim**; please refer to plugins like
    [nvim-dap-ui](https://github.com/rcarriga/nvim-dap-ui)*
- Create **custom placeholder variables** which can be used in both **task** and **debug**
    configurations; they will be substituted with user input *at runtime* when a config is launched

***NOTE :** Every floating window created by the plugin can be closed by pressing the **`q`** key*

## Installation

##### Requirements

- Neovim 0.9+ (*nightly recommended*)
- [nvim-dap](https://github.com/mfussenegger/nvim-dap) for debugging support (*optional*)
- Decorator plugins that provide customization for `vim.notify` (*optional*)  
    *Currently supports [nvim-notify](https://github.com/rcarriga/nvim-notify) and
    [noice.nvim](https://github.com/folke/noice.nvim/)*

#### [lazy.nvim](https://github.com/folke/lazy.nvim)

```lua
-- LazySpec (plugin specification)
-- return {
{
    'dasupradyumna/launch.nvim',
    -- add below plugins as per user requirement
    dependencies = {
        'mfussenegger/nvim-dap',
        'rcarriga/nvim-notify',
    }
}
-- }
```

#### [packer.nvim](https://github.com/wbthomason/packer.nvim)

```lua
-- inside setup function
-- packer.startup(function(use)
use {
    'dasupradyumna/launch.nvim',
    -- add below plugins as per user requirement
    requires = {
        'mfussenegger/nvim-dap',
        'rcarriga/nvim-notify',
    }
}
-- end)
```

#### [vim-plug](https://github.com/junegunn/vim-plug)

```vim
" add below dependencies as per user requirement
Plug 'mfussenegger/nvim-dap'
Plug 'rcarriga/nvim-notify'

Plug 'dasupradyumna/launch.nvim'
```

## Usage

### Setup

The main **setup** function can be called *independently* in your config or as a part of the plugin
specification depending on the plugin manager that you are using. The plugin sets *very sensible*
defaults so you can pass an empty configuration table to the setup to check it out.  
*(The demo video shows the plugin being run on default setup options)*

```lua
-- table of user-defined configuration options which override the plugin defaults
local cfg = {} -- uses the plugin defaults

-- independent setup (for vim-plug as well)
require('launch').setup(cfg)

-- for lazy.nvim
return { 'dasupradyumna/launch.nvim', opts = cfg }

-- for packer.nvim
use { 'dasupradyumna/launch.nvim', config = function() require('launch').setup(cfg) end}
```

The plugin sets the following default options -

```lua
-- PLUGIN DEFAULTS
default_cfg = {
  -- debugger settings
  debug = {
    -- mapping from filetypes to debug adapter names as specified in `require('dap').adapters`
    -- `nil` implies that the filetypes themselves are used as the adapter names
    adapters = nil, ---@type table<string, string>

    -- disable all debugger related functionality
    disable = false, ---@type boolean

    -- custom debugger launcher function which receives the selected debug configuration as an
    -- argument; `nil` implies `require('dap').run` is used by default
    -- NOTE : users should ignore this unless they know what they are doing
    runner = nil, ---@type function

    -- table containing debug configuration template per filetype
    templates = nil,
  },

  -- task runner settings
  task = {
    -- whether to render the task output in a tabpage or a floating window, by default
    display = 'float', ---@type 'float' | 'tab'

    -- configuration options for floating window, see {config} in `:h nvim_open_win()`
    float_config = {
      relative = 'editor',
      border = 'rounded',
      title_pos = 'center',
      style = 'minimal',
    },

    -- custom user functions which will be executed before and/or after creating a floating
    -- window or a tabpage for a newly launched task
    hooks = {
      -- floating window hooks
      float = {
        pre = nil, ---@type function
        post = nil, ---@type function
      },
      -- tabpage hooks
      tab = {
        pre = nil, ---@type function
        post = nil, ---@type function
      },
    },

    -- whether to enter INSERT mode after launching task in a buffer
    insert_on_launch = false, ---@type boolean

    -- same fields as `TaskOptions` in "Task Configuration" subsection
    options = {
      -- set the default current working directory for all tasks
      cwd = nil, ---@type string|fun():string

      -- table with definitions of environment variables to be set for all tasks
      env = nil, ---@type table<string, string|number>

      -- table containing executable and command-line arguments to launch a shell process
      shell = nil, ---@type { exec: string, args: string[] }
    },

    -- custom task launcher function which receives the selected task configuration as an
    -- argument; `nil` implies `require('launch.task').runner` is used by default
    -- NOTE : users should ignore this unless they know what they are doing
    runner = nil, ---@type function

    -- config options for opening task in a terminal instance; see {opts} in `:h jobstart()`
    term = {
      clear_env = false,
    },
  },
}
```

For further details about the configuration table and its fields, refer to [SETUP.md](doc/SETUP.md).

### Commands

- **LaunchTask**  
    Show the list of all configured tasks for the current working directory and launch the selected
    task

- **LaunchTaskFT**  
    Show the list of configured tasks filtered based on the current buffer filetype and launch the
    selected task

- **LaunchShowTaskConfigs**  
    Show all configured tasks with their options in a floating window

- **LaunchShowTaskConfigsFT**  
    Show configured tasks filtered based on the current buffer filetype with their options in a
    floating window

- **LaunchShowActiveTasks**  
    Show the list of all active tasks in a floating window; each active task can be displayed
    either in a floating window or a new window in the plugin-managed tabpage  
    `<C-T>` opens the active task under the cursor in the tabpage, `<C-F>` opens it in a floating
    window whereas `<CR>` opens it based on `display` option in its configuration  
    *An active task is one which still has a running process or finished execution but its terminal
    buffer is still open*

- **LaunchDebugger**  
    Show the list of all debug configurations for the current working directory and launch selected
    config  
    *This command will not be available if debug support is disabled during plugin setup*

- **LaunchDebuggerFT**  
    Show the list of debug configurations filtered based on the current buffer filetype and launch
    selected config  
    *This command will not be available if debug support is disabled during plugin setup*

- **LaunchShowDebugConfigs**  
    Show all debug configurations with their options in a floating window  
    *This command will not be available if debug support is disabled during plugin setup*

- **LaunchShowDebugConfigsFT**  
    Show debug configurations filtered based on the current buffer filetype with their options in a
    floating window  
    *This command will not be available if debug support is disabled during plugin setup*

- **LaunchShowUserVariables**  
    Show all defined user variables with their specifications in a floating window

- **LaunchOpenConfigFile**  
    Open the current working directory's launch configuration file in a plugin-managed floating
    window

## Schemas

The plugin configuration file should return a table with one or more of the following 3 fields:
**task**, **debug** and **var**. **task** and **debug** should be array-like tables of task and
debug configurations respectively, and **var** should be a dictionary-like table of user variable
definitions with the key being a variable's name and the value being its config.

Every field is *optional*, and can be omitted if no configurations need to be specified. If the
plugin configuration file returns `nil` or an empty table `{}`; all task, debug and variable
configurations are cleared. (*If the user variable syntax is used in any configuration, then the
corresponding variable definition should be specified under the **var** field*)

```lua
-- config file
return {
    task = {
        { --[[ TaskConfig1 ]] },
        { --[[ TaskConfig2 ]] },
    },
    debug = {
        { --[[ DebugConfig1 ]] },
        { --[[ DebugConfig2 ]] },
    },
    var = {
        InputVar1 = { --[[ InputVarConfig1 ]] },
        InputVar2 = { --[[ InputVarConfig2 ]] },
    },
}
```

***NOTE :**
The plugin will issue error notifications if the user makes any syntax errors while writing
configurations for any field. Open an issue if you notice gaps in the syntax checker's logic.*

### Task Configuration

A task configuration can have the following structure

```lua
local task_config = {
    name = '<config_name>',
    command = '<config_command>',
    args = { '<command_arg1>', '<command_arg2>' },
    display = 'float',
    options = {
        cwd = '<path_to_custom_cwd>', -- OR function() return <path_to_custom_cwd> end,
        env = {
            STRING_VAR = 'hello_world',
            NUMERIC_VAR = 42.42,
        },
        shell = {
            exec = '<shell_executable>',
            args = { '<shell_arg1>', '<shell_arg2>' },
        }
    },
}

return { task = { task_config } }
```

1. **name** *(required)* `string`  
    Specifies the name of the task configuration which will be displayed by `LaunchTask(FT)` command

2. **filetype** `string`  
    Optionally specify the filetype into which the current task is grouped, which is used by the
    `LaunchTaskFT` and `LaunchShowTaskConfigsFT` commands to filter tasks based on current buffer
    filetype

3. **command** *(required)* `string`  
    Specifies the executable or program to execute in a new task instance

4. **args** `string[]`  
    List of command-line arguments fed to the executable or program specified by **command**  
    Each argument in the list will be concatenated with a space character

5. **display** `'float' | 'tab'`  
    Specifies whether the task instance should be rendered in a floating window or plugin-managed
    tabpage by default when launched  
    If not specified, the value provided in the **setup()** configuration is taken by default

6. **options** `TaskOptions`  
    Additional options to customize the environment in which the task is run  
    If not specified, the value provided in the **setup()** configuration is taken by default

    1. **TaskOptions.cwd** `string | fun():string`  
        Path (*absolute or relative*) to the custom directory to be set as the current working
        directory for the task  
        This path can be specified directly as a *string* or via a *function* which returns a string

    2. **TaskOptions.env** `table<string, string | number>`  
        Dictionary of specifications for environment variables to be defined before running the task

    3. **TaskOptions.shell** `ShellOptions`  
        Options to customize the shell used for launching the task

        - **ShellOptions.exec** *(required)* `string`  
            Path to the custom shell executable (must be specified if `shell` is not `nil`)

        - **ShellOptions.args** `string[]`  
            List of command-line arguments fed to the custom shell executable  
            Each argument in the list will be concatenated with a space character

### Debug Configuration

A debug configuration (compatible with **nvim-dap**) requires 3 compulsory fields and other
additional fields that may depend on the actual debugger on a case-by-case basis. More information
about valid configuration fields can be found
[here](https://code.visualstudio.com/Docs/editor/debugging#_launchjson-attributes) and
[here](https://github.com/mfussenegger/nvim-dap/wiki/Debug-Adapter-installation).

```lua
local debug_config = {
    type = '<adapter_name>',
    -- OR
    -- filetype = '<target_ft>', -- filetype-adapter mapping can be specified in `setup()`

    request = 'launch',
    name = '<config_name>',

    -- Additional debugger-specific fields
    -- field1 = value1,
    -- field2 = value2,
}

return { debug = { debug_config } }
```

### User Variables

An example user-defined variable specification can look like this -

```lua
return {
    task = {
        {
            name = 'Test User Variables',
            command = 'echo',
            args = { '{@select_type}', '{@input_type}' },
        }
    },
    var = {
        select_type = {
            type = 'select',
            desc = 'selection type variable',
            items = { '<item1>', '<item2>' }
        },
        input_type = {
            type = 'input',
            desc = 'input type variable',
            default = '<default_value>',
        },
    }
}
```

Defined user variables can be used in both task and debug configurations with the following syntax
`{@<variable_name>}`, in `command` and `args` fields. When a task using this syntax is launched, it
will prompt a choice from the user, substitute the syntax with the user's choice and then launches
the task. *(this will be supported in all string fields in the future)*

1. **type** *(required)* `'select' | 'input'`  
    Describes whether user is prompted for input from keyboard or select from a list of choices

2. **desc** *(required)* `string`  
    A brief description for the user variable, displayed when user input is required

3. **default** `string | number | boolean`  
    Default value already filled in when user is prompted for keyboard input  
    *Only valid for **input** type variable*

4. **items** `(string | number | boolean)[]`  
    A list of choices for the user to select for substitution; values can be a string, a number or a
    boolean value  
    *Only valid for **select** type variable*

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
