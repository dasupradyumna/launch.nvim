------------------------------------------- LAUNCH.NVIM --------------------------------------------

local api = vim.api

-------------------------------- USER COMMANDS ---------------------------------

local function cmd(n, cb) api.nvim_create_user_command(n, cb, {}) end

cmd('LaunchTask', function() require('launch').task(true) end)
cmd('LaunchTaskFT', function() require('launch').task() end)
cmd('LaunchShowTaskConfigs', function() require('launch').view('task', true) end)
cmd('LaunchShowActiveTasks', function() require('launch').view 'active' end)
cmd('LaunchDebugger', function() require('launch').debugger(true) end)
cmd('LaunchDebuggerFT', function() require('launch').debugger() end)
cmd('LaunchShowDebugConfigs', function() require('launch').view('debug', true) end)
-- HACK: implement the user variables viewer (similar to above)
-- cmd('LaunchShowUserVariables', function() vim.print(require('launch.user').variables) end)
cmd('LaunchOpenConfigFile', function() api.nvim_command 'vsplit .nvim/launch.lua' end)

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
  desc = 'Remove cached floating window handles when the respective window is closed',
  callback = function(trigger)
    local view_handles = require('launch.view').handles
    if tonumber(trigger.match, 10) == rawget(view_handles, 'win') then view_handles.win = nil end
  end,
  group = 'launch_nvim',
})

-- TODO: add a VimResized autocommand for adjusting floating window size
