------------------------------------------ CONFIG HANDLER ------------------------------------------

local task = require 'launch.task'

local M = {}

function M.update_configs()
  local user_tasks = '.nvim/launch.lua'
  if vim.fn.filereadable(user_tasks) ~= 1 then return end

  local success, configs = pcall(dofile, user_tasks)
  if not (success and configs) then
    -- FIX: add a link to the tasks schema in the repo (to-be-added)
    vim.notify(
      '[launch.nvim] $PWD/.nvim/launch.lua has invalid configurations',
      vim.log.levels.ERROR
    )
    return
  end

  task.list = {} -- reset the list of tasks
  for _, config in
    ipairs(configs --[[@as TaskConfigFromUser[] ]])
  do
    local filetype = config.type or '<none>'
    config.type = nil

    task.list[filetype] = task.list[filetype] or {}
    table.insert(task.list[filetype], config)
  end

  vim.cmd.redraw()
  vim.notify '[launch.nvim] Configurations updated'
end

return M
