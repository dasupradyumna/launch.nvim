"--------------------------------------- SELECT POPUP BUFFER --------------------------------------"

" Script guard
if exists('b:did_ftplugin') | finish | endif
let b:did_ftplugin = 1

call launch#disable_all_keys()
call launch#setup_navigation()

function! s:callback()
    call b:callback()
    bwipeout!
endfunction

nnoremap <buffer> <CR> <Cmd>call <SID>callback()<CR>
nnoremap <buffer> <C-C> <Cmd>bwipeout!<CR>
