"------------------------------------------- LAUNCH-NVIM ------------------------------------------"

" Clears undo history of the current buffer
" Refer :help clear-undo
function! LaunchNvimClearUndo()
    let old_undolevels = &l:undolevels
    setlocal undolevels=-1
    exe "normal a \<BS>\<Esc>"
    let &l:undolevels = old_undolevels
    unlet old_undolevels
endfunction

command! LaunchConfig lua require('launch').launch()

augroup launch_nvim
    autocmd!

    autocmd User LaunchNvimTaskWindowCreated
        \ execute "autocmd launch_nvim WinClosed" win_getid() "++once"
        \   printf("lua require('launch')._impl_.task.on_winclosed(%d)", w:launch_nvim_taskdisplay)

augroup END
