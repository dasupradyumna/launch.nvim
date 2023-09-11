---------------------------------------- CORE FUNCTIONALITY ----------------------------------------

local user = require 'launch.user'
local util = require 'launch.util'

local ConfigFromFile = require 'launch.types.ConfigFromFile'

local M = {}

---@type table<string, { no_configs: string, no_selection: string, prompt: string }>
local messages = {
  debug = {
    no_configs = 'No debug configurations found',
    no_selection = 'No debug configuration selected',
    prompt = 'Debug Configurations',
  },
  task = {
    no_configs = 'No tasks found',
    no_selection = 'No task selected',
    prompt = 'Tasks',
  },
}

---@alias LaunchConfig TaskConfig | DebugConfig

---check if configuration is valid and substitute any user-defined config variables
---@param type 'debug' | 'task' whether the target is a debug or a task configuration
---@param config LaunchConfig a run configuration object
---@return LaunchConfig?
---@nodiscard
local function check_and_substitute_vars(type, config)
  if not config then
    vim.api.nvim_command 'redraw'
    util.notify('I', messages[type].no_selection)
    return
  elseif vim.tbl_isempty(user.variables) then
    return config
  end

  -- if a config is selected and user variables are defined, try substituting them
  local target = type == 'task' and 'command' or 'program'
  local sub_config = vim.deepcopy(config)
  table.insert(sub_config.args, 1, sub_config[target])
  if not user.substitute_variables(sub_config.args) then return end
  sub_config[target] = table.remove(sub_config.args, 1)
  return sub_config
end

---get and display a list of valid configurations and execute the one that the user selects
---@param type 'debug' | 'task' whether the target is a debug or a task configuration
---@param show_all_fts boolean whether to display all configs or only based on current filetype
---@param all_configs table<string, LaunchConfig[]> list of configurations for user to select from
---@param run fun(config: LaunchConfig) target runner to process selected config
function M.start(type, show_all_fts, all_configs, run)
  local configs
  if show_all_fts then
    configs = {}
    for _, ft_configs in pairs(all_configs) do
      vim.list_extend(configs, ft_configs)
    end
  else
    local filetype = vim.api.nvim_get_option_value('filetype', { buf = 0 })
    configs = all_configs[filetype]
  end

  -- skip with warning message if no configurations are available
  if not configs or #configs == 0 then
    util.notify('W', messages[type].no_configs)
    return
  end

  -- get user selection from the available configs
  vim.ui.select(configs, {
    prompt = messages[type].prompt,
    format_item = function(config) return config.name end,
  }, function(config)
    local sub_config = check_and_substitute_vars(type, config)
    if sub_config then
      vim.api.nvim_command 'redraw' -- clean up any messages before running config
      run(sub_config)
    end
  end)
end

---updates the runtime config list from the corresponding config file on disk
function M.load_config_file()
  local user_tasks = '.nvim/launch.lua'
  if vim.fn.filereadable(user_tasks) ~= 1 then return end

  local ok, configs = pcall(dofile, user_tasks)
  if not ok then
    -- FIX: add a link to the tasks schema (to-be-added) in error message
    util.notify('E', '"launch.lua" could not be compiled; Please check')
    return
  end

  ok = pcall(function() ConfigFromFile:load(configs) end)
  if not ok then return end

  util.notify('I', 'User configurations loaded')
end

return M
