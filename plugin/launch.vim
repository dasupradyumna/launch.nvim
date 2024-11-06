"------------------------------------------- LAUNCH-NVIM ------------------------------------------"

command LaunchTask lua require('launch').task()
command LaunchDebugger lua require('launch').debugger()

augroup launch_nvim

    autocmd DirChanged * lua require('launch-nvim.configs').load()

augroup END
