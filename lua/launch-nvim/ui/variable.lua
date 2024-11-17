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
  vim.api.nvim_buf_set_lines(buf, -1, -1, true, { config.description })

  local parent_co = coroutine.running()
  vim.b[buf].on_user_action = function(text)
    vim.api.nvim_win_close(0, true)
    vim.api.nvim_buf_delete(buf, { force = true })

    -- empty string is an invalid input
    -- CHECK: this branch produces an extra "Press ENTER or type command to continue" message
    if text == '' then text = nil end
    coroutine.resume(parent_co, text)
  end
  vim.bo[buf].filetype = 'launch_nvim_var_ui_' .. config.type

  local win = vim.api.nvim_open_win(buf, true, {
    relative = 'editor',
    title = (' VARIABLE: %s '):format(config.name),
    title_pos = 'center',
    border = 'rounded',
    footer = ' launch.nvim ',
    footer_pos = 'right',
    style = 'minimal',
    width = 30,
    height = 4,
    col = 105,
    row = 30,
  })
  vim.wo[win].wrap = true
  vim.wo[win].signcolumn = 'yes:1'

  -- enter insert mode, and (optionally) insert default text
  utils.start_insert_mode()
  vim.api.nvim_feedkeys(config.default_text, 't', false)

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
