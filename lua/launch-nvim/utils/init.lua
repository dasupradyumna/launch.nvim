----------------------------------------- UTILITY FUNCTIONS ----------------------------------------

local utils = {
  notify = require 'launch-nvim.utils.notify',
  validate = require 'launch-nvim.utils.validate',
}

---returns the current time as number of milliseconds from UNIX epoch
---@return number # current timestamp
function utils.curr_time_ms()
  local ts = vim.uv.clock_gettime 'realtime'
  return ts.sec * 10 ^ 3 + math.floor(ts.nsec / 10 ^ 6)
end

return utils
