------------------------------------------ PLUGIN SETTINGS -----------------------------------------

local utils = require 'launch-nvim.utils'

---@class LaunchNvimSettingsModule
---@field private default LaunchNvimSettings default plugin settings
---@field active LaunchNvimSettings active plugin settings
local settings = {
  default = {
    confirm_choice = false,
    task = {
      ui = {
        display = 'float',
        float = {
          size = 'medium',
          config = {
            title_pos = 'center',
            footer_pos = 'right',
            border = 'rounded',
            zindex = 49, -- one unit lesser than neovim default
          },
        },
        hsplit_height = 30,
        vsplit_width = 50,
      },
      env = {},
      insert_mode_on_launch = false,
    },
    debug = {},
  },

  ---@diagnostic disable-next-line:missing-fields
  active = {},
}

---apply the user specified settings to internal active settings table
---@param user_settings? LaunchNvimSettings
function settings:apply(user_settings)
  -- validate the settings table provided by the user
  local ok = utils.validate.argument(user_settings, {
    { '[[user_settings]]', true, 'record', { 'confirm_choice', 'task', 'debug' } },
    { 'confirm_choice', true, 'boolean' },
    { 'task', true, 'record', { 'ui', 'env', 'insert_mode_on_launch' } },
    { 'task.ui', true, 'record', { 'display', 'float', 'hsplit_height', 'vsplit_width' } },
    { 'task.ui.display', true, 'enum', { 'float', 'vsplit', 'hsplit' } },
    { 'task.ui.float', true, 'record', { 'size', 'config' } },
    { 'task.ui.float.size', true, 'enum', { 'small', 'medium', 'large' } },
    { 'task.ui.float.config', true, 'record', { 'title_pos', 'footer_pos', 'border', 'zindex' } },
    { 'task.ui.hsplit_height', true, 'number' },
    { 'task.ui.vsplit_width', true, 'number' },
    { 'task.env', true, 'dict', { 'string', 'number' } },
    { 'task.insert_mode_on_launch', true, 'boolean' },
  }, 'Plugin setup failed! User settings could not be applied.')
  if not ok then return end

  self.active = vim.tbl_deep_extend('force', self.default, user_settings or {})
end

---indicates whether user settings have been applied and ready to use
---@nodiscard
function settings:ready()
  local failed = vim.tbl_isempty(self.active)

  -- send error notification if settings have not been applied
  if failed then
    utils.notify:error {
      'Plugin has not been setup correctly and cannot be used.',
      'Please fix the issue and reload it.',
    }
  end

  return not failed
end

return settings
