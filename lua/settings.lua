------------------------------------------ PLUGIN SETTINGS -----------------------------------------

local utils = require 'launch-nvim.utils'

local settings = {}

---@type LaunchNvimSettings default plugin settings
settings.default = {
  confirm_choice = false,
  task = {
    display = 'float',
    env = {},
    insert_mode_on_launch = false,
  },
  debug = {},
}

---@type LaunchNvimSettings active plugin settings
---@diagnostic disable-next-line:missing-fields
settings.active = {}

---apply the user specified settings to internal active settings table
---@param user_settings? LaunchNvimSettings
function settings:apply(user_settings)
  -- validate the settings table provided by the user
  local ok = utils.validate.argument(user_settings, {
    { '[[user_settings]]', true, 'record', { 'confirm_choice', 'task', 'debug' } },
    { 'confirm_choice', true, 'boolean' },
    { 'task', true, 'record', { 'display', 'env', 'insert_mode_on_launch' } },
    { 'task.display', true, 'enum', { 'float', 'tabpage', 'vsplit', 'hsplit' } },
    { 'task.env', true, 'dict', { 'string', 'number' } },
    { 'task.insert_mode_on_launch', true, 'boolean' },
  }, 'Plugin setup failed! User settings could not be applied.')
  if not ok then return end

  self.active = vim.tbl_deep_extend('force', self.default, user_settings or {})
end

---indicates whether user settings have not been applied
---@nodiscard
function settings:failed() return vim.tbl_isempty(self.active) end

return settings
