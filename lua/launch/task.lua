------------------------------------------- TASK RUNNER --------------------------------------------

local config = require 'launch.config'
local util = require 'launch.util'

local ActiveTask = require 'launch.types.ActiveTask'

local api = vim.api

local M = {}

---@type table<string, TaskConfig[]> a list of task configurations per filetype
M.list = {}

---@type table<integer, ActiveTask> a mapping of currently active tasks by their buffers
M.active = {}

---launches a task specified by the given configuration
---@param cfg TaskConfig
function M.runner(cfg)
  local task = ActiveTask:new(cfg)
  task:render(cfg.display)
  task:run()

  M.active[task.buffer] = task
  api.nvim_create_autocmd('BufWipeout', {
    desc = 'Remove current task from active task list when its buffer is deleted',
    callback = function() M.active[task.buffer] = nil end,
    buffer = task.buffer,
    group = 'launch_nvim',
  })
  vim.keymap.set('n', 'q', '<Cmd>q<CR>', { buffer = task.buffer })
end

---display all active tasks and allow opening them in floating windows or tabpages
M.show_active = setmetatable({
  ---create a new buffer for showing active tasks
  get_buf = function(self)
    if self.buf then return end

    self.buf = api.nvim_create_buf(false, true)
    vim.keymap.set('n', 'q', '<Cmd>q<CR>', { buffer = self.buf })
    api.nvim_set_option_value('modifiable', false, { buf = self.buf })
  end,
}, {
  __call = function(self)
    self:get_buf()
    local max_width = 0
    local height = vim.tbl_count(M.active)

    api.nvim_set_option_value('modifiable', true, { buf = self.buf })
    if height == 0 then
      local msg = '  No active tasks found  '
      api.nvim_buf_set_lines(self.buf, 0, -1, false, { '', msg, '' })
      height = 1
      max_width = msg:len()
    else
      local line_fmt = height < 10 and '  %d. %s  ' or '  %2d. %s  '
      local bufs = vim.tbl_keys(M.active)
      table.sort(bufs)

      local lines = { '' }
      for i, buf in ipairs(bufs) do
        local active = M.active[buf]
        local line = line_fmt:format(i, active.title:sub(7))
        table.insert(lines, line)
        max_width = math.max(max_width, line:len())
      end
      table.insert(lines, '')
      api.nvim_buf_set_lines(self.buf, 0, -1, false, lines)

      ---open the listed task in either a floating window or a tabpage
      ---@param display DisplayType?
      local function open_task(display)
        local line, _ = unpack(api.nvim_win_get_cursor(0))
        if line == 1 or line == height + 2 then return end

        api.nvim_win_close(0, true)
        M.active[bufs[line - 1]]:render(display)
      end

      vim.keymap.set('n', '<C-F>', function() open_task 'float' end, { buffer = self.buf })
      vim.keymap.set('n', '<C-T>', function() open_task 'tab' end, { buffer = self.buf })
      vim.keymap.set('n', '<CR>', open_task, { buffer = self.buf })
    end
    api.nvim_set_option_value('modifiable', false, { buf = self.buf })

    local r, c, w, h = util.get_win_pos_centered(max_width, height + 2)
    local float_config = util.merge(config.user.task.float_config, {
      width = w,
      height = h,
      row = r,
      col = c,
      title = ' Active Tasks ',
      zindex = 60,
    })
    local win = api.nvim_open_win(self.buf, true, float_config)
    api.nvim_set_option_value('cursorline', true, { win = win })
  end,
})

return M
