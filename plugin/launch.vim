"------------------------------------------- LAUNCH-NVIM ------------------------------------------"

command LaunchTask lua require('launch').task()
command LaunchDebugger lua require('launch').debugger()
command LaunchTaskState lua require('launch').task_state()

augroup launch_nvim
    autocmd!

augroup END
