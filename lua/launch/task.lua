------------------------------------------- TASK RUNNER --------------------------------------------

local ActiveTask = require 'launch.types.ActiveTask'

local api = vim.api

local M = {}

---@type table<string, TaskConfig[]> a list of task configurations per filetype
M.list = {}

---@type table<integer, ActiveTask> a mapping of currently active tasks by their buffers
M.active = {}

---launches a task specified by the given configuration
---@param cfg TaskConfig
function M.runner(cfg)
  local task = ActiveTask:new(cfg)
  task:render(cfg.display)
  task:run()

  M.active[task.buffer] = task
  api.nvim_create_autocmd('BufWipeout', {
    desc = 'Remove current task from active task list when its buffer is deleted',
    callback = function() M.active[task.buffer] = nil end,
    buffer = task.buffer,
    group = 'launch_nvim',
  })
  vim.keymap.set('n', 'q', '<Cmd>q<CR>', { buffer = task.buffer })
end

return M
