"--------------------------------------- PROMPT POPUP BUFFER --------------------------------------"

" script guard
if exists('b:did_ftplugin') | finish | endif
let b:did_ftplugin = 1

" setup buffer as prompt
setlocal buftype=prompt

function! s:callback(input)
    if a:input->empty() | return | endif

    call b:callback(a:input)
    if !exists('b:env_var') | bwipeout! | endif
endfunction
call prompt_setcallback(bufnr(), function('s:callback'))

function! s:update_prompt()
    call prompt_setprompt(bufnr(), printf(' %s > ', b:prompt))
    call feedkeys(b:default, 't')
endfunction
let b:update_prompt = function('s:update_prompt')

" cancel prompt and exit
inoremap <buffer> <C-C> <Cmd>bwipeout!<CR>
" disable window navigation
inoremap <buffer> <C-W> <NOP>
" disable exiting insert mode
inoremap <buffer> <Esc> <NOP>
inoremap <buffer> <C-O> <NOP>

" start insert mode
call feedkeys('i')
