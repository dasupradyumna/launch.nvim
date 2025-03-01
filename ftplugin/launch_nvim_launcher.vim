"--------------------------------------- LAUNCHER UI BUFFER ---------------------------------------"

" script guard
if exists('b:did_ftplugin') | finish | endif
let b:did_ftplugin = 1

function! s:navigate(up)
    " TODO: change from cursor position to something more robust for navigation
    let cursor = line('.')
    if a:up && cursor > b:bounds[0]
        normal! k
    elseif !a:up && cursor < b:bounds[1]
        normal! j
    endif
endfunction

nnoremap <buffer> j <Cmd>call <SID>navigate(v:false)<CR>
nnoremap <buffer> k <Cmd>call <SID>navigate(v:true)<CR>

nnoremap <buffer> q <Cmd>call b:callbacks.close()<CR>
nnoremap <buffer> <CR> <Cmd>call b:callbacks.run()<CR>

autocmd launch_nvim BufWipeout <buffer> lua require('launch')._impl_.launcher.on_bufwipeout()
