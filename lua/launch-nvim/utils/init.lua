----------------------------------------- UTILITY FUNCTIONS ----------------------------------------

local M = {
  notify = require 'launch-nvim.utils.notify',
  validate = require 'launch-nvim.utils.validate',
}

---returns the current time as number of milliseconds from UNIX epoch
---@return number # current timestamp
function M.curr_time_ms()
  local ts = vim.uv.clock_gettime 'realtime'
  return ts.sec * 10 ^ 3 + math.floor(ts.nsec / 10 ^ 6)
end

---enters insert mode immediately when called
---
---> this funciton is needed because 'startinsert' does not work well with scripts
function M.start_insert_mode() vim.api.nvim_feedkeys('i', 'n', false) end

return M
