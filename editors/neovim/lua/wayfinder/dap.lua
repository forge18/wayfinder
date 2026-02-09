-- DAP adapter configuration for Wayfinder

local M = {}

-- Reference to main module
local wayfinder
local config_module

function M.setup(plugin_config)
  wayfinder = require("wayfinder")
  config_module = require("wayfinder.config")

  -- Ensure nvim-dap is available
  local dap = require("dap")

  -- Register Wayfinder as a debug adapter
  dap.adapters.wayfinder = function(callback, config)
    -- Get configuration
    local program = config.program
    local runtime = config.runtime or config_module.detect_runtime(program)
    local port = config.port or wayfinder.get_next_port()
    local cwd = config.cwd or vim.fn.getcwd()
    local args = config.args or {}

    -- Register session
    wayfinder.register_session(
      config.name,
      {
        program = program,
        runtime = runtime,
        port = port,
        cwd = cwd,
      }
    )

    -- Get Wayfinder path
    local wayfinder_path = config_module.get_wayfinder_path()

    -- Build command
    local cmd = {
      wayfinder_path,
      "dap-server",
      "--port",
      tostring(port),
      "--runtime",
      runtime,
      "--script",
      program,
      "--cwd",
      cwd,
    }

    if #args > 0 then
      table.insert(cmd, "--args")
      for _, arg in ipairs(args) do
        table.insert(cmd, arg)
      end
    end

    -- Spawn the debug server
    local stdout = vim.loop.new_pipe(false)
    local stderr = vim.loop.new_pipe(false)

    local function on_exit(code, signal)
      stdout:close()
      stderr:close()
      wayfinder.unregister_session(config.name)

      if code ~= 0 then
        vim.notify(
          "Wayfinder exited with code: " .. code,
          vim.log.levels.ERROR
        )
      end
    end

    local handle
    handle = vim.loop.spawn(cmd[1], {
      args = vim.list_slice(cmd, 2),
      stdio = { nil, stdout, stderr },
      cwd = cwd,
    }, function(code, signal)
      on_exit(code, signal)
    end)

    -- Return the server configuration
    callback({
      type = "server",
      host = "localhost",
      port = port,
    })
  end

  -- Register Lua language debug configuration
  dap.configurations.lua = {
    {
      type = "wayfinder",
      request = "launch",
      name = "Launch Lua Script",
      program = "${file}",
      cwd = "${cwd}",
      stopOnEntry = false,
    },
    {
      type = "wayfinder",
      request = "launch",
      name = "Launch with Arguments",
      program = "${file}",
      cwd = "${cwd}",
      args = {},
      stopOnEntry = false,
    },
    {
      type = "wayfinder",
      request = "launch",
      name = "Launch LuaNext",
      program = "${file}",
      cwd = "${cwd}",
      runtime = "luanext",
      stopOnEntry = false,
    },
    {
      type = "wayfinder",
      request = "attach",
      name = "Attach to Process",
      port = 5858,
      host = "localhost",
    },
  }

  -- Register configurations for LuaNext if available
  if dap.configurations.luanext == nil then
    dap.configurations.luanext = dap.configurations.lua
  end
end

-- Start debugging current file
function M.debug_file(runtime, args)
  local dap = require("dap")
  local filepath = vim.fn.expand("%")

  -- Detect runtime if not provided
  if not runtime then
    runtime = config_module.detect_runtime(filepath)
  end

  -- Prepare configuration
  local config = {
    type = "wayfinder",
    request = "launch",
    name = "Debug: " .. vim.fn.fnamemodify(filepath, ":t"),
    program = filepath,
    cwd = vim.fn.getcwd(),
    runtime = runtime,
    args = args or {},
    stopOnEntry = false,
  }

  -- Substitute variables
  config.program = config_module.substitute_variables(config.program)
  config.cwd = config_module.substitute_variables(config.cwd)

  -- Start debugging
  dap.run(config)
end

-- Attach to running process
function M.attach(port, host)
  local dap = require("dap")

  port = port or 5858
  host = host or "localhost"

  local config = {
    type = "wayfinder",
    request = "attach",
    name = "Attach: " .. host .. ":" .. tostring(port),
    port = port,
    host = host,
  }

  dap.run(config)
end

return M
