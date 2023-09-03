------------------------------------------- ACTIVE-TASK --------------------------------------------

local api = vim.api
local fn = vim.fn

---@class ActiveTask
---@field title string unique title for the current instance
---@field buffer integer buffer ID of the terminal running the task
---@field command string command string of the current task
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
  }, { __index = ActiveTask })
end

ActiveTask.renderer = {
  float = function(buffer, title)
    api.nvim_open_win(buffer, true, {
      relative = 'editor',
      width = 200,
      height = 40,
      row = 2,
      col = 20,
      border = 'rounded',
      title = (' %s '):format(title),
      title_pos = 'center',
      -- TODO: add appropriate footer?
    })
  end,
  tab = setmetatable({}, {
    __call = function(self, buffer)
      if not self.handle then
        api.nvim_set_var('disable_new_tab_name_prompt', true) -- HACK: remove this
        api.nvim_command(('tab sbuffer '):format(buffer))
        self.handle = api.nvim_get_current_tabpage()
        api.nvim_tabpage_set_var(0, 'tabname', 'TaskRunner') -- HACK: remove this
      end

      api.nvim_set_current_tabpage(self.handle)
      api.nvim_win_set_buf(0, buffer) -- FIX: check if window is a float before setting it
    end,
  }),
}

---renders the running task in either a floating window or a tabpage
---@param display 'float' | 'tab'
function ActiveTask:render(display) self.renderer[display](self.buffer, self.title) end

---runs task in a new terminal with the appropriate title
function ActiveTask:run()
  fn.termopen(self.command, { clear_env = false })

  api.nvim_buf_set_name(self.buffer, self.title)
  pcall(api.nvim_buf_delete, fn.bufnr '#', { force = true }) -- alternate buffer may not exist
end

return ActiveTask
