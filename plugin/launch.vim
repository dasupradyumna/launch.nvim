"------------------------------------------- LAUNCH-NVIM ------------------------------------------"

"-------------------------------- USER COMMANDS -------------------------------"

command LaunchTask lua require('launch').task()
command LaunchDebugger lua require('launch').debugger()

"-------------------------------- AUTOCOMMANDS --------------------------------"

function! s:remove_cached_window()
    let current = win_getid()
    let ids = v:lua.require('launch-nvim.ui.task').win_id

    " iterate over all cached display types
    for display in keys(ids)
        if current == ids[display]
            execute printf('lua require("launch-nvim.ui.task").win_id.%s = nil', display)
            return
        endif
    endfor
endfunction

augroup launch_nvim
    autocmd!

    " load all configs (if they exist) for CWD
    autocmd DirChanged * lua require('launch-nvim.configs').load()

    " remove cached window ID when that window is closed
    autocmd WinClosed * call s:remove_cached_window()

augroup END
