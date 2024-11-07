-------------------------------------- RUNTIME CONFIGURATIONS --------------------------------------

local utils = require 'launch-nvim.utils'

local configs = {}

---@type string launch.nvim data directory where all runtime configs are saved
---@diagnostic disable-next-line:param-type-mismatch
configs.data_dir = vim.fs.joinpath(vim.fn.stdpath 'data', 'launch_nvim')

---@type string runtime configs file path for the current working directory
configs.runtime_file_path = nil

-- TODO: add meta file with type definitions
---@type table
configs.list = {}

---loads runtime configs for CWD from JSON file
function configs:load()
  self.runtime_file_path = ('%s/%s.json'):format(
    self.data_dir,
    vim.uv.cwd():gsub('@', '@@'):gsub('[\\/:]', '@')
  )

  local runtime_file, error = io.open(self.runtime_file_path, 'r')
  if not runtime_file then
    -- clear runtime configs list and return if file does not exist for CWD
    -- TODO: make this internal logging instead of notification
    -- utils.notify:warn { 'Could not load configs from runtime file.', ('\tWARN: %s'):format(error) }
    self.list = {}
    return
  end

  self.list = vim.json.decode(runtime_file:read '*all')
  runtime_file:close()
end

---saves runtime configs for CWD to JSON file
function configs:save()
  local runtime_file, error = io.open(self.runtime_file_path, 'w+')
  if not runtime_file then
    utils.notify:error { 'Could not save configs to runtime file.', ('\tERROR: %s'):format(error) }
    return
  end

  runtime_file:write(vim.json.encode(self.list))
  runtime_file:close()
end

return configs
