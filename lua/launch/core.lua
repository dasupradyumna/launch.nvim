---------------------------------------- CORE FUNCTIONALITY ----------------------------------------

local user = require 'launch.user'
local util = require 'launch.util'

local M = {}

-- HACK: move to appropriate files
---@class DebugConfig
---@alias LaunchConfig TaskConfig | DebugConfig

---check if configuration is valid and substitute any user-defined config variables
---@param config LaunchConfig a run configuration object
---@return LaunchConfig?
---@nodiscard
local function check_and_substitute_vars(config)
  if not config then
    vim.api.nvim_command 'redraw'
    util.notify('I', 'No task selected')
    return
  elseif vim.tbl_isempty(user.variables) then
    return config
  end

  -- if a config is selected and user variables are defined, try substituting them
  local sub_config = vim.deepcopy(config)
  table.insert(sub_config.args, 1, sub_config.command)
  if not user.substitute_variables(sub_config.args) then return end
  sub_config.command = table.remove(sub_config.args, 1)
  return sub_config
end

---display given configs to user and execute the selection with provided runner
---@param configs LaunchConfig[] list of configurations which the user can select from
---@param run fun(config: LaunchConfig) target runner to process selected config
function M.start(configs, run)
  if not configs or #configs == 0 then
    util.notify('W', 'No tasks found')
    return
  end

  vim.ui.select(configs, {
    prompt = 'Tasks',
    format_item = function(config) return config.name end,
  }, function(config)
    local sub_config = check_and_substitute_vars(config)
    if sub_config then run(sub_config) end
  end)
end

return M
