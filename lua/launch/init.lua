------------------------------------------- LAUNCH.NVIM --------------------------------------------

local config = require 'launch.config'
local core = require 'launch.core'
local task = require 'launch.task'
local util = require 'launch.util'

local M = {}

---plugin setup function
---@param opts? PluginConfig
function M.setup(opts)
  config.apply(opts)

  -- checking for debugger support via nvim-dap
  if config.user.debug.disable then
    vim.api.nvim_del_user_command 'LaunchDebugger'
    vim.api.nvim_del_user_command 'LaunchDebuggerFT'
    vim.api.nvim_del_user_command 'LaunchShowDebugConfigs'
    vim.api.nvim_del_user_command 'LaunchShowDebugConfigsFT'
  else
    util.try_require('dap', true)
  end

  local data_folder = vim.fn.stdpath 'data' .. '/launch_nvim'
  vim.fn.mkdir(data_folder, 'p', 493)
  core.config_file_path = ('%s/%s.lua'):format(
    data_folder,
    vim.loop.cwd():gsub('@', '@@'):gsub('[\\/:]', '@')
  )

  util.no_notify = true
  core.load_config_file()
  util.no_notify = false
end

---displays available tasks to the user and launches the selected task
---@param show_all_fts? boolean whether to display all tasks or only based on current filetype
function M.task(show_all_fts)
  local run = config.user.task.runner or task.runner
  core.start('task', show_all_fts, task.list, run)
end

---displays available debug configurations to the user and launches the selected config
---@param show_all_fts? boolean whether to display all configs or only based on current filetype
function M.debugger(show_all_fts)
  if config.user.debug.disable then
    util.notify('E', 'Debugger support has been manually disabled by the user')
    return
  end

  local dap = util.try_require('dap', true)
  if not dap then return end

  if dap.session() then
    util.notify('W', 'Debug session already active; please terminate current session first')
    return
  end

  local run = config.user.debug.runner or dap.run
  core.start('debug', show_all_fts, dap.configurations, run)
end

---displays the list of available configurations based on the type
---@param type ViewType the type of content that will be displayed
---@param show_all_fts? boolean whether to display all configs or only based on current filetype
function M.view(type, show_all_fts)
  -- checking for debugger support via nvim-dap
  if type == 'debug' then
    if config.user.debug.disable then
      util.notify('E', 'Debugger support has been manually disabled by the user')
      return
    elseif not util.try_require('dap', true) then
      return
    end
  end

  require('launch.view').render(type, show_all_fts)
end

return M
