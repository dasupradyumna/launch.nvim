"--------------------------------------- PROMPT POPUP BUFFER --------------------------------------"

" Script guard
if exists('b:did_ftplugin') | finish | endif
let b:did_ftplugin = 1

" Setup buffer as prompt
setlocal modifiable
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

" Cancel prompt and exit
inoremap <buffer> <C-C> <Cmd>bwipeout!<CR>
" Reset <C-W> to insert mode default behavior
inoremap <buffer> <nowait> <C-W> <C-\><C-O>dB
" Disable exiting insert mode
inoremap <buffer> <Esc> <NOP>
inoremap <buffer> <nowait> <C-G> <NOP>
inoremap <buffer> <nowait> <C-O> <NOP>
inoremap <buffer> <nowait> <C-\> <NOP>

" Start insert mode
call feedkeys('i')
