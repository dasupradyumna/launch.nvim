"------------------------------------------- LAUNCH-NVIM ------------------------------------------"

command LaunchTask lua require('launch').task()
command LaunchDebugger lua require('launch').debugger()
command LaunchShowActive lua require('launch').show_active()

augroup launch_nvim
    autocmd!

augroup END
