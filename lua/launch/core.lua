---------------------------------------- CORE FUNCTIONALITY ----------------------------------------

local M = {}

---check if configuration is valid
---@param config RunConfig a run configuration object
---@return boolean
---@nodiscard
local function check_config(config)
  if not config then
    vim.cmd.redraw()
    vim.notify '[launch.nvim] No task selected'
    return false
  end

  return true
end

---display given configs to user and execute the selection with provided runner
---@param configs RunConfig[] list of configurations which the user can select from
---@param run fun(config: RunConfig) target runner to process selected config
function M.start(configs, run)
  if not configs or #configs == 0 then
    vim.notify('[launch.nvim] No tasks found', vim.log.levels.WARN)
    return
  end

  vim.ui.select(
    configs,
    {
      prompt = 'Tasks',
      ---@param config RunConfig
      format_item = function(config) return config.name end,
    },
    ---@param config RunConfig
    function(config)
      if check_config(config) then run(config) end
    end
  )
end

return M
