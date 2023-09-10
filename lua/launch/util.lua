---------------------------------------- UTILITY FUNCTIONS -----------------------------------------

local M = {}

---@type table<string, integer> mapping from simple string to `vim.log.levels`
local log_level = {
  I = vim.log.levels.INFO,
  W = vim.log.levels.WARN,
  E = vim.log.levels.ERROR,
}

---display message with the appropriate highlight for its notification level
---@param message string display message
---@param level 'E' | 'I' | 'W' notification level
function M.notify(level, message, ...)
  vim.api.nvim_command 'redraw'
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

---shallow-merges the template table with a custom table, prioritizing custom keys
---@param template table<string, any>
---@param custom table<string, any>
---@return table<string, any>
function M.merge(template, custom) return vim.tbl_extend('force', template, custom) end

---deep-merges the template table with a custom table, prioritizing custom keys
---@param template table<string, any>
---@param custom table<string, any>
---@return table<string, any>
function M.deep_merge(template, custom) return vim.tbl_deep_extend('force', template, custom) end

---checks if the argument table can be treated as a pure dictionary
---an empty table {} is considered a valid dictionary
---@param t any
function M.tbl_isdict(t)
  if type(t) ~= 'table' then return false end

  for k, _ in pairs(t) do
    if type(k) ~= 'string' then return false end
  end

  return true
end

---@type table<string, string> error message if the plugin is missing
local err_msg = {
  dap = 'The plugin `mfussenegger/nvim-dap` is not installed\n'
    .. '    Please install it to include support for launching debugger processes',
}

---load the specified plugin if it exists else notify the user
---@param plugin string module name of the plugin
---@return table? loaded the main plugin module if it exists
function M.load_if_exists(plugin)
  local ok, loaded = pcall(require, plugin)
  if not ok then
    M.notify('E', err_msg[plugin])
    return
  end

  return loaded
end

return M
