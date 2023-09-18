--------------------------------------- PLUGIN CONFIGURATION ---------------------------------------

local util = require 'launch.util'

local M = {}

---@alias DisplayType 'float' | 'tab'

---@class Hook
---@field pre? function executed before a certain event
---@field post? function executed after a certain event

---@class PluginConfigTask
---@field display DisplayType whether to render the task output in a tabpage or a floating window
---@field float_config table can contain the same key-values pairs as `vim.api.nvim_open_win()`
---@field hooks { float: Hook, tab: Hook } user hooks for user to customize specific behavior
---@field options TaskOptions additional task environment options
---@field runner? fun(c: TaskConfig) custom runner used to launch a selected task
---@field term table can contain the same key-value pairs as `opts` argument of `jobstart()`

---@class PluginConfigDebug
---@field adapters? table<string, string> mapping filetype to an adapter name (from `dap.adapters`)
---@field disable boolean whether to disable debugger support
---@field runner? function custom runner used to launch a selected debug config
---@field templates? table<string, DebugConfig> debug configuration templates per filetype

---@class PluginConfig
---@field debug? PluginConfigDebug
---@field insert_on_task_launch boolean whether to auto-enter insert mode after launching task
---@field task? PluginConfigTask
M.defaults = {
  -- config_type 'directory' | 'stdpath'
  debug = {
    adapters = nil, -- CHECK: change this to adapter config instead of mapping?
    disable = false,
    runner = nil,
    templates = nil,
  },
  insert_on_task_launch = false,
  task = {
    display = 'float',
    float_config = {
      relative = 'editor',
      border = 'rounded',
      title_pos = 'center',
      style = 'minimal',
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

return M
