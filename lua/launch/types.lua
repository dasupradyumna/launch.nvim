--------------------------------------- TYPE SPECIFICATIONS ----------------------------------------
---@meta

---@class RunConfig
---@field name string

---@class TaskConfig : RunConfig
---@field command string command to be executed: can be an executable or a shell command
---@field args string[]? command-line arguments that follow the command
---@field display 'tabpage' | 'float'
---@field options TaskOptions? additional options configuring how the task is run

---@class TaskConfigFromUser : TaskConfig
---@field type string specific filetype of the programming language for the task

---@class TaskOptions
---@field cwd string? current working directory of the shell which runs the task
---@field env table<string, string>? environment variables to set in shell before running task
---@field shell ShellOptions? optional shell config to use for running task
-- all above options (when specified) override the default shell environment

---@class ShellOptions
---@field exec string path to the binary shell executable
---@field args string[]? command-line arguments to the shell executable
