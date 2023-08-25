---------------------------------------- UTILITY FUNCTIONS -----------------------------------------

local M = {}

---@type table<string, integer> mapping from simple string to `vim.log.levels`
local log_level = {
  info = vim.log.levels.INFO,
  warn = vim.log.levels.WARN,
  error = vim.log.levels.ERROR,
}

---display message with the appropriate highlight for its notification level
---@param message string display message
---@param level 'error' | 'warn' | nil notification level
function M.notify(message, level)
  vim.notify('[launch.nvim] ' .. message, log_level[level or 'info'])
end

return M
