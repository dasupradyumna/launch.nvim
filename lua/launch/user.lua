---------------------------------------- USER INPUT HANDLER ----------------------------------------

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
  local var = M.variables[name]

  ---callback function for neovim UI functions
  ---@param choice UserVarValue the user entered or selected value
  local function ui_callback(choice)
    -- if user does not enter or select anything, stop the substitution process
    if not choice then
      substitution_stopped = true
      vim.cmd.redraw()
      vim.notify('[launch.nvim] Task runner launch cancelled', vim.log.levels.WARN)
    end
    replacement = choice
  end

  -- get user choice using the specified method of input
  if var.type == 'input' then
    vim.ui.input({ prompt = var.desc .. ': ', default = var.default }, ui_callback)
    return replacement
  elseif var.type == 'select' then
    vim.ui.select(var.items, { prompt = var.desc }, ui_callback)
    return replacement
  else
    -- if user variable 'type' value is invalid, stop the substitution process
    vim.notify(
      ('[launch.nvim] User variable "%s" type attribute must be "input" or "select"'):format(name),
      vim.log.levels.ERROR
    )
    substitution_stopped = true
  end
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
