------------------------------------------- LAUNCH.NVIM --------------------------------------------

local config = require 'launch.config'
local core = require 'launch.core'
local task = require 'launch.task'

local M = {}

---plugin setup function
---@param opts Config?
function M.setup(opts) config.apply(opts) end

---displays available tasks to the user and launches the selected task
---@param all_tasks boolean whether to display all tasks or only tasks based on current filetype
function M.task(all_tasks)
  local tasks

  if all_tasks then
    tasks = {}
    for _, task_list in pairs(task.list) do
      for _, t in ipairs(task_list) do
        table.insert(tasks, t)
      end
    end
  else
    local filetype = vim.api.nvim_get_option_value('filetype', { buf = 0 })
    tasks = task.list[filetype]
  end

  core.start(tasks, config.user.task.runner or task.runner)
end

return M
