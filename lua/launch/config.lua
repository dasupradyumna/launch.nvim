--------------------------------------- PLUGIN CONFIGURATION ---------------------------------------

local util = require 'launch.util'

local api = vim.api

local M = {}

---@alias DisplayType 'float' | 'tab'

---@class Hook
---@field pre? function executed before a certain event
---@field post? function executed after a certain event

---@class PluginConfigTask
---@field display DisplayType whether to render the task output in a tabpage or a floating window
---@field float_config table can contain the same key-values pairs as `vim.api.nvim_open_win()`
---@field hooks { float: Hook, tab: Hook } function hooks for user to customize specific behavior
---@field insert_on_launch boolean whether to auto-enter insert mode after launching task
---@field options TaskOptions additional task environment options
---@field runner? fun(c: TaskConfig) custom runner used to launch a selected task
---@field term table can contain the same key-value pairs as `opts` argument of `jobstart()`

---@class PluginConfigDebug
---@field adapters? table<string, string> mapping filetype to an adapter name (from `dap.adapters`)
---@field disable boolean whether to disable debugger support
---@field runner? function custom runner used to launch a selected debug config
---@field templates? table<string, DebugConfig> debug configuration template per filetype

---@class PluginConfig
---@field debug? PluginConfigDebug
---@field task? PluginConfigTask
M.defaults = {
  debug = {
    adapters = nil, -- CHECK: change this to adapter config instead of mapping?
    disable = false,
    runner = nil,
    templates = nil,
  },
  task = {
    display = 'float',
    float_config = {
      relative = 'editor',
      border = 'rounded',
      title_pos = 'center',
    },
    hooks = {
      float = {
        pre = nil,
        post = nil,
      },
      tab = {
        pre = nil,
        post = nil,
      },
    },
    insert_on_launch = false,
    options = {
      cwd = nil,
      env = nil,
      shell = nil,
    },
    -- rerun_replace_current = false, -- replace previous task or create unique using timestamp
    runner = nil,
    term = {
      clear_env = false,
    },
  },
}

---@type PluginConfig runtime user configuration
M.user = {} ---@diagnostic disable-line

---applies the argument options to the defaults and saves it as user config
---@param opts? PluginConfig
function M.apply(opts)
  -- TODO: validation of the argument options (below merging might fail otherwise)

  M.user = util.deep_merge(M.defaults, opts or {})
  M.user.task.float_config = util.merge(M.user.task.float_config, {
    -- NOTE: values below are placeholders to safely initialize a blank float
    title = '',
    row = 1,
    col = 1,
    width = 5,
    height = 5,
  })
end

---opens the config file in a plugin-managed floating window in a buffer managed by this table
M.open_file = setmetatable({}, {
  __call = function(self)
    if not self.buf then
      self.buf = vim.fn.bufadd(require('launch.core').config_file_path)
      -- CHECK: possible refactor; repeated code for plugin-created buffers
      vim.keymap.set('n', 'q', '<Cmd>quit<CR>', { buffer = self.buf })
      api.nvim_create_autocmd('BufWipeout', {
        desc = 'Uncache the buffer handle holding the content of the view',
        callback = function() self.buf = nil end,
        buffer = self.buf,
        group = 'launch_nvim',
      })
    end
    local win = require('launch.view').handles.win
    api.nvim_win_set_buf(win, self.buf)

    -- CHECK: very similar to `launch.view.open_win()`; possible refactor?
    local r, c, w, h = util.get_win_pos_centered(0.8, 0.9)
    local float_config = util.merge(require('launch.config').user.task.float_config, {
      width = w,
      height = h,
      row = r,
      col = c,
      title = ' [launch.nvim] User Configurations File ',
    })
    api.nvim_win_set_config(win, float_config)
    api.nvim_set_option_value('winbar', '', { win = win })
    api.nvim_set_option_value('signcolumn', 'yes:1', { win = win })
  end,
})

return M
