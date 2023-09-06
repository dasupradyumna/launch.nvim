--------------------------------------- PLUGIN CONFIGURATION ---------------------------------------

local util = require 'launch.util'

local M = {}

---@alias DisplayType 'float' | 'tab'

---@class PluginConfigTask
---@field display DisplayType whether to render the task output in a tabpage or a floating window
---@field float_config table can contain the same key-values pairs as `vim.api.nvim_open_win()`
---@field options TaskOptions additional task environment options
---@field runner fun(c: TaskConfig)? custom runner used to launch a selected task
---@field term table can contain the same key-value pairs as `opts` argument of `jobstart()`

---@class PluginConfigDebug
---@field runner function? custom runner used to launch a selected debug config

---@class PluginConfig
---@field task PluginConfigTask?
---@field debug PluginConfigDebug?
---@field insert_on_task_launch boolean? whether to auto-enter insert mode after launching task
M.defaults = {
  task = {
    display = 'float',
    -- rerun_replace_current = false, -- replace previous task or create unique using timestamp
    float_config = {
      relative = 'editor',
      border = 'rounded',
      title_pos = 'center',
      style = 'minimal',
    },
    options = {
      cwd = nil,
      env = nil,
      shell = nil,
    },
    runner = nil,
    term = { clear_env = false },
  },
  debug = {
    runner = nil,
    -- optional DAP default template for each filetype (for smaller config files)
    -- ft_templates = {}
  },
  insert_on_task_launch = false,
  -- config_type 'directory' | 'stdpath'
}

---@type PluginConfig runtime user configuration
M.user = {} ---@diagnostic disable-line

---applies the argument options to the defaults and saves it as user config
---@param opts PluginConfig?
function M.apply(opts) M.user = util.deep_merge(M.defaults, opts or {}) end

return M
