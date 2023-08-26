--------------------------------------- USER-VARIABLE CLASS ----------------------------------------

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

---creates a new instance of `UserVariable`
---@param var table argument with fields to initialize a `UserVariable`
---@return UserVariable
---POSSIBLY THROWS ERROR
function UserVariable:new(name, var)
  self.__validate_input(name, var)

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

---checks and validates if argument `var` is a valid `UserVariable` object
---@param name string name of the variable
---@param var table variable specification under validation
---POSSIBLY THROWS ERROR
function UserVariable.__validate_input(name, var)
  -- FIX: add a link to the user variables schema (to-be-added) in error message
  if type(var) ~= 'table' then
    util.throw_notify('err', 'User variable `%s` should be a table', name)
  elseif not vim.list_contains({ 'input', 'select' }, var.type) then
    util.throw_notify(
      'err',
      'User variable `%s.type` field should be either "input" or "select"',
      name
    )
  elseif type(var.desc) ~= 'string' then
    util.throw_notify('err', 'User variable `%s.desc` field should be a string', name)
  elseif var.type == 'input' then
    if type(var.items) ~= 'nil' then
      util.throw_notify(
        'err',
        'User variable `%s.items` field should be defined only for "select" type variables',
        name
      )
    elseif not vim.list_contains({ 'boolean', 'string', 'number', 'nil' }, type(var.default)) then
      util.throw_notify(
        'err',
        'User variable `%s.default` field should be a `UserVarValue`, if defined',
        name
      )
    end
  elseif var.type == 'select' then
    if type(var.default) ~= 'nil' then
      util.throw_notify(
        'err',
        'User variable `%s.default` field should be defined only for "input" type variables',
        name
      )
    elseif type(var.items) == 'nil' then
      util.throw_notify(
        'err',
        'User variable `%s.items` field is not optional for "select" type variables',
        name
      )
    elseif not vim.tbl_islist(var.items) or vim.tbl_isempty(var.items) then
      util.throw_notify(
        'err',
        'User variable `%s.items` field should be a non-empty list-like table',
        name
      )
    end
    for _, i in ipairs(var.items) do
      if not vim.list_contains({ 'boolean', 'string', 'number' }, type(i)) then
        util.throw_notify(
          'err',
          'User variable `%s.items` field should only contain `UserVarValue` objects',
          name
        )
      end
    end
  end
end

return UserVariable
