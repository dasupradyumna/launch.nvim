--------------------------------------- PLUGIN CONFIGURATION ---------------------------------------

local util = require 'launch.util'

local M = {}

---@alias DisplayType 'float' | 'tab'

---@class ConfigTask
---@field runner fun(c: TaskConfig)? custom runner used to launch a selected task
---@field display DisplayType whether to render the task output in a tabpage or a floating window
---@field float_config table can contain the same key-values pairs as `vim.api.nvim_open_win()`

---@class ConfigDebug
---@field runner function? custom runner used to launch a selected debug config

---@class Config
---@field task ConfigTask?
---@field debug ConfigDebug?
---@field insert_on_task_launch boolean? whether to auto-enter insert mode after launching task
M.defaults = {
  task = {
    runner = nil,
    display = 'float',
    -- rerun_replace_current = false, -- replace previous task or create unique using timestamp
    float_config = {
      relative = 'editor',
      border = 'rounded',
      title_pos = 'center',
      style = 'minimal',
    },
    -- shell_options = {}
  },
  debug = {
    runner = nil,
    -- optional DAP default template for each filetype (for smaller config files)
    -- ft_templates = {}
  },
  insert_on_task_launch = false,
  -- config_type 'directory' | 'stdpath'
}

---@type Config runtime user configuration
M.user = {} ---@diagnostic disable-line

---applies the argument options to the defaults and saves it as user config
---@param opts Config?
function M.apply(opts) M.user = util.merge(M.defaults, opts or {}) end

return M
