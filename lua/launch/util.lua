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

---display the argument message of specified level and propagate an error up the stack
---POSSIBLY THROWS ERROR
function M.throw_notify(...)
  M.notify(...)
  error()
end

---calculates the row and column coordinate of the NW corner of a floating window
---@param w number desired width; can be a integral pixel value (>=1) or a floating fraction (0-1)
---@param h number desired height; can be a integral pixel value (>=1) or a floating fraction (0-1)
---@return integer # row of NW corner
---@return integer # column of NW corner
---@return integer # pixel width of window
---@return integer # pixel height of window
---@nodiscard
---POSSIBLY THROWS ERROR
function M.get_win_pos_centered(w, h)
  local W = vim.api.nvim_get_option_value('columns', {})
  local H = vim.api.nvim_get_option_value('lines', {})
  local r, c
  if w < 1 and h < 1 then
    w, h = w * W, h * H
    w, h = math.floor(w), math.floor(h)
  elseif not (w >= 1 and h >= 1) then
    error 'arguments `w` and `h` should both be greater than (equal to) 1 or both lesser than 1'
  end

  r = (H - h) / 2 - 1
  c = (W - w) / 2 - 1
  return math.floor(r), math.floor(c), w, h
end

return M
