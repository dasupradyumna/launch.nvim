"------------------------------------------- LAUNCH-NVIM ------------------------------------------"

command! LaunchConfig lua require('launch').launch()
command! LaunchListActiveTasks lua require('launch').list_active_tasks()

augroup launch_nvim
    autocmd!

    autocmd User LaunchNvimTaskWindowCreated
        \ execute "autocmd launch_nvim WinClosed" win_getid() "++once"
        \   printf("lua require('launch')._impl_.task.on_winclosed(%d)", w:taskdisplay)

augroup END
