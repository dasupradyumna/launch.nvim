---------------------------------------- USER INPUT HANDLER ----------------------------------------

local util = require 'launch.util'

local M = {}

---@type table<string, UserVariable> mapping of user-defined variable names to their specifications
M.variables = {}

---callback function for global substitution of an argument
---@param name string matched `${input:...}` variable
---@return string? # replacement string
---@nodiscard
---*[POSSIBLY THROWS ERROR]*
local function gsub_callback(name)
  vim.api.nvim_command 'redraw' -- clean up previous substitution
  local replacement
  local var = M.variables[name]
  if not var then util.throw_notify('E', 'User variable "%s" not defined', name) end

  var:get_user_choice(function(choice)
    if not choice then
      -- if user does not enter or select anything, stop substitution
      vim.api.nvim_command 'redraw'
      util.throw_notify('W', 'Task runner launch cancelled')
    end
    replacement = choice
  end)
  return replacement
end

---substitution of argument strings with user input
---@param args string[] list of argument strings to substitute
---@return boolean # whether substitution was successful or not
---@nodiscard
function M.substitute_variables(args)
  local ok
  for i = 1, #args do
    ok, args[i] = pcall(string.gsub, args[i], '{@([_%a][_%w]*)}', gsub_callback)
    if not ok then return false end
  end

  return true
end

return M
