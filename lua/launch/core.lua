---------------------------------------- CORE FUNCTIONALITY ----------------------------------------

local user = require 'launch.user'

local M = {}

---check if configuration is valid and substitute any user-defined config variables
---@param config RunConfig a run configuration object
---@return RunConfig?
---@nodiscard
local function check_and_substitute_vars(config)
  if not config then
    vim.cmd.redraw()
    vim.notify '[launch.nvim] No task selected'
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
---@param configs RunConfig[] list of configurations which the user can select from
---@param run fun(config: RunConfig) target runner to process selected config
function M.start(configs, run)
  if not configs or #configs == 0 then
    vim.notify('[launch.nvim] No tasks found', vim.log.levels.WARN)
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
