"--------------------------------------- ACTIVE TASKS BUFFER --------------------------------------"

" Script guard
if exists('b:did_ftplugin') | finish | endif
let b:did_ftplugin = 1

call launch#disable_all_keys()
call launch#setup_navigation()
