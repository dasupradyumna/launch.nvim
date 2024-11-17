--------------------------------------- VARIABLE UI RENDERING --------------------------------------

local utils = require 'launch-nvim.utils'

local var_ui = {}

local renderer = {}

---render 'text' variable UI and get user input
---@param config LaunchNvimVariableConfigText variable config
---@return string? # user text input
---@nodiscard
function renderer.text(config)
  local buf = vim.api.nvim_create_buf(false, true)
  -- vim.bo[buf].filetype = 'launch_nvim_var_ui_' .. type
  vim.api.nvim_buf_set_lines(buf, -1, -1, true, { config.description })
  vim.bo[buf].buftype = 'prompt'
  vim.fn.prompt_setprompt(buf, ' > ')

  local win = vim.api.nvim_open_win(buf, true, {
    relative = 'editor',
    title = (' VARIABLE: %s '):format(config.name),
    title_pos = 'center',
    border = 'rounded',
    footer = ' launch.nvim ',
    footer_pos = 'right',
    style = 'minimal',
    width = 80,
    height = 10,
    col = 80,
    row = 25,
  })
  vim.wo[win].wrap = true
  vim.wo[win].signcolumn = 'yes:1'

  local parent_co = coroutine.running()
  vim.fn.prompt_setcallback(buf, function(text)
    vim.api.nvim_win_close(win, true)
    vim.api.nvim_buf_delete(buf, { force = true })
    coroutine.resume(parent_co, text)
  end)
  utils.start_insert_mode()

  return coroutine.yield()
end

---render 'list' variable UI and get user choice
---@param config LaunchNvimVariableConfigList variable config
---@return string? # user list choice
---@nodiscard
function renderer.list(config) end

---open a UI for the user to provide for variable substitution
---@param config LaunchNvimVariableConfig variable config
---@return string? # substitution string
---@nodiscard
function var_ui:open(config) return renderer[config.type](config) end

return var_ui
