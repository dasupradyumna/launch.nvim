----------------------------------------- CONFIG-FROM-FILE -----------------------------------------

local task = require 'launch.task'
local user = require 'launch.user'
local util = require 'launch.util'

local UserVariable = require 'launch.types.UserVariable'
local TaskConfig = require 'launch.types.TaskConfig'

---@class ConfigFromFile
---@field task TaskConfigFromFile[]?
---@field input table<string, UserVariable>?
local ConfigFromFile = {}

---load all valid configurations from file
---@param cfg ConfigFromFile
---POSSIBLY THROWS ERROR
function ConfigFromFile:load(cfg)
  self.validate_input(cfg)

  -- load all tasks and user-defined variables from file
  self.load_from_task(cfg.task)
  self.load_from_input(cfg.input)
end

---load valid task configurations from file
---@param configs TaskConfigFromFile[]?
---POSSIBLY THROWS ERROR
function ConfigFromFile.load_from_task(configs)
  task.list = {} -- reset the tasks list for reloading
  if not configs then return end -- skip function if argument is empty

  local tasks = {}
  for _, config in ipairs(configs) do
    local filetype, cfg = TaskConfig:new(config)
    tasks[filetype] = tasks[filetype] or {}
    table.insert(tasks[filetype], cfg)
  end

  task.list = tasks
end

---load valid user-defined variables from file
---@param variables table<string, UserVariable>
---POSSIBLY THROWS ERROR
function ConfigFromFile.load_from_input(variables)
  user.variables = {} -- reset the variables list for reloading
  if not variables then return end -- skip function if argument is empty

  for name, var in pairs(variables) do
    variables[name] = UserVariable:new(name, var)
  end

  user.variables = variables
end

---@type table<string, boolean> set of valid fields for `ConfigFromFile`
local valid_fields = { task = true, input = true }

---checks and validates if argument `cfg` is a valid `ConfigFromFile` object
---@param cfg table configuration table from file under validation
---POSSIBLY THROWS ERROR
function ConfigFromFile.validate_input(cfg)
  local msg, invalid_fields
  if type(cfg) == 'table' then
    invalid_fields = vim.tbl_filter(function(f) return not valid_fields[f] end, vim.tbl_keys(cfg))
  end

  if type(cfg) ~= 'table' or vim.tbl_isempty(cfg) then
    msg = { 'should return a non-empty table\n    Got: %s', cfg }
  elseif not util.tbl_isdict(cfg) then
    local non_str = {}
    for k, v in pairs(cfg) do
      if type(k) ~= 'string' then non_str[tostring(k)] = v end
    end
    msg = { 'table should be a dictionary\n    Got non-string key-value pairs: %s', non_str }
  elseif #invalid_fields > 0 then
    msg = { 'table has the following invalid fields : %s', invalid_fields }
  elseif
    type(cfg.task) ~= 'nil' and (not vim.tbl_islist(cfg.task) or vim.tbl_isempty(cfg.task))
  then
    msg = { 'table `task` field should be a non-empty list-like table\n    Got: %s', cfg.task }
  elseif
    type(cfg.input) ~= 'nil' and (not util.tbl_isdict(cfg.input) or vim.tbl_isempty(cfg.input))
  then
    msg =
      { 'table `input` field should be a non-empty dictionary-like table\n    Got: %s', cfg.input }
  end

  if msg then util.throw_notify('E', 'Config file "launch.lua" ' .. msg[1], vim.inspect(msg[2])) end
end

return ConfigFromFile
