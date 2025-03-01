"-------------------------------------- TASK TERMINAL BUFFER --------------------------------------"

" script guard
if exists('b:did_ftplugin') | finish | endif
let b:did_ftplugin = 1

nnoremap <buffer> q <Cmd>quit<CR>

autocmd launch_nvim BufWipeout <buffer>
    \ execute printf("lua require('launch')._impl_.task.on_bufwipeout(%d)", expand('<abuf>'))
