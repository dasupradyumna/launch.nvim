-------------------------------------------- LAUNCH-NVIM -------------------------------------------

local configs = require 'launch-nvim.configs'
local settings = require 'launch-nvim.settings'
local utils = require 'launch-nvim.utils'

local launch = {}

---plugin setup function
---@param user_settings? LaunchNvimSettings
function launch.setup(user_settings)
  settings:apply(user_settings)

  -- ensure plugin data directory exists and load configs for CWD
  vim.fn.mkdir(configs.data_dir, 'p')
  configs:load()
end

function launch.task()
  if settings:failed() then
    utils.notify:error {
      'Plugin has not been setup correctly and cannot be used.',
      'Please fix the issue and reload it.',
    }
    return
  end

  vim.notify 'Task launched'
end

function launch.debugger()
  if settings:failed() then
    utils.notify:error {
      'Plugin has not been setup correctly and cannot be used.',
      'Please fix the issue and reload it.',
    }
    return
  end

  vim.notify 'Debugger launched'
end

return launch
