-- DO NOT change the paths
local root = vim.fn.fnamemodify('./.repro', ':p')

-- set stdpaths to use .repro
for _, name in ipairs { 'config', 'data', 'state', 'cache' } do
  vim.env[('XDG_%s_HOME'):format(name:upper())] = vim.fs.joinpath(root, name)
end

-- bootstrap lazy
local lazypath = vim.fs.joinpath(root, 'plugins', 'lazy.nvim')
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system {
    'git',
    'clone',
    '--filter=blob:none',
    'https://github.com/folke/lazy.nvim',
    lazypath,
  }
end
vim.opt.runtimepath:prepend(lazypath)

-- install plugins
require('lazy').setup({
  { 'dasupradyumna/launch.nvim', opts = {} },
}, { root = root .. '/plugins' })

-- BUG: use treesitter builtin parsers (for lua, vimdoc filetypes)
vim.opt.runtimepath:append '/usr/lib/x86_64-linux-gnu/nvim'
