"--------------------------------------- LAUNCHER UI BUFFER ---------------------------------------"

" script guard
if exists('b:did_ftplugin') | finish | endif
let b:did_ftplugin = 1

setlocal nomodifiable

function! s:navigate(up)
    " TODO: change from cursor position to something more robust for navigation
    let cursor = line('.')
    if a:up && cursor > b:bounds[0]
        normal! k
    elseif !a:up && cursor < b:bounds[1]
        normal! j
    endif
endfunction

nnoremap <buffer> <nowait> j <Cmd>call <SID>navigate(v:false)<CR>
nnoremap <buffer> <nowait> k <Cmd>call <SID>navigate(v:true)<CR>
" disable window navigation
nnoremap <buffer> <C-W> <NOP>

"--------------------- CALLBACK LOGIC ---------------------"

function! s:setup_callbacks()
    for idx in range(len(b:callbacks))
        execute printf("nnoremap <buffer> <nowait> %s <Cmd>call b:callbacks[%d][1]()<CR>",
                    \ b:callbacks[idx][0], idx)
    endfor
endfunction
let b:setup_callbacks = function('s:setup_callbacks')
