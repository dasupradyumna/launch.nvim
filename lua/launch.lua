-------------------------------------------- LAUNCH-NVIM -------------------------------------------

local configs = require 'launch-nvim.configs'
local core = require 'launch-nvim.core'
local settings = require 'launch-nvim.settings'

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
  if not settings:ready() then return end

  -- REMOVE:
  ---@type LaunchNvimTaskConfig
  local test_config = {
    name = 'Launch Test',
    command = 'echo',
    args = { '{@greeting}', '$USERNAME', 'from {@country}', 'at', '"$PWD"!' },
    cwd = vim.fs.dirname(vim.uv.cwd()),
    display = 'float',
    -- env = { USERNAME = 'Pradyumna' },
  }

  coroutine.wrap(function()
    local variable = require 'launch-nvim.core.variable'
    local ok = variable:substitute_config(test_config)
    if not ok then error 'variable substitution failed' end

    core:run('TASK', test_config)
  end)()
end

function launch.debugger()
  if not settings:ready() then return end

  vim.notify 'Debugger launched'

  core:run 'DEBUG'
end

return launch
