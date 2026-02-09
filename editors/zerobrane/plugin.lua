-- Wayfinder Plugin for ZeroBrane Studio
-- Integrates Wayfinder debugger with ZeroBrane editor

local ide = ide
local wayfinder = require("wayfinder")

-- Configuration
local config = {
  name = "Wayfinder",
  description = "Debug Adapter Protocol for Lua and LuaNext",
  author = "Wayfinder Project",
  homepage = "https://github.com/forge18/wayfinder",
}

-- Initialize plugin
wayfinder.initialize()

-- ===== Menu Commands =====

-- Debug File command
ide:AddCommand("Wayfinder.DebugFile", "Wayfinder: Debug File", function()
  local editor = ide:GetEditor()
  if not editor then
    print("Error: No file open")
    return
  end

  local filePath = editor:GetFilePath()
  if not filePath then
    print("Error: File not saved")
    return
  end

  -- Start debug session
  local debugConfig, err = wayfinder.startDebug(filePath)
  if err then
    print("Error: " .. err)
    return
  end

  print(string.format("Starting debug session on port %d", debugConfig.port))
  -- Note: ZeroBrane integration with DAP would happen here
end, "Ctrl+F5")

-- Select Runtime command
ide:AddCommand("Wayfinder.SelectRuntime", "Wayfinder: Select Runtime", function()
  local choices = wayfinder.selectRuntime()
  if not choices then
    print("Error: No Lua runtimes found")
    return
  end

  -- Note: ZeroBrane would show a dialog here to select runtime
  for i, choice in ipairs(choices) do
    print(i .. ": " .. choice.text)
  end
end)

-- List Runtimes command
ide:AddCommand("Wayfinder.ListRuntimes", "Wayfinder: List Runtimes", function()
  local output = wayfinder.listRuntimes()
  print(output)
end)

-- Debug with Arguments command
ide:AddCommand("Wayfinder.DebugWithArgs", "Wayfinder: Debug with Arguments", function()
  local editor = ide:GetEditor()
  if not editor then
    print("Error: No file open")
    return
  end

  local filePath = editor:GetFilePath()
  if not filePath then
    print("Error: File not saved")
    return
  end

  -- Note: ZeroBrane would show a dialog for input here
  local args = {} -- Would be populated from user input
  local debugConfig, err = wayfinder.startDebug(filePath, args)
  if err then
    print("Error: " .. err)
    return
  end

  print(string.format("Starting debug session on port %d with arguments", debugConfig.port))
end)

-- ===== Project Commands =====

-- Load project configuration
ide:AddCommand("Wayfinder.LoadProjectConfig", "Wayfinder: Load Project Config", function()
  wayfinder.loadConfig()
  print("Project configuration loaded")
end)

-- ===== Menu Items =====

-- Add to Tools menu
local menu = ide:GetMenu("Tools")
if menu then
  menu:Append(wayfinder.menu.debug_file, "&Debug File")
  menu:Append(wayfinder.menu.select_runtime, "&Select Runtime")
  menu:Append(wayfinder.menu.list_runtimes, "&List Runtimes")
  menu:AppendSeparator()
  menu:Append(wayfinder.menu.debug_with_args, "Debug &with Arguments")
end

-- ===== Key Bindings =====

-- F5: Continue/Debug
ide:GetKeys():AddBinding("Wayfinder.DebugFile", "Ctrl+F5")

-- ===== Editor Integration =====

-- Right-click context menu
ide:AddEditorContextMenu(function(editor, menu)
  local filePath = editor:GetFilePath()
  if filePath and (filePath:match("%.lua$") or filePath:match("%.luax$")) then
    menu:Append("Wayfinder.DebugFile", "Debug '" .. filePath:match("[^/\\]*$") .. "'")
  end
end)

-- ===== Status Bar Updates =====

-- Show active debug sessions
local function updateStatusBar()
  local sessions = wayfinder.getActiveSessions()
  local count = 0
  for _ in pairs(sessions) do
    count = count + 1
  end

  if count > 0 then
    ide:UpdateStatusBar(string.format("Wayfinder: %d active session(s)", count))
  end
end

-- ===== Output Panel =====

-- Setup output panel for debug messages
local output = GetOutput and GetOutput("Wayfinder") or print

-- ===== Hooks =====

-- On file open: Show runtime info
ide:AddHook("OnFileOpen", function(file)
  if file:match("%.lua$") or file:match("%.luax$") then
    wayfinder.loadConfig()
    local runtime = wayfinder.detectRuntime(file)
    output("Detected runtime: " .. runtime)
  end
end)

-- On project open: Load project config
ide:AddHook("OnProjectOpen", function(project)
  wayfinder.loadConfig()
end)

-- ===== Configuration Panel =====

-- Settings for Wayfinder
local settingsPanel = {
  {
    name = "Wayfinder Path",
    type = "string",
    default = "wayfinder",
    description = "Path to Wayfinder binary"
  },
  {
    name = "Default Port",
    type = "number",
    default = 5858,
    description = "DAP server port (auto-increments)"
  },
  {
    name = "Default Runtime",
    type = "choice",
    default = "lua54",
    choices = { "lua51", "lua52", "lua53", "lua54", "luanext" },
    description = "Default Lua version for debugging"
  },
  {
    name = "Auto-Detect Runtime",
    type = "boolean",
    default = true,
    description = "Auto-detect runtime from file extension"
  },
  {
    name = "Source Map Behavior",
    type = "choice",
    default = "ask",
    choices = { "ask", "lenient", "strict" },
    description = "How to handle source maps for bundled files"
  }
}

-- ===== Error Handling =====

local function handleError(err)
  output("Wayfinder Error: " .. tostring(err))
end

setmetatable(_G, {
  __index = function(t, k)
    if k == "ide" then
      return ide
    end
  end
})

-- ===== Export Configuration =====

return {
  name = config.name,
  description = config.description,
  author = config.author,
  version = wayfinder.version or "1.0.0",
  homepage = config.homepage,
  settingsPanel = settingsPanel,
}
