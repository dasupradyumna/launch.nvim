------------------------------------------ CONFIG HANDLER ------------------------------------------

local task = require 'launch.task'
local user = require 'launch.user'
local util = require 'launch.util'

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
    util.notify('"launch.lua" could not be compiled', 'error')
    return
  elseif not configs then
    util.notify('"launch.lua" does not return any configs', 'error')
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
  util.notify 'Configurations updated'
end

return M
