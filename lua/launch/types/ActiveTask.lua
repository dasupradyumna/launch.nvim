------------------------------------------- ACTIVE-TASK --------------------------------------------

local config = require 'launch.config'
local util = require 'launch.util'
local view = require 'launch.view'

local api = vim.api
local fn = vim.fn

---@class ActiveTask
---@field title string unique title for the current instance
---@field buffer integer buffer ID of the terminal running the task
---@field command string command string of the current task
---@field display DisplayType whether to render the task output in a tabpage or a floating window
---@field options TaskOptions additional options configuring how the task is run
local ActiveTask = {}

---creates a new instance of `ActiveTask`
---@param cfg TaskConfig task configuration which needs to run
---@return ActiveTask
---@nodiscard
function ActiveTask:new(cfg)
  local buffer = api.nvim_create_buf(false, true)
  api.nvim_set_option_value('filetype', 'launch_nvim_terminal', { buf = buffer })

  return setmetatable({
    title = ('TASK: %s [%s]'):format(cfg.name, os.date '%d-%b %H:%M:%S'),
    buffer = buffer,
    command = ('%s %s'):format(cfg.command, table.concat(cfg.args or {}, ' ')),
    display = cfg.display,
    options = cfg.options,
  }, { __index = self })
end

---@type { float: function, tab: table } holds the rendering functions for floats and tabs
ActiveTask.renderer = {

  ---render active task in a floating window
  ---@param buffer integer buffer of active task
  ---@param title string title of active task
  float = function(buffer, title)
    -- CHECK: very similar to `launch.view.open_win()`; possible refactor?
    local r, c, w, h = util.get_win_pos_centered(0.8, 0.9)
    local float_config = util.merge(config.user.task.float_config, {
      width = w,
      height = h,
      row = r,
      col = c,
      title = (' %s '):format(title),
    })

    local win = view.handles.win
    api.nvim_win_set_buf(win, buffer)
    api.nvim_win_set_config(win, float_config) -- for some reason, resets all window options
    api.nvim_set_option_value('signcolumn', 'yes:1', { win = win })
    api.nvim_set_option_value('winbar', '', { win = win })
  end,

  -- CHECK: can this be converted into a function?
  tab = setmetatable({}, {
    ---render active task in a tabpage
    ---@param buffer integer buffer of active task
    __call = function(self, buffer)
      if not self.handle then
        api.nvim_set_var('disable_new_tab_name_prompt', true) -- HACK: remove this
        api.nvim_command(('tab sbuffer %s'):format(buffer))
        self.handle = api.nvim_get_current_tabpage()
        api.nvim_tabpage_set_var(0, 'tabname', 'TaskRunner') -- HACK: remove this
      else
        api.nvim_set_current_tabpage(self.handle)
      end

      api.nvim_win_set_buf(0, buffer) -- FIX: check if window is a float before setting it
    end,
  }),
}

---renders the running task in either a floating window or a tabpage
---@param display? DisplayType
function ActiveTask:render(display)
  display = display or self.display
  self.renderer[display](self.buffer, self.title)
end

---runs task in a new terminal with the appropriate title
function ActiveTask:run()
  local tmp = {}
  if self.options.shell then
    tmp.shell = api.nvim_get_option_value('shell', {})
    tmp.shellcmdflag = api.nvim_get_option_value('shellcmdflag', {})
    if self.options.shell.exec then
      api.nvim_set_option_value('shell', self.options.shell.exec, {})
    end
    if self.options.shell.args then
      api.nvim_set_option_value('shellcmdflag', table.concat(self.options.shell.args, ' '), {})
    end
  end
  fn.termopen(
    self.command,
    util.merge(config.user.task.term, {
      cwd = self.options.cwd,
      env = self.options.env,
    })
  )
  if self.options.shell then
    api.nvim_set_option_value('shell', tmp.shell, {})
    api.nvim_set_option_value('shellcmdflag', tmp.shellcmdflag, {})
  end

  api.nvim_buf_set_name(self.buffer, self.title)
  pcall(api.nvim_buf_delete, fn.bufnr '#', { force = true }) -- alternate buffer may not exist

  if config.user.insert_on_task_launch then api.nvim_command 'startinsert' end
end

return ActiveTask
