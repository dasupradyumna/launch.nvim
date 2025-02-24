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

nnoremap <buffer> q <Cmd>bwipeout<CR>
nnoremap <buffer> j <Cmd>call <SID>navigate(v:false)<CR>
nnoremap <buffer> k <Cmd>call <SID>navigate(v:true)<CR>
