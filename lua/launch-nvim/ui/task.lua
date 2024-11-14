--------------------------------------- TASK RENDERING LOGIC ---------------------------------------

local settings = require 'launch-nvim.settings'

---@class LaunchNvimTaskUIModule
---@field private win_id table<LaunchNvimTaskDisplayType, integer> window ID per display type
local task_ui = { win_id = vim.empty_dict() }

---mapping from size names to fraction of screen dimensions
local float_size_to_ratio = { small = 0.45, medium = 0.65, large = 0.85 }

---get the floating window UI position-size specifications
---@param size LaunchNvimSettingsTaskFloatSize
local function get_float_specs(size)
  local W, H = vim.go.columns, vim.go.lines
  local ratio = float_size_to_ratio[size]
  local w, h = ratio * W, ratio * H
  local c = (W - w) / 2 - 2
  local r = (H - h) / 2 - 2
  return math.floor(r), math.floor(c), math.ceil(w), math.ceil(h)
end

---table of renderer methods per display type
local renderer = {}

---open the task in a floating window
---@param buffer integer task buffer ID
---@param title string floating window title
---@return integer # current window ID
---@nodiscard
function renderer.float(buffer, title)
  local float_settings = settings.active.task.float

  -- construct floating window config
  local row, col, width, height = get_float_specs(float_settings.size)
  local float_config = vim.tbl_extend('force', float_settings.config, {
    relative = 'editor', -- CHECK: can other options be supported?
    row = row,
    col = col,
    width = width,
    height = height,
    title = (' %s '):format(title),
    footer = ' launch.nvim ',
    style = 'minimal',
  })

  local win = vim.api.nvim_open_win(buffer, true, float_config)
  vim.api.nvim_set_option_value('signcolumn', 'yes:1', { scope = 'local' })
  vim.api.nvim_set_option_value('winbar', '', { scope = 'local' })

  -- auto-close when focus is lost
  vim.api.nvim_create_autocmd('WinLeave', {
    desc = 'Close the task UI floating window if it loses focus',
    callback = function()
      if vim.api.nvim_get_current_win() == win then vim.api.nvim_win_close(win, true) end
    end,
    group = 'launch_nvim',
    once = true,
  })

  return win
end

---open the task in a vertically split window
---@return integer # current window ID
---@nodiscard
function renderer.vsplit()
  vim.api.nvim_command 'vsplit'
  vim.api.nvim_set_option_value('signcolumn', 'yes:1', { scope = 'local' })
  return vim.api.nvim_get_current_win()
end

---open the task in a horizontally split window
---@return integer # current window ID
---@nodiscard
function renderer.hsplit()
  vim.api.nvim_command 'split'
  vim.api.nvim_set_option_value('signcolumn', 'yes:1', { scope = 'local' })
  return vim.api.nvim_get_current_win()
end

---open the task in UI mode specified by its config
---@param active_task LaunchNvimActiveTask runtime data of current task
function task_ui:open(active_task)
  local display = active_task.config.display
  local win = self.win_id[display] or renderer[display](active_task.buffer, active_task.title)
  self.win_id[display] = win

  vim.api.nvim_set_current_win(win)
  vim.api.nvim_set_option_value('winfixbuf', false, { scope = 'local' })
  vim.api.nvim_set_current_buf(active_task.buffer)
  vim.api.nvim_set_option_value('winfixbuf', true, { scope = 'local' })
end

return task_ui
