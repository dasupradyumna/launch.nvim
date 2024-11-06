-------------------------------------------- LAUNCH-NVIM -------------------------------------------

local configs = require 'launch-nvim.configs'
local settings = require 'launch-nvim.settings'

local launch = {}

---plugin setup function
-- TODO: add meta file with type definitions
---@param user_settings? table
function launch.setup(user_settings)
  settings:apply(user_settings)

  -- ensure plugin data directory exists and load configs for CWD
  vim.fn.mkdir(configs.data_dir, 'p')
  configs:load()
end

function launch.task() vim.notify 'Task launched' end

function launch.debugger() vim.notify 'Debugger launched' end

return launch
