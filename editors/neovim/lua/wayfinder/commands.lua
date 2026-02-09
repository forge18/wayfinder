-- User commands for Wayfinder Neovim plugin

local M = {}

-- Reference to modules
local wayfinder
local config_module
local dap_module

function M.setup()
  wayfinder = require("wayfinder")
  config_module = require("wayfinder.config")
  dap_module = require("wayfinder.dap")

  -- Register user commands
  vim.api.nvim_create_user_command("WayfinderDebugFile", M.debug_file, {
    desc = "Debug the current Lua file",
    nargs = "*",
  })

  vim.api.nvim_create_user_command("WayfinderSelectRuntime", M.select_runtime, {
    desc = "Select Lua runtime version",
  })

  vim.api.nvim_create_user_command("WayfinderAttachProcess", M.attach_process, {
    desc = "Attach to a running Lua process",
    nargs = "*",
  })

  vim.api.nvim_create_user_command("WayfinderRuntimes", M.list_runtimes, {
    desc = "List available Lua runtimes",
  })

  -- Key mappings (optional)
  if vim.g.wayfinder_use_keymaps ~= false then
    M.setup_keymaps()
  end
end

-- Debug current file
function M.debug_file(opts)
  local args = opts.fargs or {}
  dap_module.debug_file(nil, args)
end

-- Select runtime
function M.select_runtime()
  -- Check if Telescope is available
  if wayfinder.has_telescope() then
    require("wayfinder.telescope").select_runtime()
  else
    M.select_runtime_fallback()
  end
end

-- Fallback runtime selection without Telescope
function M.select_runtime_fallback()
  local runtimes = { "lua51", "lua52", "lua53", "lua54", "luanext" }
  local choices = {}

  -- Build choices
  for i, runtime in ipairs(runtimes) do
    local path = config_module.get_runtime_path(runtime)
    local available = config_module.verify_runtime(runtime)
    local status = available and "✓" or "✗"
    choices[i] = string.format("%s %s (%s)", status, runtime, path)
  end

  -- Use vim.ui.select
  vim.ui.select(choices, {
    prompt = "Select Lua Runtime: ",
  }, function(choice, idx)
    if choice then
      wayfinder.set_runtime(runtimes[idx])
    end
  end)
end

-- Attach to process
function M.attach_process(opts)
  local args = opts.fargs or {}
  local port = tonumber(args[1]) or 5858
  local host = args[2] or "localhost"

  dap_module.attach(port, host)
end

-- List available runtimes
function M.list_runtimes()
  local runtimes = config_module.verify_all_runtimes()

  print("\nAvailable Lua Runtimes:")
  print("=" .. string.rep("=", 40))

  for runtime, available in pairs(runtimes) do
    local path = config_module.get_runtime_path(runtime)
    local status = available and "✓ available" or "✗ not found"
    print(string.format("  %s: %s (%s)", runtime, status, path))
  end

  print("=" .. string.rep("=", 41))
end

-- Setup key mappings
function M.setup_keymaps()
  local opts = { noremap = true, silent = true }

  -- Debug file with Ctrl+F5
  if vim.fn.exists(":WayfinderDebugFile") > 0 then
    vim.keymap.set("n", "<C-F5>", ":WayfinderDebugFile<CR>", opts)
  end

  -- Select runtime with Ctrl+Shift+R
  if vim.fn.exists(":WayfinderSelectRuntime") > 0 then
    vim.keymap.set("n", "<C-S-R>", ":WayfinderSelectRuntime<CR>", opts)
  end

  -- Attach with Ctrl+Shift+A
  if vim.fn.exists(":WayfinderAttachProcess") > 0 then
    vim.keymap.set("n", "<C-S-A>", ":WayfinderAttachProcess<CR>", opts)
  end
end

return M
