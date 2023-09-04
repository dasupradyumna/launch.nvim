------------------------------------------- ACTIVE-TASK --------------------------------------------

local util = require 'launch.util'

local api = vim.api
local fn = vim.fn

---@class ActiveTask
---@field title string unique title for the current instance
---@field buffer integer buffer ID of the terminal running the task
---@field command string command string of the current task
---@field display DisplayType whether to render the task output in a tabpage or a floating window
local ActiveTask = {}

---creates a new instance of `ActiveTask`
---@param config TaskConfig task configuration which needs to run
---@return ActiveTask
---@nodiscard
function ActiveTask.new(config)
  local buffer = api.nvim_create_buf(false, true)
  api.nvim_set_option_value('filetype', 'launch_nvim_terminal', { buf = buffer })

  return setmetatable({
    title = ('TASK: %s [%s]'):format(config.name, os.date '%d-%b %H:%M:%S'),
    buffer = buffer,
    command = ('%s %s'):format(config.command, table.concat(config.args or {}, ' ')),
    display = config.display,
  }, { __index = ActiveTask })
end

---@type { float: table, tab: table } holds the rendering functions for floats and tabs
ActiveTask.renderer = {
  float = setmetatable({}, {
    ---render active task in a floating window
    ---@param buffer integer buffer of active task
    ---@param title string title of active task
    __call = function(self, buffer, title)
      if not self.handle then
        local r, c, w, h = util.get_win_pos_centered(0.8, 0.9)
        self.handle = api.nvim_open_win(buffer, true, {
          relative = 'editor',
          width = w,
          height = h,
          row = r,
          col = c,
          border = 'rounded',
          title = (' %s '):format(title),
          title_pos = 'center',
          style = 'minimal',
        })
        api.nvim_set_option_value('signcolumn', 'yes:1', { win = self.handle })
      else
        api.nvim_win_set_buf(self.handle, buffer)
        api.nvim_win_set_config(self.handle, {
          title = (' %s '):format(title),
          title_pos = 'center',
        })
      end

      api.nvim_set_option_value('winbar', '', { win = self.handle })
    end,
  }),
  tab = setmetatable({}, {
    ---render active task in a tabpage
    ---@param buffer integer buffer of active task
    __call = function(self, buffer)
      if not self.handle then
        api.nvim_set_var('disable_new_tab_name_prompt', true) -- HACK: remove this
        api.nvim_command(('tab sbuffer '):format(buffer))
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
---@param display DisplayType?
function ActiveTask:render(display)
  display = display or self.display
  self.renderer[display](self.buffer, self.title)
end

---runs task in a new terminal with the appropriate title
function ActiveTask:run()
  fn.termopen(self.command, { clear_env = false })

  api.nvim_buf_set_name(self.buffer, self.title)
  pcall(api.nvim_buf_delete, fn.bufnr '#', { force = true }) -- alternate buffer may not exist
end

return ActiveTask
