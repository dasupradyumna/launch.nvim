---------------------------------------- ARGUMENT VALIDATION ---------------------------------------

local notify = require 'launch-nvim.utils.notify'

local validate = {}

---@type table<string, fun(target: any, rules: string[]): string?> validator methods table
local validators = setmetatable({}, {
  __index = function(_, valid_type)
    -- fall back to builtin type checker
    return function(target)
      return type(target) == valid_type and nil
        or ('option has value of type "%s".'):format(type(target))
    end
  end,
})

---performs dictionary validation
--- 1. must be a table
--- 2. every key must be a string
--- 3. every value must be of an allowed type
---@param target any value under validation
---@param types string[] list of allowed types for values
---@return string? # nil if successful, else returns error message
function validators.dict(target, types)
  if type(target) ~= 'table' then return 'must be a "table".' end
  for key, value in pairs(target) do
    if type(key) ~= 'string' then
      return 'must have "string" keys. Instead has this key: ' .. vim.inspect(key)
    elseif not vim.list_contains(types, type(value)) then
      return ('has a value of type "%s" for key "%s". Allowed value types: %s'):format(
        type(value),
        key,
        vim.inspect(types)
      )
    end
  end
end

---performs enumeration validation
---  - target must be one of enum values
---@param target any value under validation
---@param values string[] list of allowed types for values
---@return string? # nil if successful, else returns error message
function validators.enum(target, values)
  return vim.list_contains(values, target) and nil
    or ('must be one of %s.'):format(vim.inspect(values))
end

---performs list validation
--- 1. must be a table
--- 2. must have continuous integer indices
--- 3. every element must be of an allowed type
---@param target any value under validation
---@param types string[] list of allowed types for values
---@return string? # nil if successful, else returns error message
function validators.list(target, types)
  if type(target) ~= 'table' then return 'must be a "table".' end
  local idx = 0
  for key, value in pairs(target) do
    idx = idx + 1
    if key ~= idx then
      return 'must have continuous "integer" indices. Instead has this index: ' .. vim.inspect(key)
    elseif not vim.list_contains(types, type(value)) then
      return ('has an element of type "%s" at index %d. Allowed element types: %s'):format(
        type(value),
        idx,
        vim.inspect(types)
      )
    end
  end
end

---performs record validation
--- 1. must be a table
--- 2. must only contain specified fields
---@param target any value under validation
---@param fields string[] list of allowed types for values
---@return string? # nil if successful, else returns error message
function validators.record(target, fields)
  if type(target) ~= 'table' then return 'must be a "table".' end
  for key in pairs(target) do
    if not vim.list_contains(fields, key) then
      return ('has unknown field "%s". Allowed fields: %s'):format(key, vim.inspect(fields))
    end
  end
end

---perform argument validation according to specifications
---@param value any argument under validation
---@param spec_list LaunchNvimValidationSpec[] list of validation specifications
---@param failure_msg string header message in case of failure
---@return boolean # whether validation was successful or not
function validate.argument(value, spec_list, failure_msg)
  ---@type string[] list of error strings from checking every specification
  local error_list = { failure_msg, '' }
  for _, spec in ipairs(spec_list) do
    local spec_name, optional, valid_type, extra_info
    spec_name = spec[1]
    optional = spec[2]
    valid_type = spec[3]
    extra_info = spec[4]

    -- NOTE: pcall() here ensures nesting-into-nil cases are safely skipped
    local ok, target = pcall(function()
      local n_subs
      spec_name, n_subs = spec_name:gsub('^%[%[(.+)%]%]$', '%1')
      -- get target as 'value' directly or as nested key into 'value' if specified
      if n_subs == 1 then
        return value
      else
        return vim.tbl_get(value, unpack(vim.split(spec_name, '.', { plain = true })))
      end
    end)
    if not ok then goto continue end

    local error_msg
    if type(target) == 'nil' then
      -- check if optional is satisfied
      if not optional then
        error_msg = ('  > "%s" %s must not be nil.'):format(spec_name, valid_type)
      end
    else
      -- check if type specification is satisfied with the respective validator
      ---@diagnostic disable-next-line:param-type-mismatch
      error_msg = validators[valid_type](target, extra_info)
      if error_msg then error_msg = ('  > "%s" %s %s'):format(spec_name, valid_type, error_msg) end
    end

    table.insert(error_list, error_msg)
    ::continue::
  end
  table.insert(error_list, '')

  -- send an error notification if any
  if #error_list > 3 then
    notify:error(error_list)
    return false
  end

  return true
end

return validate
