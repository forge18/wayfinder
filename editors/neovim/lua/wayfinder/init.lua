-- Wayfinder Debug Adapter for Neovim
-- Main entry point for the plugin

local M = {}

-- Plugin configuration
M.config = {
  wayfinder_path = "wayfinder",
  default_port = 5858,
  auto_detect_runtime = true,
  source_map_behavior = "ask", -- "ask", "lenient", "strict"
  runtime_paths = {
    lua51 = "lua5.1",
    lua52 = "lua5.2",
    lua53 = "lua5.3",
    lua54 = "lua5.4",
    luanext = "luanext",
  },
}

-- Internal state
M.state = {
  current_runtime = nil,
  active_sessions = {},
  next_port = nil,
}

-- Module imports
local config_module = require("wayfinder.config")
local dap_module = require("wayfinder.dap")
local commands_module = require("wayfinder.commands")
local telescope_module = require("wayfinder.telescope")

-- Initialize plugin
function M.setup(user_config)
  -- Merge user config with defaults
  if user_config then
    M.config = vim.tbl_deep_extend("force", M.config, user_config)
  end

  -- Initialize configuration
  config_module.setup(M.config)

  -- Initialize DAP
  dap_module.setup(M.config)

  -- Register commands
  commands_module.setup()

  -- Initialize Telescope (if available)
  pcall(function()
    telescope_module.setup()
  end)

  -- Set initial port
  M.state.next_port = M.config.default_port

  -- Print initialization message
  vim.notify("Wayfinder Debug Adapter initialized", vim.log.levels.INFO)
end

-- Get next available DAP port
function M.get_next_port()
  local port = M.state.next_port
  M.state.next_port = M.state.next_port + 1
  return port
end

-- Get current runtime
function M.get_runtime()
  return M.state.current_runtime
end

-- Set current runtime
function M.set_runtime(runtime)
  M.state.current_runtime = runtime
  vim.notify("Selected runtime: " .. runtime, vim.log.levels.INFO)
end

-- Register active session
function M.register_session(session_id, session_info)
  M.state.active_sessions[session_id] = session_info
end

-- Unregister session
function M.unregister_session(session_id)
  M.state.active_sessions[session_id] = nil
end

-- Get active sessions
function M.get_active_sessions()
  return M.state.active_sessions
end

-- Get configuration value
function M.get_config(key)
  return vim.tbl_get(M.config, vim.split(key, "."))
end

-- Set configuration value
function M.set_config(key, value)
  local keys = vim.split(key, ".")
  local config = M.config
  for i = 1, #keys - 1 do
    local k = keys[i]
    if not config[k] then
      config[k] = {}
    end
    config = config[k]
  end
  config[keys[#keys]] = value
end

-- Check if nvim-dap is available
function M.has_dap()
  local ok = pcall(require, "dap")
  return ok
end

-- Check if Telescope is available
function M.has_telescope()
  local ok = pcall(require, "telescope")
  return ok
end

-- Initialize on plugin load if called via vim.fn
if not vim.g.loaded_wayfinder then
  vim.g.loaded_wayfinder = 1

  -- Auto-setup with defaults if nvim-dap is available
  if M.has_dap() then
    M.setup({})
  else
    vim.notify(
      "Wayfinder: nvim-dap not found. Please install nvim-dap first.",
      vim.log.levels.WARN
    )
  end
end

return M
