------------------------------------------ USER-VARIABLE -------------------------------------------

local util = require 'launch.util'

---@alias UserVarType 'input' | 'select'
---@alias UserVarValue boolean | string | number

---@class UserVariable
---@field type UserVarType user input method; either entered at a prompt or selected from a list
---@field desc string a description for the input variable which is displayed in user input prompt
---@field default UserVarValue? a default value for an 'input' type variable (ignored for 'select')
---@field items UserVarValue[]? a list of choices for a 'select' type variable (ignored for 'input')
---@field get_user_choice fun(self:UserVariable, callback:function) user input processsing function
local UserVariable = {}

---@type function
local validate_input -- defined at the bottom of the file

---creates a new instance of `UserVariable`
---@param var table argument with fields to initialize a `UserVariable`
---@return UserVariable
---@nodiscard
---POSSIBLY THROWS ERROR
function UserVariable.new(name, var)
  validate_input(name, var)

  local methods = {}
  methods.get_user_choice = var.type == 'input' and UserVariable.get_user_choice_input
    or UserVariable.get_user_choice_select
  return setmetatable(var, { __index = methods })
end

---receive input from the user via a prompt by `vim.ui.input`
---@param callback fun(choice: UserVarValue) the value entered by user at the prompt
function UserVariable:get_user_choice_input(callback)
  vim.ui.input({ prompt = self.desc .. ': ', default = self.default }, callback)
end

---receive input from the user through selection from a list using `vim.ui.select`
---@param callback fun(choice: UserVarValue) the value selected by user from the list of choices
function UserVariable:get_user_choice_select(callback)
  vim.ui.select(self.items, { prompt = self.desc .. ':' }, callback)
end

---@type table<string, boolean> set of valid fields for `UserVariable`
local valid_fields = { type = true, desc = true, default = true, items = true }

---checks and validates if argument `var` is a valid `UserVariable` object
---@param name string name of the variable
---@param var table variable specification under validation
---POSSIBLY THROWS ERROR
function validate_input(name, var)
  -- FIX: add a link to the user variables schema (to-be-added) in error message
  local msg, invalid_fields
  if type(var) == 'table' then
    invalid_fields = vim.tbl_filter(function(f) return not valid_fields[f] end, vim.tbl_keys(var))
  end

  if type(var) ~= 'table' then
    msg = { '`%s` should be a table\n    Got: %s', var }
  elseif #invalid_fields > 0 then
    msg = { '`%s` has the following invalid fields : %s', invalid_fields }
  elseif not vim.list_contains({ 'input', 'select' }, var.type) then
    msg = { '`%s.type` field should be either "input" or "select"\n    Got: %s', var.type }
  elseif type(var.desc) ~= 'string' then
    msg = { '`%s.desc` field should be a string\n    Got: %s', var.desc }
  elseif var.type == 'input' then
    if type(var.items) ~= 'nil' then
      msg = { '`%s.items` field should not be defined; it is an "input" type variable' }
    elseif not vim.list_contains({ 'boolean', 'string', 'number', 'nil' }, type(var.default)) then
      msg = { '`%s.default` (optional) field should be a `UserVarValue`\n    Got: %s', var.default }
    end
  elseif var.type == 'select' then
    if type(var.default) ~= 'nil' then
      msg = { '`%s.default` field should not be defined; it is a "select" type variable' }
    elseif type(var.items) == 'nil' then
      msg = { '`%s.items` field is missing and should be defined; it is a "select" type variable' }
    elseif not vim.tbl_islist(var.items) or vim.tbl_isempty(var.items) then
      msg = { '`%s.items` field should be a non-empty list-like table\n    Got: %s', var.items }
    else
      for _, i in ipairs(var.items) do
        if not vim.list_contains({ 'boolean', 'string', 'number' }, type(i)) then
          msg =
            { '`%s.items` list should only contain `UserVarValue` objects\n    Got: %s', var.items }
          break
        end
      end
    end
  end

  if msg then util.throw_notify('err', 'User variable ' .. msg[1], name, vim.inspect(msg[2])) end
end

return UserVariable
