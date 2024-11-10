--------------------------------------- TASK RENDERING LOGIC ---------------------------------------

---@class LaunchNvimTaskUIModule
---@field private win_id table<LaunchNvimTaskDisplayType, integer> window ID per display type
local task_ui = { win_id = vim.empty_dict() }

---table of renderer methods per display type
local renderer = {}

---open the task in a floating window
---@param buffer integer task buffer ID
---@param title string floating window title
---@return integer # current window ID
---@nodiscard
function renderer.float(buffer, title)
  local float_config = {
    row = 5,
    col = 45,
    width = 150,
    height = 50,
    title = ' ' .. title .. ' ',
    title_pos = 'center',
    footer = ' launch.nvim ',
    footer_pos = 'right',
    style = 'minimal',
    relative = 'editor',
    border = 'rounded',
    zindex = 49,
  }

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
