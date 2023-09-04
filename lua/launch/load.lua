------------------------------------------ CONFIG HANDLER ------------------------------------------

local task = require 'launch.task'
local user = require 'launch.user'
local util = require 'launch.util'

local UserVariable = require 'launch.types.UserVariable'
local TaskConfig = require 'launch.types.TaskConfig'

local M = {}

---updates the runtime config list from the corresponding config file on disk
function M.load_config_list()
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

  -- TODO: perform validation of the `configs` table
  -- load all the configured tasks
  for _, cfg in ipairs(configs) do
    local ok, filetype, config = pcall(TaskConfig.new, cfg --[[@as TaskConfigFromFile]])
    if not ok then return end

    task.list[filetype] = task.list[filetype] or {}
    table.insert(task.list[filetype], config)
  end

  -- load all user-defined variables
  local ok = pcall(function()
    for name, var in pairs(configs.input) do
      configs.input[name] = UserVariable.new(name, var)
    end
  end)
  if not ok then return end
  user.variables = configs.input

  vim.api.nvim_command 'redraw'
  util.notify('info', 'Configurations updated')
end

return M
