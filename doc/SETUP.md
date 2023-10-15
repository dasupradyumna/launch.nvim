# Setting up `launch.nvim`

This document contains detailed information about fields in the configuration table which require
more context for a proper understanding.

## `task` field in `setup()` config

#### Overridable defaults : `display` and `options`

The `display` and `options` fields in the `task` configuration table are global defaults which apply
to *every task in all neovim instances*. They obey the same format as the `display` and `options`
fields under [Task Configuration](https://github.com/dasupradyumna/launch.nvim#task-configuration).

For every task, the complete configuration is constructed by using the *global defaults* and merging
them with the values specified in the *individual task configuration* table recursively, with the
values from the individual config *overriding* the defaults when in conflict. Finally, this
constructed configuration is used to launch the task.

## `debug` field in `setup()` config

Shown below are some example debug configurations for C++ and Python debuggers

```lua
-- launch.lua
return {
    debug = {
        {
            type = 'cppdbg',
            name = 'Debug C++ Tests',
            request = 'launch',
            program = 'bin/test',
            cwd = '~/CppProjects/CLIApp',
            stopAtEntry = true,
            externalConsole = true,
            MIMode = 'gdb',
            miDebuggerPath = '/usr/bin/gdb',
        },
        {
            type = 'cppdbg',
            name = 'Debug C++ Binary',
            request = 'launch',
            program = 'bin/final_exe',
            args = { '-o', 'fast' },
            cwd = '~/CppProjects/CLIApp',
            stopAtEntry = true,
            externalConsole = true,
            MIMode = 'gdb',
            miDebuggerPath = '/usr/bin/gdb',
        },
        {
            type = 'debugpy',
            name = 'Debug PyServer',
            request = 'launch',
            program = 'src/main.py',
            args = { '127.16.5.82:1000' },
            console = 'integratedTerminal',
            cwd = '~/PythonProjects/PyServer',
            pythonPath = '~/miniconda3/envs/server/bin/python',
            stopOnEntry = true,
            justMyCode = false,
            showReturnValue = true,
        }
    }
}
```

Here, the adapters used are **cppdbg** for C++ and **debugpy** for Python files.  
An example adapter configuration using **nvim-dap** could be

```lua
local dap = require 'dap'
dap.adapters.cppdbg = { --[[ adapter configuration ]] }
dap.adapters.debugpy = { --[[ adapter configuration ]] }
```

#### 1. **adapters** `table<string, string>`

This option tells **launch.nvim** the names of the adapters that the user has configured for
**nvim-dap**.  
The user then can conveniently specify the *more natural* filetype instead of the **nvim-dap**
configured adapter name, by adding the following in the `setup()` function -

```lua
---- In the plugin specification or setup 
require('launch').setup { debug = {
    -- keys are filetypes, values are nvim-dap configured adapter names
    adapters = { cpp = 'cppdbg', python = 'debugpy' }
} }

-----------------------------------------------------------------------------------------
---- Now, the configurations in launch.lua can look like this
return {
    debug = {
        {   -- `type` field can be skipped now
            -- "launch.nvim" will automatically interpret it from the filetype
            filetype = 'cpp',
            name = 'Debug C++ Tests',
            ...
        },
        {
            filetype = 'cpp',
            name = 'Debug C++ Binary',
            ...
        },
        {
            filetype = 'python',
            name = 'Debug PyServer',
            ...
        },
    }
}
```

#### 2. **templates** `table<string, table>`

Debuggers typically allow a ton of configuration options, but they do not provide a way to specify
certain options to apply to every configuration. Thus, when a user has certain option preferences
that are common to every configuration for a particular adapter, the debug configurations tend to
become very verbose and redundant, as seen in the above example `launch.lua`.  
Users can make their debug configurations concise and less error-prone by letting **launch.nvim**
know of these common options, by adding the following in the `setup()` function -

```lua
---- In the plugin specification or setup 
require('launch').setup { debug = {
    -- keys are filetypes, values are common options for the respective debug adapters
    templates = {
        cpp = {
            request = 'launch',
            stopAtEntry = true,
            externalConsole = true,
            MIMode = 'gdb',
            miDebuggerPath = '/usr/bin/gdb',
        },
        python = {
            request = 'launch',
            console = 'integratedTerminal',
            stopOnEntry = true,
            justMyCode = false,
            showReturnValue = true,
        },
    }
} }

-----------------------------------------------------------------------------------------
---- Now, the configurations in launch.lua can look like this
return {
    debug = {
        {
            type = 'cppdbg',
            name = 'Debug C++ Tests',
            program = 'bin/test',
            cwd = '~/CppProjects/CLIApp',
        },
        {
            type = 'cppdbg',
            name = 'Debug C++ Binary',
            program = 'bin/final_exe',
            args = { '-o', 'fast' },
            cwd = '~/CppProjects/CLIApp',
        },
        {
            type = 'debugpy',
            name = 'Debug PyServer',
            program = 'src/main.py',
            args = { '127.16.5.82:1000' },
            cwd = '~/PythonProjects/PyServer',
            pythonPath = '~/miniconda3/envs/server/bin/python',
        }
    }
}
```

Essentially, **launch.nvim** merges a debug configuration table with the template for its filetype
to produce the complete configuration which is passed to `dap.run()` for launching the debugger.
