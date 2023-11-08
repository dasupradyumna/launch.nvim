---------------------------------------- UTILITY FUNCTIONS -----------------------------------------

local api = vim.api

local M = {}

---@type table<string, integer> mapping from simple string to `vim.log.levels`
local log_level = {
  I = vim.log.levels.INFO,
  W = vim.log.levels.WARN,
  E = vim.log.levels.ERROR,
}

---@type boolean whether to display notifications or not
M.no_notify = false

---display message with the appropriate highlight for its notification level
---@param message string display message
---@param level 'E' | 'I' | 'W' notification level
function M.notify(level, message, ...)
  if M.no_notify then return end

  local msg = message:format(...)
  if type(vim.notify) == 'function' then -- builtin
    vim.notify('[launch.nvim] ' .. msg, log_level[level])
  elseif type(vim.notify) == 'table' then -- nvim-notify
    vim.notify(msg, log_level[level], { title = 'launch.nvim' })
  end
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
  local W = api.nvim_get_option_value('columns', {})
  local H = api.nvim_get_option_value('lines', {})
  local r, c
  if w < 1 and h < 1 then
    w, h = w * W, h * H
  elseif w >= 1 and h >= 1 then
    w, h = math.min(w, 0.9 * W), math.min(h, 0.9 * H)
  else
    error 'arguments `w` and `h` should both be greater than (equal to) 1 or both lesser than 1'
  end

  r = (H - h) / 2 - 1
  c = (W - w) / 2 - 1
  return math.floor(r), math.floor(c), math.floor(w), math.floor(h)
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

M.try_require = setmetatable({
  ---@type table<string, string> error message if the plugin is missing
  warn_msg = {
    dap = 'The plugin `mfussenegger/nvim-dap` was not found.\n    Please ensure it is installed and'
      .. ' loaded before `launch.nvim` to include support for launching debugger processes',
  },
}, {
  ---load the specified plugin if it exists else (*optionally*) notify the user
  ---@param plugin string module name of the plugin
  ---@param emit_warn? boolean whether to emit a warning if module could not be loaded
  ---@return table? # the main plugin module if it exists
  __call = function(self, plugin, emit_warn)
    if not self[plugin] then
      local ok, loaded = pcall(require, plugin)
      self[plugin] = ok and loaded or {}
    end

    if vim.tbl_isempty(self[plugin]) then
      if emit_warn then M.notify('W', self.warn_msg[plugin]) end
      return
    else
      return self[plugin]
    end
  end,
})

---filter the given dictionary of configs by current buffer filetype or list all configs
---@param configs table<string, LaunchConfig[]> list of configurations for user to filter
---@param all_filetypes? boolean whether to return all configs or only filtered by current filetype
---@return LaunchConfig[] filtered filtered configuration list
---@return string? filetype filetype of the current buffer (**nil** if `all_filetypes` is **true**)
function M.filter_configs_by_filetype(configs, all_filetypes)
  local filtered, filetype
  if all_filetypes then
    filtered = {}
    for _, ft_configs in pairs(configs) do
      vim.list_extend(filtered, ft_configs)
    end
  else
    filetype = vim.api.nvim_get_option_value('filetype', { buf = 0 })
    filtered = configs[filetype] or {}
  end

  return filtered, filetype
end

---generate a string representation of the argument key-value pair, with indentation based on level
---@param key string
---@param value table
---@param level integer
---@param key_sorter function?
---@return string[]
function M.key_value_repr(key, value, level, key_sorter)
  if vim.tbl_isempty(value) then return {} end

  local ws = (' '):rep(level * 2)
  local repr = { ('%s%s:'):format(ws, key) }
  ws = ws .. '  '

  local fields = vim.tbl_keys(value)
  table.sort(fields, key_sorter)
  for _, field in ipairs(fields) do
    local f_value = value[field]
    if type(f_value) == 'table' then
      vim.list_extend(repr, M.key_value_repr(field, f_value, level + 1))
    else
      table.insert(repr, ('%s%s: %s'):format(ws, field, f_value))
    end
  end

  return repr
end

return M
