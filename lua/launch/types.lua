--------------------------------------- TYPE SPECIFICATIONS ----------------------------------------
---@meta

------------------ CONFIG TYPES ------------------

---@alias RunConfig TaskConfig | DebugConfig

---@class TaskConfig
---@field name string display name of the task
---@field command string command to be executed: can be an executable or a shell command
---@field args string[]? command-line arguments that follow the command
---@field display DisplayType whether to render the task output in a tabpage or a floating window
---@field options TaskOptions? additional options configuring how the task is run

---@alias DisplayType 'float' | 'tab' | nil

---@class TaskConfigFromUser : TaskConfig
---@field type string? filetype of the programming language for the task ('none', by default)

---@class TaskOptions
---@field cwd string? current working directory of the shell which runs the task
---@field env table<string, string>? environment variables to set in shell before running task
---@field shell ShellOptions? optional shell config to use for running task
-- all above options (when specified) override the default shell environment

---@class ShellOptions
---@field exec string path to the binary shell executable
---@field args string[]? command-line arguments to the shell executable

---@class DebugConfig

----------------- VARIABLE TYPES -----------------

---@class UserVariable
---@field type UserVarType user input method; either entered at a prompt or selected from a list
---@field desc string a description for the input variable which is displayed in user input prompt
---@field default UserVarValue a default value for an 'input' type variable (ignored for 'select')
---@field items UserVarValue[] a list of choices for a 'select' type variable (ignored for 'input')

---@alias UserVarType 'input' | 'select'
---@alias UserVarValue boolean | string | number
