"------------------------------------------- LAUNCH-NVIM ------------------------------------------"

command! LaunchConfig lua require('launch').launch()

augroup launch_nvim
    autocmd!

    autocmd User LaunchNvimTaskWindowCreated
        \ execute "autocmd launch_nvim WinClosed" win_getid() "++once"
        \   printf("lua require('launch')._impl_.on_task_winclosed(%d)", w:launch_nvim_taskdisplay)

augroup END
