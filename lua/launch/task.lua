------------------------------------------- TASK RUNNER --------------------------------------------

local ActiveTask = require 'launch.types.ActiveTask'

local api = vim.api

local M = {}

---@type table<string, TaskConfig[]> a list of task configurations per filetype
M.list = {}

---@type table<integer, ActiveTask> a mapping of currently active tasks by their job IDs
M.active = {}

---@type { tab: integer, float: integer }
M.handles = {}

---launches a task specified by the given configuration
---@param config TaskConfig
function M.runner(config)
  if not M.handles.tab then
    api.nvim_set_var('disable_new_tab_name_prompt', true) -- HACK: remove this
    api.nvim_command 'tabnew'
    M.handles.tab = api.nvim_get_current_tabpage()
    api.nvim_tabpage_set_var(0, 'tabname', 'TaskRunner') -- HACK: remove this
  end
  api.nvim_set_current_tabpage(M.handles.tab)

  local task = ActiveTask.new(config)
  task:run()

  M.active[task.channel] = task
  api.nvim_create_autocmd('BufWipeout', {
    desc = 'Remove current task from active task list when its buffer is deleted',
    callback = function() M.active[task.channel] = nil end,
    buffer = task.buffer,
    group = 'launch_nvim',
  })
end

return M
