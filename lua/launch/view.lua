-------------------------------------- FLOATING VIEW HANDLER ---------------------------------------

local config = require 'launch.config'
local util = require 'launch.util'

local api = vim.api

local M = {}

---@alias ViewType 'active' | 'user' | LaunchType

---@type table<ViewType, string>
local title = {
  active = 'Active Tasks',
  debug = 'Debug Configurations',
  task = 'Configured Tasks',
  user = 'User Variables',
}

M.handles = setmetatable({}, {
  __index = function(self, key)
    local val

    -- create content buffer if none exists
    if key == 'buf' then
      val = api.nvim_create_buf(false, true)
      api.nvim_set_option_value('modifiable', false, { buf = val })
      vim.keymap.set('n', 'q', '<Cmd>quit<CR>', { buffer = val })
      -- CHECK: possible refactor; repeated code for plugin-created buffers
      api.nvim_create_autocmd('BufWipeout', {
        desc = 'Uncache the buffer handle holding the content of the view',
        callback = function() self.buf = nil end,
        buffer = val,
        group = 'launch_nvim',
      })
    -- create floating window if none exists
    elseif key == 'win' then
      pcall(config.user.task.hooks.float.pre)
      val = api.nvim_open_win(self.buf, true, config.user.task.float_config)
      pcall(config.user.task.hooks.float.post)
      api.nvim_win_set_buf(val, self.buf)
    end

    rawset(self, key, val)
    return val
  end,
})

---open or use an existing floating window with given specs
---@param type ViewType the type of content that will be displayed
---@param width number window width
---@param height number window height
---@param ft? string target filetype for window title
function M.open_win(type, width, height, ft)
  local _title = (' [launch.nvim] %s %s'):format(title[type], (ft and (': %s '):format(ft) or ''))
  local r, c, w, h = util.get_win_pos_centered(width, height)
  w = (w >= #_title + 8) and w or (#_title + 8) -- if title is wider than buffer content
  local float_config = util.merge(config.user.task.float_config, {
    width = w,
    height = h,
    row = r,
    col = c,
    title = _title,
    style = 'minimal',
  })

  local win = M.handles.win
  api.nvim_win_set_buf(win, M.handles.buf)
  api.nvim_win_set_config(win, float_config) -- for some reason, resets all window options
  api.nvim_set_option_value('cursorline', true, { win = win })
end

---get a string representation for the argument config
---@param cfg LaunchConfig
function M.get_repr(cfg)
  local name = cfg.name
  cfg.name = nil
  local display = util.key_value_repr(name, cfg, 0)
  cfg.name = name

  return display
end

---returns a list of lines given a particular type of content
---@param type ViewType the type of content that will be displayed
---@param show_all_fts? boolean whether to display all configs or only based on current filetype
---@return string[]? lines
---@return string? ft buffer filetype (*optional*)
function M.get_content(type, show_all_fts)
  local task = require 'launch.task'
  local lines, ft = {}, nil

  if type == 'active' then
    -- display content for currently active tasks

    local bufs = vim.tbl_keys(task.active)
    local n_bufs = #bufs
    local line_fmt = n_bufs < 10 and '  %d. %s  ' or '  %2d. %s  '

    table.sort(bufs)
    for i, buf in ipairs(bufs) do
      local line = line_fmt:format(i, task.active[buf].title)
      table.insert(lines, line)
    end

    -- TODO: refactor the below keymap functionality out of this function (how?)

    ---open the listed task in either a floating window or a tabpage
    ---@param display? DisplayType
    local function open_task(display)
      local line, _ = unpack(api.nvim_win_get_cursor(0))
      if line == 1 or line == n_bufs + 2 then return end

      api.nvim_win_close(0, true)
      task.active[bufs[line - 1]]:render(display)
    end

    vim.keymap.set('n', '<C-F>', function() open_task 'float' end, { buffer = M.handles.buf })
    vim.keymap.set('n', '<C-T>', function() open_task 'tab' end, { buffer = M.handles.buf })
    vim.keymap.set('n', '<CR>', open_task, { buffer = M.handles.buf })
  elseif type == 'user' then
    -- display content for user variables

    for name, cfg in pairs(require('launch.user').variables) do
      local cfg_repr = util.key_value_repr(name, cfg, 0)
      table.insert(cfg_repr, '')
      vim.list_extend(lines, cfg_repr)
    end
    lines[#lines] = nil -- remove extra new line at the end
  else
    -- display content for task and debug configurations

    local list = type == 'task' and task.list or require('dap').configurations
    list, ft = util.filter_configs_by_filetype(list, show_all_fts)
    for _, cfg in ipairs(list) do
      local cfg_repr = vim.tbl_map(function(str)
        ---@cast str string
        -- escaping whitespace characters for `nvim_buf_set_lines()`
        str = str:gsub('\n', '\\n')
        str = str:gsub('\t', '\\t')
        str = str:gsub('\r', '\\r')
        return str
      end, M.get_repr(cfg))
      table.insert(cfg_repr, '')
      vim.list_extend(lines, cfg_repr)
    end
    lines[#lines] = nil -- remove extra new line at the end
  end

  if vim.tbl_isempty(lines) then lines = nil end
  return lines, ft
end

---render a list of configurations either for a specific filetype or all filetypes
---@param type ViewType the type of content that will be displayed
---@param show_all_fts? boolean whether to display all configs or only based on current filetype
function M.render(type, show_all_fts)
  local lines, ft = M.get_content(type, show_all_fts)
  local width, height = 0, 0

  if lines then
    lines = vim.tbl_map(function(line)
      line = ('    %s'):format(line)
      width = math.max(width, line:len())
      height = height + 1
      return line
    end, lines)
  else
    lines = { ('       No %s found'):format(title[type]:lower()) }
    width = lines[1]:len()
    height = 1
  end
  table.insert(lines, 1, '')
  table.insert(lines, '')

  api.nvim_set_option_value('modifiable', true, { buf = M.handles.buf })
  api.nvim_buf_set_lines(M.handles.buf, 0, -1, false, lines)
  api.nvim_set_option_value('modifiable', false, { buf = M.handles.buf })

  M.open_win(type, width + 4, height + 2, ft)
end

return M
