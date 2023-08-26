------------------------------------------ CONFIG HANDLER ------------------------------------------

local task = require 'launch.task'
local user = require 'launch.user'
local util = require 'launch.util'

local UserVariable = require 'launch.class.UserVariable'

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
    -- FIX: add a link to the tasks schema (to-be-added) in error message
    util.notify('err', '"launch.lua" could not be compiled')
    return
  elseif not configs then
    util.notify('err', '"launch.lua" does not return any configs')
    return
  end

  -- load all the configured tasks
  for _, config in ipairs(configs) do
    ---@cast config TaskConfigFromUser
    local filetype = config.type or 'none'
    config.type = nil

    task.list[filetype] = task.list[filetype] or {}
    table.insert(task.list[filetype], config)
  end

  -- load all user-defined variables
  local ok = pcall(function()
    for name, var in pairs(configs.input) do
      configs.input[name] = UserVariable:new(name, var)
    end
  end)
  if not ok then return end
  user.variables = configs.input

  vim.cmd.redraw()
  util.notify('info', 'Configurations updated')
end

return M
