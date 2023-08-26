---------------------------------------- USER INPUT HANDLER ----------------------------------------

local util = require 'launch.util'

local M = {}

---@type table<string, UserVariable> mapping of user-defined variable names to their specifications
M.variables = {}

---@type boolean helper flag to stop substitution in the event of cancellation
local substitution_stopped = false

---callback function for global substitution of an argument
---@param name string matched `${input:...}` variable
---@return string? # replacement string
---@nodiscard
local function gsub_callback(name)
  if substitution_stopped then return end

  vim.cmd.redraw() -- clean up previous substitution
  local replacement
  M.variables[name]:get_user_choice(function(choice)
    ---@cast choice UserVarValue the user entered or selected value
    if not choice then
      -- if user does not enter or select anything, stop the substitution process
      substitution_stopped = true
      vim.cmd.redraw()
      util.notify('warn', 'Task runner launch cancelled')
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
  substitution_stopped = false

  local i = 1
  while i <= #args and not substitution_stopped do
    args[i] = args[i]:gsub('%${input:([_%a][_%w]*)}', gsub_callback)
    i = i + 1
  end

  return not substitution_stopped
end

return M
