------------------------------------------- TASK-CONFIG --------------------------------------------

local config = require 'launch.config'
local util = require 'launch.util'

---@class TaskConfig
---@field name string display name of the task
---@field command string command to be executed: can be an executable or a shell command
---@field args string[]? command-line arguments that follow the command
---@field display DisplayType? whether to render the task output in a tabpage or a floating window
local TaskConfig = {}

---@class TaskConfigFromFile : TaskConfig
---@field type string? valid filetype that the task applies to
local TaskConfigFromFile = {}

---creates a new instance of `TaskConfig`
---@param cfg TaskConfigFromFile task configuration received from a file
---@return string ft filetype associated with the current task
---@return TaskConfig
---@nodiscard
---POSSIBLY THROWS ERROR
function TaskConfig.new(cfg)
  TaskConfigFromFile.validate_input(cfg)

  -- extract the filetype out of the config table
  local ft = cfg.type or ''
  cfg.type = nil

  -- set defaults
  cfg.display = cfg.display or config.user.task.display

  return ft, setmetatable(cfg, { __index = TaskConfig })
end

---@type table<string, boolean> set of valid fields for `TaskConfigFromFile`
local valid_fields = { name = true, type = true, command = true, args = true, display = true }

---checks and validates if the argument `cfg` is a valid `TaskConfigFromFile` object
---@param cfg table task configuration under validation
---POSSIBLY THROWS ERROR
function TaskConfigFromFile.validate_input(cfg)
  local msg, invalid_fields
  if type(cfg) == 'table' then
    invalid_fields = vim.tbl_filter(function(f) return not valid_fields[f] end, vim.tbl_keys(cfg))
  end

  if type(cfg) ~= 'table' then
    msg = { 'should be a table\n    Got: %s', vim.inspect(cfg) }
  elseif type(cfg.name) ~= 'string' then
    msg = { '`name` field should be a string\n    Got: %s', vim.inspect(cfg.name) }
  elseif #invalid_fields > 0 then
    msg = { '"%s" has the following invalid fields : %s', cfg.name, invalid_fields }
  elseif not vim.list_contains({ 'string', 'nil' }, type(cfg.type)) then
    msg = { '"%s" `type` (optional) field should be a string\n    Got: %s', cfg.name, cfg.type }
  elseif type(cfg.command) ~= 'string' then
    msg = { '"%s" `command` field should be a string\n    Got: %s', cfg.name, cfg.command }
  elseif type(cfg.display) ~= 'nil' and not vim.list_contains({ 'float', 'tab' }, cfg.display) then
    msg = {
      '"%s" `display` (optional) field should be either "float" or "tab"\n    Got: %s',
      cfg.name,
      cfg.display,
    }
  elseif type(cfg.args) ~= 'nil' then
    if not vim.tbl_islist(cfg.args) or vim.tbl_isempty(cfg.args) then
      msg = {
        '"%s" `args` field should be a non-empty list-like table\n    Got: %s',
        cfg.name,
        cfg.args,
      }
    else
      for _, a in ipairs(cfg.args) do
        if type(a) ~= 'string' then
          msg = { '"%s" `args` list should only contain strings\n    Got: %s', cfg.name, cfg.args }
          break
        end
      end
    end
  end

  if msg then util.throw_notify('err', 'Task config ' .. msg[1], msg[2], vim.inspect(msg[3])) end
end

return TaskConfig
