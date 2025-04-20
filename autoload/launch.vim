"------------------------------------- LAUNCH-NVIM AUTOLOAD ---------------------------------------"

" Clears undo history of the current buffer
" Refer :help clear-undo
function! launch#clear_undo_history()
    let old_undolevels = &l:undolevels
    setlocal undolevels=-1
    exe "normal a \<BS>\<Esc>"
    let &l:undolevels = old_undolevels
    unlet old_undolevels
endfunction

" Sets up buffer-local callbacks defined by a list variable
function! launch#setup_callbacks()
    for idx in range(len(b:callbacks))
        execute printf('silent nnoremap <buffer> <nowait> %s <Cmd>call b:callbacks[%d][1]()<CR>',
                    \ b:callbacks[idx][0], idx)
    endfor
endfunction

" Disables all default keymaps in a buffer
function! launch#disable_all_keys()
    const normal_keys = range(33, 126)->map('nr2char(v:val)')
    const ctrl_keys = range(33, 126)->map('printf("<C-%s>", nr2char(v:val))')
    const special_keys = ['<CR>', '<Tab>', '<Space>', '<Esc>', '<BS>', '<Bar>',
            \ '<Up>', '<Down>', '<Left>', '<Right>', '<S-Up>', '<S-Down>', '<S-Left>', '<S-Right>',
            \ '<Home>', '<End>', '<PageUp>', '<PageDown>', '<Insert>', '<Del>', '<Undo>',
            \ '<C-Up>', '<C-Down>', '<C-Left>', '<C-Right>']
    for key in normal_keys + ctrl_keys + special_keys
        try
            if key == '|' || key == '<C-|>' | continue | endif
            execute 'silent nnoremap <buffer> <nowait>' key '<NOP>'
        catch
            echom key 'has error:' v:exception
        endtry
    endfor

    " For user convenience
    nnoremap <buffer> <nowait> : :
endfunction

function! s:navigate(up)
    let cursor = line('.')
    let bounds = b:->get('bounds', [1, line('$')]) " Set default bounds to buffer size
    if a:up && cursor > bounds[0]
        normal! k
    elseif !a:up && cursor < bounds[1]
        normal! j
    endif
endfunction

function! launch#setup_navigation()
    " Up motion
    nnoremap <buffer> <nowait> j <Cmd>call <SID>navigate(v:false)<CR>
    nmap <buffer> <nowait> <Down> j
    nmap <buffer> <nowait> <C-N> j
    " Down motion
    nnoremap <buffer> <nowait> k <Cmd>call <SID>navigate(v:true)<CR>
    nmap <buffer> <nowait> <Up> k
    nmap <buffer> <nowait> <C-P> k
endfunction
