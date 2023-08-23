------------------------------------------- TASK RUNNER --------------------------------------------

local M = {}

---@type table<string, TaskConfig[]> a list of task configurations per filetype
M.list = {}

---launches a task specified by the given configuration
---@param config TaskConfig
function M.runner(config)
  local exe = ('%s %s'):format(config.command, table.concat(config.args or {}, ' '))
  vim.cmd.tabnew()
  vim.cmd.terminal(exe)

  vim.api.nvim_buf_set_name(0, ('TASK: %s [%s]'):format(config.name, os.date '%d-%b %H:%M:%S'))
  vim.api.nvim_buf_delete(vim.fn.bufnr '#' --[[@as integer]], { force = true })
end

return M
