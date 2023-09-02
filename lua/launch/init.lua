------------------------------------------- LAUNCH.NVIM --------------------------------------------

local core = require 'launch.core'
local task = require 'launch.task'

local M = {}

function M.setup(config)
  -- task configuration
  ---- behavior when task is invoked more than once (replace old window or unique name)
  ---- default tasks display behavior (tabpage or floating window)
  ---- custom task shell options
  -- custom task and/or debug runner functions
  -- optional DAP default template for each filetype (for smaller config files)
  -- config location: per directory OR stdpath('data')
  -- whether to automatically enter insert mode after launching task
end

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

  core.start(tasks, task.runner)
end

return M
