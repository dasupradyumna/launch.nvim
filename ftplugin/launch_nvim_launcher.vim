"--------------------------------------- LAUNCHER UI BUFFER ---------------------------------------"

" Script guard
if exists('b:did_ftplugin') | finish | endif
let b:did_ftplugin = 1

call launch#disable_all_keys()
call launch#setup_navigation()

function! s:remove_callbacks()
    if !exists('b:callbacks') | return | endif
    for [key; _] in b:callbacks
        execute 'silent nnoremap <buffer> <nowait>' key '<NOP>'
    endfor
endfunction
let b:remove_callbacks = function('s:remove_callbacks')
