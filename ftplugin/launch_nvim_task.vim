"--------------------------------------------------------------------------------------------------"

" script guard
if exists('b:did_ftplugin') | finish | endif
let b:did_ftplugin = 1

nnoremap <buffer> q <Cmd>quit<CR>

autocmd launch_nvim BufWipeout <buffer>
    \ lua require('launch').__internal__.on_task_bufwipeout(tonumber(vim.fn.expand('<abuf>'), 10))
