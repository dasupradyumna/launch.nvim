------------------------------------ ANNOTATION TYPE DEFINITIONS -----------------------------------
---@meta _

---@alias LaunchNvimConfigType 'TASK' | 'DEBUG'

--------------------------- VALIDATION SPECIFICATION ---------------------------

---@class LaunchNvimValidationSpec
---@field [1] string specification name, can be a nested key sequence
---@field [2] boolean indicates whether this argument is optional, i.e. can be nil
---@field [3] LaunchNvimValidatorType data type that maps to the respective validator
---@field [4] string[]? list of strings; extra rules to the validator method

---@alias LaunchNvimValidatorType 'dict' | 'enum' | 'list' | 'record' | type

-------------------------------- PLUGIN SETTINGS -------------------------------

---@class LaunchNvimSettings
---@field confirm_choice boolean whether to confirm user choice before launch
---@field task LaunchNvimSettingsTask task section of settings
---@field debug LaunchNvimSettingsDebug debug section of settings

---@class LaunchNvimSettingsTask
---@field ui LaunchNvimSettingsTaskUI task UI rendering settings
---@field env table<string, string|number> default environment variables dictionary. overridable
---@field insert_mode_on_launch boolean whether to enter insert mode after launching task

---@class LaunchNvimSettingsTaskUI
---@field display LaunchNvimTaskDisplayType default task rendering type, overridable
---@field float LaunchNvimSettingsTaskUIFloat settings for task UI in float mode
---@field hsplit_height integer percentage of screen height for horizontal split (if possible)
---@field vsplit_width integer percentage of screen width for vertical split (if possible)

---@alias LaunchNvimTaskDisplayType 'float' | 'hsplit' | 'vsplit'

---@class LaunchNvimSettingsTaskUIFloat
---@field size LaunchNvimSettingsTaskFloatSize size of the task UI float
---@field config vim.api.keyset.win_config neovim floating window configuration

---@alias LaunchNvimSettingsTaskFloatSize 'small' | 'medium' | 'large'

---@class LaunchNvimSettingsDebug

---------------------------- RUNTIME CONFIGURATIONS ----------------------------

---@class LaunchNvimTaskConfig
---@field name string task name
---@field command string executable or program to be launched by this task
---@field args? string[] command-line argument list for specified command
---@field cwd? string custom working directory path, defaults to neovim CWD
---@field display? LaunchNvimTaskDisplayType controls the task output rendering
---@field env? table<string, string|number> custom task environment variables dictionary

---@class LaunchNvimActiveTaskConfig: LaunchNvimTaskConfig
---@field cwd string custom working directory path, defaults to neovim CWD
---@field display LaunchNvimTaskDisplayType controls the task output rendering
---@field env table<string, string|number> custom task environment variables dictionary

---@class LaunchNvimActiveTask
---@field title string title for current active task
---@field buffer integer task buffer ID
---@field config LaunchNvimActiveTaskConfig final configuration used to launch this task
---@field spawn_time integer task spawn UNIX timestamp (in ms)
---@field exit_time integer task exit UNIX timestamp (in ms)

---@class LaunchNvimDebugConfig: Configuration
