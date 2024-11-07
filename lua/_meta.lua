------------------------------------ ANNOTATION TYPE DEFINITIONS -----------------------------------
---@meta _

---@alias LaunchNvimConfigType 'TASK' | 'DEBUG'

---@alias LaunchNvimTaskDisplayType 'float' | 'tabpage' | 'vsplit' | 'hsplit'

--------------------------- VALIDATION SPECIFICATION ---------------------------

---@alias LaunchNvimValidatorType 'dict' | 'enum' | 'list' | 'record' | type

---@class LaunchNvimValidationSpec
---@field [1] string specification name, can be a nested key sequence
---@field [2] boolean indicates whether this argument is optional, i.e. can be nil
---@field [3] LaunchNvimValidatorType data type that maps to the respective validator
---@field [4] string[]? list of strings; extra rules to the validator method

-------------------------------- PLUGIN SETTINGS -------------------------------

---@class LaunchNvimSettings
---@field confirm_choice boolean whether to confirm user choice before launch
---@field task LaunchNvimSettingsTask task section of settings
---@field debug LaunchNvimSettingsDebug debug section of settings

---@class LaunchNvimSettingsTask
---@field display LaunchNvimTaskDisplayType default task rendering type, overridable
---@field env table<string, string|number> default environment variables dictionary. overridable
---@field insert_mode_on_launch boolean whether to enter insert mode after launching task

---@class LaunchNvimSettingsDebug

---------------------------- RUNTIME CONFIGURATIONS ----------------------------

---@class LaunchNvimTaskConfig
---@field name string task name
---@field command string executable or program to be launched by this task
---@field args? string[] command-line argument list for specified command
---@field cwd? string custom working directory path, defaults to neovim CWD
---@field display? LaunchNvimTaskDisplayType controls the task output rendering
---@field env? table<string, string|number> custom task environment variables dictionary

---@class LaunchNvimDebugConfig: Configuration

---@class LaunchNvimActiveTask
