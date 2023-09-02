------------------------------------------- LAUNCH.NVIM --------------------------------------------

local api = vim.api

-------------------------------- USER COMMANDS ---------------------------------

local function cmd(n, cb) api.nvim_create_user_command(n, cb, {}) end

cmd('LaunchTaskList', function() vim.print(require('launch.task').list) end)
cmd('LaunchUserVariables', function() vim.print(require('launch.user').variables) end)
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
    local handles = require('launch.task').handles
    local ok, launch_tab = pcall(api.nvim_tabpage_get_number, handles.tab)
    if ok and tonumber(trigger.match, 10) == launch_tab then handles.tab = nil end
  end,
  group = 'launch_nvim',
})
