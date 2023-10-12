----------------------------------------- CONFIG-FROM-FILE -----------------------------------------

local config = require 'launch.config'
local task = require 'launch.task'
local user = require 'launch.user'
local util = require 'launch.util'

local DebugConfig = require 'launch.types.DebugConfig'
local TaskConfig = require 'launch.types.TaskConfig'
local UserVariable = require 'launch.types.UserVariable'

---@class ConfigFromFile
---@field task? TaskConfigFromFile[]
---@field debug? DebugConfigFromFile[]
---@field var? table<string, UserVariable>
local ConfigFromFile = {}

---load all valid configurations from file
---@param cfg ConfigFromFile
---*[POSSIBLY THROWS ERROR]*
function ConfigFromFile:load(cfg)
  self.validate_input(cfg)

  -- load all tasks, debug configurations and user-defined variables from file
  self.load_from_task(cfg.task)
  if not config.user.debug.disable then self.load_from_debug(cfg.debug) end
  self.load_from_var(cfg.var)
end

---build configurations using the specified config builder
---@param configs DebugConfigFromFile[] | TaskConfigFromFile[]
---@param Builder DebugConfig | TaskConfig
---@return table<string, DebugConfig | TaskConfig>
local function build_configs(configs, Builder)
  local built = {}
  for _, raw_cfg in ipairs(configs) do
    local ft, cfg = Builder:new(raw_cfg)
    built[ft] = built[ft] or {}
    table.insert(built[ft], cfg)
  end

  return built
end

---load valid task configurations from file
---@param configs? TaskConfigFromFile[]
---*[POSSIBLY THROWS ERROR]*
function ConfigFromFile.load_from_task(configs)
  task.list = {} -- reset the tasks list for reloading
  if not configs then return end -- skip function if argument is empty

  task.list = build_configs(configs, TaskConfig)
end

---load valid debugger configurations from file
---@param configs? DebugConfigFromFile[]
---*[POSSIBLY THROWS ERROR]*
function ConfigFromFile.load_from_debug(configs)
  local dap = util.try_require 'dap'
  if not dap then return end

  dap.configurations = {} -- reset the debug configs list for reloading
  if not configs then return end -- skip function if argument is empty

  dap.configurations = build_configs(configs, DebugConfig)
end

---load valid user-defined variables from file
---@param variables? table<string, UserVariable>
---*[POSSIBLY THROWS ERROR]*
function ConfigFromFile.load_from_var(variables)
  user.variables = {} -- reset the variables list for reloading
  if not variables then return end -- skip function if argument is empty

  for name, var in pairs(variables) do
    variables[name] = UserVariable:new(name, var)
  end

  user.variables = variables
end

---@type table<string, boolean> set of valid fields for `ConfigFromFile`
local valid_fields = { task = true, debug = true, var = true }

---checks and validates if argument `cfg` is a valid `ConfigFromFile` object
---@param cfg table configuration table from file under validation
---*[POSSIBLY THROWS ERROR]*
function ConfigFromFile.validate_input(cfg)
  local msg, invalid_fields
  if type(cfg) == 'table' then
    invalid_fields = vim.tbl_filter(function(f) return not valid_fields[f] end, vim.tbl_keys(cfg))
  end

  if type(cfg) ~= 'table' or vim.tbl_isempty(cfg) then
    msg = { 'should return a non-empty table. Got:\n%s', cfg }
  elseif not util.tbl_isdict(cfg) then
    local non_str = {}
    for k, v in pairs(cfg) do
      if type(k) ~= 'string' then non_str[tostring(k)] = v end
    end
    msg = {
      'table should be a dictionary. Got the following key-value pairs with non-string keys:\n%s',
      non_str,
    }
  elseif #invalid_fields > 0 then
    msg = { 'table has the following invalid fields : %s', invalid_fields }
  elseif
    type(cfg.task) ~= 'nil' and (not vim.tbl_islist(cfg.task) or vim.tbl_isempty(cfg.task))
  then
    msg = { 'table `task` field should be a non-empty list-like table. Got:\n%s', cfg.task }
  elseif type(cfg.var) ~= 'nil' and (not util.tbl_isdict(cfg.var) or vim.tbl_isempty(cfg.var)) then
    msg = { 'table `input` field should be a non-empty dictionary-like table. Got:\n%s', cfg.var }
  elseif type(cfg.debug) ~= 'nil' then
    if config.user.debug.disable then
      msg = {
        'table `debug` field should not be specified; user has manually disabled debugger support. '
          .. 'Got:\n%s',
        cfg.debug,
      }
    elseif not util.try_require 'dap' then
      msg = {
        'table `debug` field has been specified, but the plugin `mfussenegger/nvim-dap` was not '
          .. 'found. Got:\n%s',
        cfg.debug,
      }
    elseif not vim.tbl_islist(cfg.debug) or vim.tbl_isempty(cfg.debug) then
      msg = { 'table `debug` field should be a non-empty list-like table. Got:\n%s', cfg.debug }
    end
  end

  if msg then util.throw_notify('E', 'Config file "launch.lua" ' .. msg[1], vim.inspect(msg[2])) end
end

return ConfigFromFile
