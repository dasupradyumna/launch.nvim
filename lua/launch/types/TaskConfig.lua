------------------------------------------- TASK-CONFIG --------------------------------------------

local config = require 'launch.config'
local util = require 'launch.util'

---@class ShellOptions
---@field exec string path to the binary shell executable
---@field args? string[] command-line arguments to the shell executable
local ShellOptions = {}

---@class TaskOptions
---@field cwd? string current working directory of the shell which runs the task
---@field env? table<string, string> environment variables to set in shell before running task
---@field shell? ShellOptions optional shell config to use for running task
-- all above options (when specified) override the default shell environment
local TaskOptions = {}

---@class TaskConfig
---@field name string display name of the task
---@field command string command to be executed: can be an executable or a shell command
---@field args? string[] command-line arguments that follow the command
---@field display? DisplayType whether to render the task output in a tabpage or a floating window
---@field options? TaskOptions additional options configuring how the task is run
local TaskConfig = {}

---@class TaskConfigFromFile : TaskConfig
---@field filetype? string valid filetype that the task applies to
local TaskConfigFromFile = {}

---creates a new instance of `TaskConfig`
---@param cfg TaskConfigFromFile task configuration received from a file
---@return string ft filetype associated with the current task
---@return TaskConfig
---@nodiscard
---POSSIBLY THROWS ERROR
function TaskConfig:new(cfg)
  TaskConfigFromFile.validate_input(cfg)
  if cfg.options then
    TaskOptions.validate_input(cfg.name, cfg.options)
    if cfg.options.shell then ShellOptions.validate_input(cfg.name, cfg.options.shell) end
  end

  -- extract the filetype out of the config table
  local ft = cfg.filetype or ''
  cfg.filetype = nil

  -- set defaults
  cfg.display = cfg.display or config.user.task.display
  cfg.options = vim.tbl_deep_extend('force', config.user.task.options, cfg.options or {})
  if cfg.options.cwd then cfg.options.cwd = vim.fs.normalize(cfg.options.cwd) end

  return ft, setmetatable(cfg, { __index = self })
end

---@type table<string, table<string, boolean>>
---set of valid fields for `TaskConfigFromFile` and `TaskOptions`
local valid_fields = {
  taskconfigfromfile = {
    name = true,
    filetype = true,
    command = true,
    args = true,
    display = true,
    options = true,
  },
  taskoptions = { cwd = true, env = true, shell = true },
  shelloptions = { exec = true, args = true },
}

---checks and validates if the argument `cfg` is a valid `TaskConfigFromFile` object
---@param cfg table task configuration under validation
---POSSIBLY THROWS ERROR
function TaskConfigFromFile.validate_input(cfg)
  local msg, invalid_fields
  if type(cfg) == 'table' then
    invalid_fields = vim.tbl_filter(
      function(f) return not valid_fields.taskconfigfromfile[f] end,
      vim.tbl_keys(cfg)
    )
  end

  if type(cfg) ~= 'table' or vim.tbl_isempty(cfg) then
    msg = { 'should be a non-empty table. Got:\n%s', vim.inspect(cfg) }
  elseif not util.tbl_isdict(cfg) then
    local non_str = {}
    for k, v in pairs(cfg) do
      if type(k) ~= 'string' then non_str[tostring(k)] = v end
    end
    msg = {
      'should be a dictionary. Got the following key-value pairs with non-string keys:\n%s',
      vim.inspect(non_str),
    }
  elseif type(cfg.name) ~= 'string' then
    msg = { '`name` field should be a string. Got:\n%s', vim.inspect(cfg.name) }
  elseif #invalid_fields > 0 then
    msg = { '"%s" has the following invalid fields : %s', cfg.name, invalid_fields }
  elseif not vim.list_contains({ 'string', 'nil' }, type(cfg.filetype)) then
    msg =
      { '"%s" `filetype` (optional) field should be a string. Got:\n%s', cfg.name, cfg.filetype }
  elseif type(cfg.command) ~= 'string' then
    msg = { '"%s" `command` field should be a string. Got:\n%s', cfg.name, cfg.command }
  elseif type(cfg.display) ~= 'nil' and not vim.list_contains({ 'float', 'tab' }, cfg.display) then
    msg = {
      '"%s" `display` (optional) field should be either "float" or "tab". Got:\n%s',
      cfg.name,
      cfg.display,
    }
  elseif not vim.list_contains({ 'table', 'nil' }, type(cfg.options)) then
    msg = { '"%s" `options` (optional) field should be a table. Got:\n%s', cfg.name, cfg.options }
  elseif type(cfg.args) ~= 'nil' then
    if not vim.tbl_islist(cfg.args) or vim.tbl_isempty(cfg.args) then
      msg =
        { '"%s" `args` field should be a non-empty list-like table. Got:\n%s', cfg.name, cfg.args }
    else
      for _, a in ipairs(cfg.args) do
        if type(a) ~= 'string' then
          msg = { '"%s" `args` list should only contain strings. Got:\n%s', cfg.name, cfg.args }
          break
        end
      end
    end
  end

  if msg then util.throw_notify('E', 'Task config ' .. msg[1], msg[2], vim.inspect(msg[3])) end
