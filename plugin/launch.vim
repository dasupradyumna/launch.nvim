"------------------------------------------- LAUNCH-NVIM ------------------------------------------"

command! LaunchConfig lua require('launch').open()
command! LaunchListActiveTasks lua require('launch').list_active_tasks()

augroup launch_nvim
    autocmd!

    " Handle task windows when they are closed
    autocmd User LaunchNvimTaskWindowCreated
        \ execute "autocmd launch_nvim WinClosed" win_getid() "++once"
        \   printf("lua require('launch')._impl_.task.on_winclosed('%s')", w:taskdisplay)

    " Support for changing working director inside neovim
    autocmd DirChanged * lua require('launch')._impl_.dirchanged.post()
    autocmd DirChangedPre * lua require('launch')._impl_.dirchanged.pre()

augroup END
