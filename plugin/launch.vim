command LaunchTaskList lua =require'launch.task'.list
command LaunchUserVariables lua =require'launch.user'.variables
command LaunchUpdateConfig lua require'launch.config'.update_config_list()
command LaunchTask lua require'launch'.task(true)
command LaunchOpenConfig vsplit .nvim/launch.lua
