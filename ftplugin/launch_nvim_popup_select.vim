"--------------------------------------- SELECT POPUP BUFFER --------------------------------------"

" script guard
if exists('b:did_ftplugin') | finish | endif
let b:did_ftplugin = 1

function! s:callback()
    call b:callback()
    bwipeout!
endfunction

nnoremap <buffer> <CR> <Cmd>call <SID>callback()<CR>
nnoremap <buffer> <C-C> <Cmd>bwipeout!<CR>
" disable window navigation
nnoremap <buffer> <C-W> <NOP>
