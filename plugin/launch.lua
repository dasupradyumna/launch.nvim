------------------------------------------- LAUNCH.NVIM --------------------------------------------

local api = vim.api

-------------------------------- USER COMMANDS ---------------------------------

local function cmd(n, cb) api.nvim_create_user_command(n, cb, {}) end

cmd('LaunchShowTasks', function() vim.print(require('launch.task').list) end)
cmd('LaunchShowUserVariables', function() vim.print(require('launch.user').variables) end)
cmd('LaunchTask', function() require('launch').task(true) end)
cmd('LaunchOpenConfig', function() api.nvim_command 'vsplit .nvim/launch.lua' end)

------------------------------ USER AUTOCOMMANDS -------------------------------

api.nvim_create_augroup('launch_nvim', { clear = true })

api.nvim_create_autocmd('VimEnter', {
  desc = 'Update the configurations whenever the launch file is modified',
  callback = require('launch.config').update_config_list,
  group = 'launch_nvim',
})
api.nvim_create_autocmd('BufWritePost', {
  desc = 'Update the configurations whenever the launch file is modified',
  pattern = vim.loop.cwd() .. '/.nvim/launch.lua',
  callback = require('launch.config').update_config_list,
  group = 'launch_nvim',
})

api.nvim_create_autocmd('TabClosed', {
  desc = 'Remove the cached plugin tabpage handle when it is closed',
  callback = function(trigger)
    local tab = require('launch.types.ActiveTask').renderer.tab
    local ok, tab_num = pcall(api.nvim_tabpage_get_number, tab.handle)
    if ok and tonumber(trigger.match, 10) == tab_num then tab.handle = nil end
  end,
  group = 'launch_nvim',
})
