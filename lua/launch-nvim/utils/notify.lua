--------------------------------------- NOTIFICATION MANAGER ---------------------------------------

---@class LaunchNvimNotifyModule
---@field private level table<LaunchNvimNotifyLevel, integer> enum mapping to neovim log levels
local M = {}

---@enum (key) LaunchNvimNotifyLevel
M.level = {
  I = vim.log.levels.INFO,
  W = vim.log.levels.WARN,
  E = vim.log.levels.ERROR,
}

---display a notification message of the specified level
---
---> the message can either be a string or a list of strings which will be joined by newline
---@param level LaunchNvimNotifyLevel notification level
---@param message string|string[] notification message(s)
---@private
function M:send(level, message)
  if type(message) == 'string' then message = { message } end
  local msg = table.concat(message, '\n')

  -- builtin : sends to messages
  vim.notify('[launch.nvim] ' .. msg, self.level[level])
end

---display an information message
---
---> the message can either be a string or a list of strings which will be joined by newline
---@overload fun(self, msg: string)
---@overload fun(self, msg_list: string[])
function M:info(...) self:send('I', ...) end

---display a warning message
---
---> the message can either be a string or a list of strings which will be joined by newline
---@overload fun(self, msg: string)
---@overload fun(self, msg_list: string[])
function M:warn(...) self:send('W', ...) end

---display an error message
---
---> the message can either be a string or a list of strings which will be joined by newline
---@overload fun(self, msg: string)
---@overload fun(self, msg_list: string[])
function M:error(...) self:send('E', ...) end

---display an error message and raise an error that propagates up the call stack
---
---> the message can either be a string or a list of strings which will be joined by newline
---@overload fun(self, msg: string)
---@overload fun(self, msg_list: string[])
function M:throw(...)
  self:error(...)
  error()
end

return M
