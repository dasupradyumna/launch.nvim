----------------------------------------- TASK RUNNER LOGIC ----------------------------------------

local settings = require 'launch-nvim.settings'
local ui = require 'launch-nvim.ui.task'
local utils = require 'launch-nvim.utils'

local task = {}

---@type LaunchNvimActiveTask[] list of currently active tasks
task.active = {}

---sets up buffer-local keymaps and autocommands for the new task
---@param new_task LaunchNvimActiveTask new task config
---@private
function task:setup_buffer(new_task)
  -- create a buffer for the task
  new_task.buffer = vim.api.nvim_create_buf(false, true)
  vim.api.nvim_set_option_value('filetype', 'launch_nvim_task', { buf = new_task.buffer })
  vim.keymap.set('n', 'q', '<Cmd>quit<CR>', { buffer = new_task.buffer })

  -- add current task to list of active tasks
  self.active[new_task.buffer] = new_task
  vim.api.nvim_create_autocmd('BufWipeout', {
    desc = 'Remove task from list of active tasks when its buffer is wiped out',
    buffer = new_task.buffer,
    callback = function() self.active[new_task.buffer] = nil end,
    group = 'launch_nvim',
  })
end

---run the task specified by the argument config
---@param config LaunchNvimTaskConfig target task config
function task:run(config)
  local task_settings = settings.task.active
  local new_task = { config = vim.deepcopy(config) }

  -- apply defaults to optional configuration fields
  new_task.config.display = config.display or task_settings.display
  new_task.config.env = vim.tbl_deep_extend('force', task_settings.env, config.env or {})
  new_task.config.cwd = vim.fs.normalize(config.cwd or vim.uv.cwd())

  -- ensure that task directory is valid and exists
  if not vim.uv.fs_stat(new_task.config.cwd) then
    utils.notify:error(
      ('Task config "%s" cwd is not a valid directory.\nGot CWD: %s'):format(
        config.name,
        new_task.config.cwd
      )
    )
    return
  end

  -- setup buffer and UI for the new task
  self:setup_buffer(new_task)
  ui:open(new_task)

  -- launch the task in the terminal buffer
  vim.fn.termopen(table.concat({ config.command, unpack(config.args or {}) }, ' '), {
    clear_env = false,
    cwd = new_task.config.cwd,
    env = new_task.config.env,
  })

  -- start insert mode if user enabled that option
  if task_settings.insert_mode_on_launch then vim.api.nvim_command 'startinsert' end
end

return task
