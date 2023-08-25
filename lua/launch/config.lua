------------------------------------------ CONFIG HANDLER ------------------------------------------

local task = require 'launch.task'
local user = require 'launch.user'

local M = {}

---updates the runtime config list from the corresponding config file on disk
function M.update_config_list()
  -- reset the list of tasks and user variables
  task.list = {}
  user.variables = {}

  local user_tasks = '.nvim/launch.lua'
  if vim.fn.filereadable(user_tasks) ~= 1 then return end
  local success, configs = pcall(dofile, user_tasks)
  if not success then
    -- FIX: add a link to the tasks schema in the repo (to-be-added)
    vim.notify('[launch.nvim] "launch.lua" could not be compiled', vim.log.levels.ERROR)
    return
  elseif not configs then
    vim.notify('[launch.nvim] "launch.lua" does not return any configs', vim.log.levels.ERROR)
    return
  end

  -- load all the configured tasks
  for _, config in
    ipairs(configs --[=[@as TaskConfigFromUser[]]=])
  do
    local filetype = config.type or 'none'
    config.type = nil

    task.list[filetype] = task.list[filetype] or {}
    table.insert(task.list[filetype], config)
  end

  user.variables = configs.input -- load all user-defined variables

  vim.cmd.redraw()
  vim.notify '[launch.nvim] Configurations updated'
end

return M
