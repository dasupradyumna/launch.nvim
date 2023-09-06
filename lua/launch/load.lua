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
    util.notify('E', '"launch.lua" could not be compiled')
    return
  elseif type(configs) ~= 'table' then
    util.notify('E', '"launch.lua" should return a table of configurations\n    Got: %s', configs)
    return
  end

  -- ensure that configs is a list of configurations
  local input = configs.input
  configs.input = nil
  if not vim.tbl_islist(configs) or vim.tbl_isempty(configs) then
    util.notify('E', '"launch.lua" should return a non-empty list-like table of configurations')
    return
  end

  -- load all the configured tasks
  for _, cfg in ipairs(configs) do
    local ok, filetype, config = pcall(TaskConfig.new, cfg --[[@as TaskConfigFromFile]])
    if not ok then return end

    task.list[filetype] = task.list[filetype] or {}
    table.insert(task.list[filetype], config)
  end

  -- load all user-defined variables
  local ok = pcall(function()
    for name, var in pairs(input) do
      input[name] = UserVariable.new(name, var)
    end
  end)
  if not ok then return end
  user.variables = input

  vim.api.nvim_command 'redraw'
  util.notify('I', 'Configurations updated')
end

return M
