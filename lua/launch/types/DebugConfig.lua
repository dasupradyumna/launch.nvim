------------------------------------------- DEBUG-CONFIG -------------------------------------------

local config = require 'launch.config'
local util = require 'launch.util'

---@class DebugConfig
---mandatory DAP fields
---@field name string display name of the debug configuration
---@field type string? name of the debug adapter to use for launching
---@field request 'launch' | 'attach' whether to launch a program or connect to a running program
local DebugConfig = {}

---@class DebugConfigFromFile : DebugConfig
---@field filetype string valid filetype that the task applies to
local DebugConfigFromFile = {}

---creates a new instance of `DebugConfig`
---@param cfg DebugConfigFromFile debug configuration received from a file
---@return string ft filetype associated with the current debug config
---@return DebugConfig
---@nodiscard
---POSSIBLY THROWS ERROR
function DebugConfig:new(cfg)
  DebugConfigFromFile.validate_input(cfg)

  -- extract filetype out of the config and set the debug configuration's adapter accordingly
  local ft = cfg.filetype
  cfg.filetype = nil
  cfg.type = cfg.type or config.user.debug.adapters[ft] or ft

  -- merge with the filetype template if available
  cfg = util.merge(config.user.debug.templates[ft] or {}, cfg)
  return ft, setmetatable(cfg, { __index = self })
end

---checks and validates if the argument `cfg` is a valid `DebugConfigFromFile` object
---@param cfg table debug configuration under validation
---POSSIBLY THROWS ERROR
function DebugConfigFromFile.validate_input(cfg)
  local msg

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
  elseif type(cfg.filetype) ~= 'string' then
    msg = { '"%s" `filetype` field should be a string. Got:\n%s', cfg.name, cfg.filetype }
  elseif not vim.list_contains({ 'attach', 'launch' }, cfg.request) then
    msg = {
      '"%s" `request` field should be either "attach" or "launch". Got:\n%s',
      cfg.name,
      cfg.request,
    }
  end

  if msg then util.throw_notify('E', 'Debug config ' .. msg[1], msg[2], vim.inspect(msg[3])) end
end

return DebugConfig
