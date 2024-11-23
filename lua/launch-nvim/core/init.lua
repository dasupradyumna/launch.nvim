---------------------------------------- CORE FUNCTIONALITY ----------------------------------------

local task = require 'launch-nvim.core.task'

local M = {}

---@type LaunchNvimActiveTask[] reference to original list of active tasks
M.active_tasks = task.active

---run the logic required to launch the specified config type
---@param config_type LaunchNvimConfigType config type
---@param config LaunchNvimTaskConfig | LaunchNvimDebugConfig selected config to launch
function M:run(config_type, config)
  if config_type == 'TASK' then
    task:run(config --[[@as LaunchNvimTaskConfig]])
  elseif config_type == 'DEBUG' then
    require('dap').run(config --[[@as LaunchNvimDebugConfig]])
  end
end

return M
