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
  local screen_w, screen_h = vim.o.columns, vim.o.lines
  local ratio = float_size_to_ratio[size]
  local win_w, win_h = ratio * screen_w, ratio * screen_h
  local win_c = (screen_w - win_w) / 2 - 2
  local win_r = (screen_h - win_h) / 2 - 2
  return math.floor(win_r), math.floor(win_c), math.ceil(win_w), math.ceil(win_h)
end

---table of renderer methods per display type
local renderer = {}

---open the task in a floating window
---@param buffer integer task buffer ID
---@param title string floating window title
---@return integer # current window ID
---@nodiscard
function renderer.float(buffer, title)
  local float_settings = settings.active.task.ui.float

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
  vim.wo.signcolumn = 'yes:1'
  vim.wo.winbar = ''

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
function renderer.vsplit(buffer)
  local width = math.floor(vim.o.columns * settings.active.task.ui.vsplit_width * 0.01)
  local win = vim.api.nvim_open_win(buffer, true, { split = 'right', width = width })
  vim.wo.signcolumn = 'yes:1'

  return win
end

---open the task in a horizontally split window
---@return integer # current window ID
---@nodiscard
function renderer.hsplit(buffer)
  local height = math.floor(vim.o.lines * settings.active.task.ui.hsplit_height * 0.01)
  local win = vim.api.nvim_open_win(buffer, true, { split = 'below', height = height })
  vim.wo.signcolumn = 'yes:1'

  return win
end

---open the task in UI mode specified by its config
---@param active_task LaunchNvimActiveTask runtime data of current task
function task_ui:open(active_task)
  local display = active_task.config.display
  local win = self.win_id[display] or renderer[display](active_task.buffer, active_task.title)
  self.win_id[display] = win

  vim.api.nvim_set_current_win(win)
  vim.wo.winfixbuf = false
  vim.api.nvim_set_current_buf(active_task.buffer)
  vim.wo.winfixbuf = true
end

return task_ui
