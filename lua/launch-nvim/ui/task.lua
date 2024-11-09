--------------------------------------- TASK RENDERING LOGIC ---------------------------------------

---@class LaunchNvimTaskUIModule
---@field private win_id integer window ID managed by this module
---@field private tab_id integer tabpage ID managed by this module
local task_ui = {}

---open the task in UI mode specified by its config
---@param active_task LaunchNvimActiveTask runtime data of current task
function task_ui:open(active_task)
  if active_task.config.display == 'float' then
    self:float(active_task.buffer, active_task.config.name)
  end
end

---open the task in a floating window
---@param buffer integer task buffer ID
---@param title string floating window title
function task_ui:float(buffer, title)
  local float_config = {
    row = 5,
    col = 45,
    width = 150,
    height = 50,
    title = (' TASK: %s '):format(title),
    title_pos = 'center',
    footer = ' launch.nvim ',
    footer_pos = 'right',
    style = 'minimal',
    relative = 'editor',
    border = 'rounded',
    zindex = 49,
  }

  self.win_id = vim.api.nvim_open_win(buffer, true, float_config)
  vim.api.nvim_win_set_buf(self.win_id, buffer)
  vim.api.nvim_set_option_value('signcolumn', 'yes:1', { win = self.win_id })
  vim.api.nvim_set_option_value('winbar', '', { win = self.win_id })
end

function task_ui:tabpage() end

function task_ui:vsplit() end

function task_ui:hsplit() end

return task_ui
