"--------------------------------------- PROMPT POPUP BUFFER --------------------------------------"

" script guard
if exists('b:did_ftplugin') | finish | endif
let b:did_ftplugin = 1

function! s:callback(input)
    call b:callback(a:input)
    bwipeout!
endfunction

" setup buffer as prompt
setlocal buftype=prompt
let buffer = bufnr()
call prompt_setprompt(buffer, printf(' %s > ', b:prompt))
call prompt_setcallback(buffer, function('s:callback'))

" cancel prompt and exit
inoremap <buffer> <C-C> <Cmd>bwipeout!<CR>
" disable window navigation
inoremap <buffer> <C-W> <NOP>
" disable exiting insert mode
inoremap <buffer> <Esc> <NOP>
inoremap <buffer> <C-O> <NOP>

" start insert mode
call nvim_feedkeys(printf('i%s', b:default), 'n', v:false)
