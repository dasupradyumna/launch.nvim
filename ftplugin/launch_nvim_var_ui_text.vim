"------------------------------------- VARIABLE UI : TEXT MODE ------------------------------------"

" script guard
if exists('b:did_ftplugin') | finish | endif
let b:did_ftplugin = 1

" setup buffer as text prompt
let buf = bufnr()
setlocal buftype=prompt
call prompt_setprompt(buf, '> ')
call prompt_setcallback(buf, b:on_user_action)

" exit variable substitution
inoremap <buffer> <C-C> <Cmd>call b:on_user_action()<CR>
imap <buffer> <Esc> <C-C>

" disable window navigation
inoremap <buffer> <C-W> <NOP>
" prevent exiting insert mode
inoremap <buffer> <C-O> <NOP>
inoremap <buffer> <C-\><C-G> <NOP>
inoremap <buffer> <C-\><C-N> <NOP>
inoremap <buffer> <C-\><C-O> <NOP>
