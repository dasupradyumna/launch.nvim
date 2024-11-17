------------------------------------ VARIABLE SUBSTITUTION LOGIC -----------------------------------

local configs = require 'launch-nvim.configs'
local utils = require 'launch-nvim.utils'
local var_ui = require 'launch-nvim.ui.variable'

local variable = {}

---perform variable substitution for the argument variable
---
---! **THROWS ERROR**
---@param target string name of variable to substitute
---@return string? # substitution string
---@nodiscard
---@private
function variable:substitute_variable(target)
  local var_config = configs.list.variable[target]
  if not var_config then
    utils.notify:throw {
      'Task runner launch cancelled!',
      ('  Defintion of variable "%s" not found.'):format(target),
    }
  end

  -- get user input for current variable
  local user_input = var_ui:open(var_config)
  if not user_input then
    utils.notify:throw {
      'Task runner launch cancelled!',
      ('  Substitution of variable "%s" failed ; did not receive user input.'):format(target),
    }
  end

  return user_input
end

---substitute all (if any) defined variable instances in the argument string
---
---! **THROWS ERROR**
---@param target string
---@return string # argument string with variables substituted (if any)
---@nodiscard
---@private
function variable:substitute_string(target)
  local iter = 1
  local start_idx, end_idx, var_name
  local output = {}

  -- iterate as long as the pattern finds a match
  while true do
    start_idx, end_idx, var_name = target:find('{@([_%w]+)}', iter)
    if not start_idx then
      table.insert(output, target:sub(iter))
      break
    end

    table.insert(output, target:sub(iter, start_idx - 1))
    table.insert(output, self:substitute_variable(var_name))

    iter = end_idx + 1
  end

  return table.concat(output)
end

---perform variable substitution on all string fields in the argument config (will be mutated)
---
---! **THROWS ERROR**
---@param target table<string, any> config for in-place substitution
function variable:substitute_config(target)
  -- iterate and perform substitution over all string and table fields
  for key, value in pairs(target) do
    if type(value) == 'string' then
      -- NOTE: custom function was required since string.gsub did not work with Lua coroutines
      value = self:substitute_string(value)
    elseif type(value) == 'table' then
      self:substitute_config(value)
    end

    target[key] = value
  end
end

return variable
