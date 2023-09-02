------------------------------------------- ACTIVE-TASK --------------------------------------------

local api = vim.api
local fn = vim.fn

---@class ActiveTask
---@field title string unique title for the current instance
---@field buffer integer buffer ID of the terminal running the task
---@field channel integer task terminal channel ID
---@field config TaskConfig current task configuration after user variable substitution
local ActiveTask = {}

---creates a new instance of `ActiveTask`
---@param config TaskConfig task configuration which needs to run
---@return ActiveTask
---@nodiscard
function ActiveTask.new(config)
  local buffer = api.nvim_get_current_buf()
  buffer = api.nvim_buf_get_name(buffer) == '' and buffer or api.nvim_create_buf(false, true)
  api.nvim_set_option_value('filetype', 'launch_nvim_terminal', { buf = buffer })

  return setmetatable({
    title = ('TASK: %s [%s]'):format(config.name, os.date '%d-%b %H:%M:%S'),
    buffer = buffer,
    config = config,
  }, { __index = ActiveTask })
end

---runs task in a new terminal with the appropriate title
function ActiveTask:run()
  api.nvim_win_set_buf(0, self.buffer)

  self.channel = fn.termopen(
    ('%s %s'):format(self.config.command, table.concat(self.config.args or {}, ' ')),
    { clear_env = false }
  )

  api.nvim_buf_set_name(self.buffer, self.title)
  api.nvim_buf_delete(fn.bufnr '#' --[[@as integer]], { force = true })

  api.nvim_command 'startinsert'
end

return ActiveTask
