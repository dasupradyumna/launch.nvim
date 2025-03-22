"------------------------------------- LAUNCH-NVIM AUTOLOAD ---------------------------------------"

" Clears undo history of the current buffer
" Refer :help clear-undo
function! launch#clear_undo_history()
    let old_undolevels = &l:undolevels
    setlocal undolevels=-1
    exe "normal a \<BS>\<Esc>"
    let &l:undolevels = old_undolevels
    unlet old_undolevels
endfunction

" Sets up buffer-local callbacks defined by a list variable
function! launch#setup_callbacks()
    for idx in range(len(b:callbacks))
        execute printf("nnoremap <buffer> <nowait> %s <Cmd>call b:callbacks[%d][1]()<CR>",
                    \ b:callbacks[idx][0], idx)
    endfor
endfunction
