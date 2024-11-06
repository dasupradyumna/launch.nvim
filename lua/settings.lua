------------------------------------------ PLUGIN SETTINGS -----------------------------------------

local utils = require 'launch-nvim.utils'

local settings = {}

---@type table default plugin settings
settings.default = {}

---@type table active plugin settings
settings.active = {}

-- TODO: add meta file with type definitions
---apply the user specified settings to internal active settings table
---@param user_settings? table
function settings:apply(user_settings)
  utils.validate_args()

  self.active = vim.tbl_deep_extend('force', self.default, user_settings or {})
end

return settings
