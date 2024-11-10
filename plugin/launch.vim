"------------------------------------------- LAUNCH-NVIM ------------------------------------------"

command LaunchTask lua require('launch').task()
command LaunchDebugger lua require('launch').debugger()

" remove the cached window ID when that window is closed
function! s:remove_cached_window()
    let current = win_getid()
    let ids = v:lua.require('launch-nvim.ui.task').win_id

    for display in keys(ids)
        if current == ids[display]
            execute printf('lua require("launch-nvim.ui.task").win_id.%s = nil', display)
            return
        endif
    endfor
endfunction

augroup launch_nvim
    autocmd!

    autocmd DirChanged * lua require('launch-nvim.configs').load()

    autocmd WinClosed * call s:remove_cached_window()

augroup END