end

---checks and validates if the argument `opts` is a valid `TaskOptions` object
---@param opts table task options under validation
---POSSIBLY THROWS ERROR
function TaskOptions.validate_input(name, opts)
  local msg, invalid_fields
  invalid_fields = vim.tbl_filter(
    function(f) return not valid_fields.taskoptions[f] end,
    vim.tbl_keys(opts)
  )

  if vim.tbl_isempty(opts) then
    msg = { '"%s" should be a non-empty dictionary' }
  elseif not util.tbl_isdict(opts) then
    local non_str = {}
    for k, v in pairs(opts) do
      if type(k) ~= 'string' then non_str[tostring(k)] = v end
    end
    msg = {
      '"%s" should be a dictionary. Got the following key-value pairs with non-string keys:\n%s',
      non_str,
    }
  elseif #invalid_fields > 0 then
    msg = { '"%s" has the following invalid fields : %s', invalid_fields }
  elseif not vim.list_contains({ 'string', 'nil' }, type(opts.cwd)) then
    msg = { '"%s" `cwd` field should be a string. Got:\n%s', opts.cwd }
  elseif not vim.list_contains({ 'table', 'nil' }, type(opts.shell)) then
    msg = { '"%s" `shell` (optional) field should be a table. Got:\n%s', opts.shell }
  elseif type(opts.env) ~= 'nil' then
    if not util.tbl_isdict(opts.env) or vim.tbl_isempty(opts.env) then
      msg = { '"%s" `env` field should be a non-empty dictionary-like table. Got:\n%s', opts.env }
    else
      for _, v in pairs(opts.env) do
        if not vim.list_contains({ 'string', 'number' }, type(v)) then
          msg = {
            '"%s" `env` dictionary should only contain strings or numbers as values. Got:\n%s',
            opts.env,
          }
          break
        end
      end
    end
  end

  if msg then
    util.throw_notify('E', 'Task config `options` for ' .. msg[1], name, vim.inspect(msg[2]))
  end
end

---checks and validates if the argument `opts` is a valid `ShellOptions` object
---@param opts table shell options under validation
---POSSIBLY THROWS ERROR
function ShellOptions.validate_input(name, opts)
  local msg, invalid_fields
  invalid_fields = vim.tbl_filter(
    function(f) return not valid_fields.shelloptions[f] end,
    vim.tbl_keys(opts)
  )

  if vim.tbl_isempty(opts) then
    msg = { '"%s" should be a non-empty dictionary' }
  elseif not util.tbl_isdict(opts) then
    local non_str = {}
    for k, v in pairs(opts) do
      if type(k) ~= 'string' then non_str[tostring(k)] = v end
    end
    msg = {
      '"%s" should be a dictionary. Got the following key-value pairs with non-string keys:\n%s',
      non_str,
    }
  elseif #invalid_fields > 0 then
    msg = { '"%s" has the following invalid fields : %s', invalid_fields }
  elseif type(opts.exec) ~= 'string' then
    msg = { '"%s" `exec` field should be a string. Got:\n%s', opts.exec }
  elseif type(opts.args) ~= 'nil' then
    if not vim.tbl_islist(opts.args) or vim.tbl_isempty(opts.args) then
      msg = { '"%s" `args` field should be a non-empty list-like table. Got:\n%s', opts.args }
    else
      for _, a in ipairs(opts.args) do
        if type(a) ~= 'string' then
          msg = { '"%s" `args` list should only contain strings. Got:\n%s', opts.args }
          break
        end
      end
    end
  end

  if msg then
    util.throw_notify('E', 'Task config `options.shell` for ' .. msg[1], name, vim.inspect(msg[2]))
  end
end

return TaskConfig
