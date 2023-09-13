-------------------------------------- FLOATING VIEW HANDLER ---------------------------------------

local config = require 'launch.config'
local util = require 'launch.util'

local api = vim.api

local M = setmetatable({}, {
  __index = function(self, key)
    local val
    if key == 'buf' then -- create new buffer for holding view's content
      val = api.nvim_create_buf(false, true)
      api.nvim_set_option_value('modifiable', false, { buf = val })
      vim.keymap.set('n', 'q', '<Cmd>q<CR>', { buffer = val })
      api.nvim_create_autocmd('BufWipeout', {
        desc = 'Uncache the buffer handle holding the content of the view',
        callback = function() self.buf = nil end,
        buffer = val,
        group = 'launch_nvim',
      })
    end

    rawset(self, key, val)
    return val
  end,
})

local title = { debug = 'Debug Configurations', task = 'Configured Tasks' }

M.open_win = setmetatable({}, {
  ---open or use an existing floating window with given specs
  ---@param type LaunchType whether the target is a debug or a task configuration
  ---@param width number window width
  ---@param height number window height
  ---@param ft string target filetype for window title
  __call = function(self, type, width, height, ft)
    local r, c, w, h = util.get_win_pos_centered(width, height)
    local float_config = util.merge(config.user.task.float_config, {
      width = w,
      height = h,
      row = r,
      col = c,
      title = (' %s %s'):format(title[type], (ft and (': %s '):format(ft) or '')),
      zindex = 60,
    })

    if not self.handle then
      self.handle = api.nvim_open_win(M.buf, true, float_config)

      api.nvim_set_option_value('cursorline', true, { win = self.handle })
    else
      api.nvim_win_set_buf(self.handle, M.buf)
      api.nvim_win_set_config(self.handle, float_config)
    end
  end,
})

---get a string representation for the argument config
---@param cfg LaunchConfig
---@return string[]
function M.get_repr(cfg)
  local name = cfg.name
  cfg.name = nil
  local display = util.key_value_repr(name, cfg, 0)
  cfg.name = name

  return display
end

---render a list of configurations either for a specific filetype or all filetypes
---@param list table<string, LaunchConfig[]> list of configurations for user to select from
---@param type LaunchType whether the target is a debug or a task configuration
---@param show_all_fts boolean whether to display all configs or only based on current filetype
function M.render(list, type, show_all_fts)
  local configs, ft = util.filter_configs_by_filetype(list, show_all_fts)
  local width, height = 0, 1

  api.nvim_set_option_value('modifiable', true, { buf = M.buf })
  if vim.tbl_isempty(configs) then
    local msg = ('    No %s found    '):format(title[type]:lower())
    api.nvim_buf_set_lines(M.buf, 0, -1, false, { '', msg, '' })
    height = 3
    width = msg:len()
  else
    local lines = { '' }
    for _, cfg in ipairs(configs) do
      local cfg_display = M.get_repr(cfg)
      cfg_display = vim.tbl_map(function(str)
        local out = ('    %s'):format(str)
        -- escaping whitespace characters for `nvim_buf_set_lines`
        out = out:gsub('\n', '\\n')
        out = out:gsub('\r', '\\r')
        out = out:gsub('\t', '\\t')
        width = math.max(width, out:len() + 4)
        height = height + 1
        return out
      end, cfg_display)
      vim.list_extend(lines, cfg_display)
      table.insert(lines, '')
      height = height + 1
    end
    api.nvim_buf_set_lines(M.buf, 0, -1, false, lines)
  end
  api.nvim_set_option_value('modifiable', false, { buf = M.buf })

  M.open_win(type, width, height, ft)
end

return M
