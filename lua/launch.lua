-------------------------------------------- LAUNCH-NVIM -------------------------------------------

local launch = {}

function launch.setup(settings) vim.print(settings) end

function launch.task() vim.notify 'Task launched' end

function launch.debugger() vim.notify 'Debugger launched' end

return launch
