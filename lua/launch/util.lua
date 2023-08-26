---------------------------------------- UTILITY FUNCTIONS -----------------------------------------

local M = {}

---@type table<string, integer> mapping from simple string to `vim.log.levels`
local log_level = {
  info = vim.log.levels.INFO,
  warn = vim.log.levels.WARN,
  err = vim.log.levels.ERROR,
}

---display message with the appropriate highlight for its notification level
---@param message string display message
---@param level 'err' | 'warn' | 'info' notification level
function M.notify(level, message, ...)
  local msg = message:format(...)
  vim.notify('[launch.nvim] ' .. msg, log_level[level])
end

---display the argument error message and propagate an error up the stack
---POSSIBLY THROWS ERROR
function M.throw_notify(...)
  M.notify(...)
  error()
end

return M
