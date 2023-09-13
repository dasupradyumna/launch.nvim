------------------------------------------- LAUNCH.NVIM --------------------------------------------

local api = vim.api

-------------------------------- USER COMMANDS ---------------------------------

local function cmd(n, cb) api.nvim_create_user_command(n, cb, {}) end

cmd('LaunchTask', function() require('launch').task(true) end)
cmd('LaunchDebugger', function() require('launch').debugger(true) end)
cmd('LaunchShowTaskConfigs', function() require('launch').view('task', true) end)
cmd('LaunchShowActiveTasks', function() require('launch.task').show_active() end)
cmd('LaunchShowDebugConfigs', function() require('launch').view('debug', true) end)
-- HACK: implement the user variables viewer (similar to above)
cmd('LaunchShowUserVariables', function() vim.print(require('launch.user').variables) end)
cmd('LaunchOpenConfig', function() api.nvim_command 'vsplit .nvim/launch.lua' end)

------------------------------ USER AUTOCOMMANDS -------------------------------

api.nvim_create_augroup('launch_nvim', { clear = true })

api.nvim_create_autocmd('BufWritePost', {
  desc = 'Update the configurations whenever the launch file is modified',
  pattern = vim.uv.cwd() .. '/.nvim/launch.lua',
  callback = function()
    api.nvim_command 'redraw'
    require('launch.core').load_config_file()
  end,
  group = 'launch_nvim',
})

api.nvim_create_autocmd('TabClosed', {
  desc = 'Remove the cached task tabpage handle when it is closed',
  callback = function(trigger)
    local tab = require('launch.types.ActiveTask').renderer.tab
    local ok, tab_num = pcall(api.nvim_tabpage_get_number, tab.handle)
    if ok and tonumber(trigger.match, 10) == tab_num then tab.handle = nil end
  end,
  group = 'launch_nvim',
})

api.nvim_create_autocmd('WinClosed', {
  desc = 'Remove the cached task floating window handle when it is closed',
  callback = function(trigger)
    local float1 = require('launch.types.ActiveTask').renderer.float
    local float2 = require('launch.view').open_win
    if tonumber(trigger.match, 10) == float1.handle then float1.handle = nil end
    if tonumber(trigger.match, 10) == float2.handle then float2.handle = nil end
  end,
  group = 'launch_nvim',
})
