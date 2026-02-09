-- Wayfinder Debugger for ZeroBrane Studio
-- Lua-based debug adapter for Wayfinder DAP server

local wayfinder = {
  name = "Wayfinder",
  description = "Debug Adapter Protocol for Lua and LuaNext",
  author = "Wayfinder Project",
  version = "1.0.0",
}

-- Configuration
local config = {
  wayfinderPath = "wayfinder",
  defaultPort = 5858,
  defaultRuntime = "lua54",
  autoDetectRuntime = true,
  sourceMapBehavior = "ask",
  runtimePaths = {
    lua51 = "lua5.1",
    lua52 = "lua5.2",
    lua53 = "lua5.3",
    lua54 = "lua5.4",
    luanext = "luanext",
  }
}

-- Session management
local sessions = {}
local nextPort = config.defaultPort

-- ===== Configuration Management =====

function wayfinder.loadConfig()
  -- Check environment variables
  if os.getenv("WAYFINDER_PATH") then
    config.wayfinderPath = os.getenv("WAYFINDER_PATH")
  end

  if os.getenv("WAYFINDER_PORT") then
    local port = tonumber(os.getenv("WAYFINDER_PORT"))
    if port then
      nextPort = port
      config.defaultPort = port
    end
  end

  -- Load from wayfinder.yaml if exists
  local yamlFile = io.open("wayfinder.yaml", "r")
  if yamlFile then
    local content = yamlFile:read("*a")
    yamlFile:close()

    for line in content:gmatch("[^\n]+") do
      local key, value = line:match("^%s*([^:]+):%s*(.*)$")
      if key then
        key = key:match("^%s*(.-)%s*$")
        value = value:match("^%s*(.-)%s*$")
        if key == "runtime" then
          config.defaultRuntime = value
        elseif key == "port" then
          config.defaultPort = tonumber(value) or config.defaultPort
        elseif key == "sourceMapBehavior" then
          config.sourceMapBehavior = value
        end
      end
    end
  end
end

-- ===== Runtime Detection =====

function wayfinder.detectRuntime(filePath)
  if filePath:match("%.luax$") then
    return "luanext"
  end

  -- Check project config
  if config.defaultRuntime ~= "lua54" then
    return config.defaultRuntime
  end

  -- Default
  return "lua54"
end

function wayfinder.getRuntimePath(runtime)
  return config.runtimePaths[runtime] or runtime
end

function wayfinder.verifyRuntime(runtime)
  local path = wayfinder.getRuntimePath(runtime)
  local handle = io.popen("which " .. path .. " 2>/dev/null")
  if handle then
    local result = handle:read("*a")
    handle:close()
    return result ~= ""
  end
  return false
end

function wayfinder.getAvailableRuntimes()
  local available = {}
  for name, _ in pairs(config.runtimePaths) do
    if wayfinder.verifyRuntime(name) then
      available[name] = wayfinder.getRuntimePath(name)
    end
  end
  return available
end

-- ===== Session Management =====

function wayfinder.getNextPort()
  local port = nextPort
  nextPort = nextPort + 1
  return port
end

function wayfinder.registerSession(filePath, runtime, port)
  local sessionId = "wayfinder-" .. os.time() .. "-" .. math.random(1000)
  sessions[sessionId] = {
    filePath = filePath,
    runtime = runtime,
    port = port,
    timestamp = os.time()
  }
  return sessionId
end

function wayfinder.unregisterSession(sessionId)
  sessions[sessionId] = nil
end

function wayfinder.getSession(sessionId)
  return sessions[sessionId]
end

function wayfinder.getActiveSessions()
  return sessions
end

-- ===== Debugger Integration =====

function wayfinder.startDebug(filePath, args)
  -- Initialize config
  wayfinder.loadConfig()

  -- Detect runtime
  local runtime = wayfinder.detectRuntime(filePath)

  -- Verify runtime available
  if not wayfinder.verifyRuntime(runtime) then
    return nil, "Runtime '" .. runtime .. "' not found"
  end

  -- Get next port
  local port = wayfinder.getNextPort()

  -- Build command
  local cmd = {
    config.wayfinderPath,
    "dap-server",
    "--port", tostring(port),
    "--runtime", runtime,
    "--script", filePath
  }

  -- Add arguments if provided
  if args then
    for _, arg in ipairs(args) do
      table.insert(cmd, arg)
    end
  end

  -- Register session
  local sessionId = wayfinder.registerSession(filePath, runtime, port)

  -- Return debug configuration
  return {
    host = "localhost",
    port = port,
    sessionId = sessionId,
    command = table.concat(cmd, " ")
  }, nil
end

function wayfinder.stopDebug(sessionId)
  wayfinder.unregisterSession(sessionId)
end

-- ===== UI Commands =====

function wayfinder.selectRuntime()
  wayfinder.loadConfig()
  local available = wayfinder.getAvailableRuntimes()

  if not available or next(available) == nil then
    return nil, "No Lua runtimes found"
  end

  -- Build choice table for ZeroBrane
  local choices = {}
  for name, path in pairs(available) do
    table.insert(choices, {
      text = name .. " (" .. path .. ")",
      value = name
    })
  end

  return choices
end

function wayfinder.listRuntimes()
  wayfinder.loadConfig()
  local available = wayfinder.getAvailableRuntimes()

  local output = "Available Lua Runtimes:\n"
  output = output .. "========================================\n"

  if not available or next(available) == nil then
    output = output .. "  (none installed)\n"
  else
    for name, path in pairs(available) do
      local status = wayfinder.verifyRuntime(name) and "✓" or "✗"
      output = output .. "  " .. status .. " " .. name .. ": " .. path .. "\n"
    end
  end

  output = output .. "=========================================\n"
  return output
end

-- ===== File Operations =====

function wayfinder.getFileContent(filePath)
  local file = io.open(filePath, "r")
  if not file then
    return nil
  end
  local content = file:read("*a")
  file:close()
  return content
end

function wayfinder.getLineCount(filePath)
  local file = io.open(filePath, "r")
  if not file then
    return 0
  end

  local count = 0
  for _ in file:lines() do
    count = count + 1
  end
  file:close()
  return count
end

-- ===== Expression Evaluation =====

function wayfinder.evaluateExpression(expr, frameVars)
  local env = frameVars or {}
  setmetatable(env, { __index = _G })

  local func, err = load(expr, "expression", "t", env)
  if not func then
    return nil, err
  end

  local success, result = pcall(func)
  if not success then
    return nil, result
  end

  return result
end

-- ===== Initialization =====

function wayfinder.initialize()
  wayfinder.loadConfig()

  -- Verify Wayfinder binary exists
  local handle = io.popen("which " .. config.wayfinderPath .. " 2>/dev/null")
  if handle then
    local result = handle:read("*a")
    handle:close()
    if result == "" then
      print("Warning: Wayfinder binary not found in PATH")
      return false
    end
  end

  return true
end

-- ===== Export API =====

return wayfinder
