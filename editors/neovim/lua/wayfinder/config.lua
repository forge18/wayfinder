-- Configuration management for Wayfinder Neovim plugin

local M = {}

-- Configuration instance
local config = {}

-- Load configuration from various sources
function M.setup(user_config)
  -- Start with user config
  config = user_config

  -- Load from environment variables (optional)
  if vim.env.WAYFINDER_PATH then
    config.wayfinder_path = vim.env.WAYFINDER_PATH
  end

  if vim.env.WAYFINDER_PORT then
    config.default_port = tonumber(vim.env.WAYFINDER_PORT) or config.default_port
  end

  -- Load from nvim config if available
  if vim.g.wayfinder_config then
    config = vim.tbl_deep_extend("force", config, vim.g.wayfinder_config)
  end
end

-- Get configuration value
function M.get(key, default)
  local keys = vim.split(key, ".")
  local value = config

  for _, k in ipairs(keys) do
    if type(value) ~= "table" then
      return default
    end
    value = value[k]
    if value == nil then
      return default
    end
  end

  return value
end

-- Set configuration value
function M.set(key, value)
  local keys = vim.split(key, ".")
  local cfg = config

  for i = 1, #keys - 1 do
    local k = keys[i]
    if not cfg[k] then
      cfg[k] = {}
    end
    cfg = cfg[k]
  end

  cfg[keys[#keys]] = value
end

-- Get all configuration
function M.get_all()
  return config
end

-- Detect runtime from file
function M.detect_runtime(filepath)
  if not filepath then
    filepath = vim.fn.expand("%")
  end

  local ext = vim.fn.fnamemodify(filepath, ":e")

  -- LuaNext files use LuaNext runtime
  if ext == "luax" then
    return "luanext"
  end

  -- Check for wayfinder.yaml in current directory
  local cwd = vim.fn.getcwd()
  local yaml_path = cwd .. "/wayfinder.yaml"

  if vim.fn.filereadable(yaml_path) == 1 then
    local yaml_content = vim.fn.readfile(yaml_path)
    for _, line in ipairs(yaml_content) do
      local runtime = line:match("runtime:%s*(%w+)")
      if runtime then
        return runtime
      end
    end
  end

  -- Default to lua54
  return "lua54"
end

-- Get runtime path
function M.get_runtime_path(runtime)
  return M.get("runtime_paths." .. runtime, runtime)
end

-- Verify runtime is available
function M.verify_runtime(runtime)
  local path = M.get_runtime_path(runtime)
  local result = vim.fn.system("which " .. path .. " 2>/dev/null")
  return vim.v.shell_error == 0
end

-- Verify all runtimes
function M.verify_all_runtimes()
  local runtimes = { "lua51", "lua52", "lua53", "lua54", "luanext" }
  local results = {}

  for _, runtime in ipairs(runtimes) do
    results[runtime] = M.verify_runtime(runtime)
  end

  return results
end

-- Get Wayfinder binary path
function M.get_wayfinder_path()
  local path = M.get("wayfinder_path", "wayfinder")

  -- Try various locations
  local candidates = {
    path,
    os.getenv("HOME") .. "/.cargo/bin/wayfinder",
    "/usr/local/bin/wayfinder",
    "/usr/bin/wayfinder",
  }

  for _, candidate in ipairs(candidates) do
    if vim.fn.executable(candidate) == 1 then
      return candidate
    end
  end

  -- Default to wayfinder in PATH
  return "wayfinder"
end

-- Substitute variables in string
function M.substitute_variables(str)
  -- ${cwd} or ${workspaceFolder}
  str = str:gsub("${cwd}", vim.fn.getcwd())
  str = str:gsub("${workspaceFolder}", vim.fn.getcwd())

  -- ${file}
  str = str:gsub("${file}", vim.fn.expand("%"))

  -- ${fileDir} or ${fileDirname}
  str = str:gsub("${fileDir}", vim.fn.expand("%:p:h"))
  str = str:gsub("${fileDirname}", vim.fn.expand("%:p:h"))

  -- ${fileBasename}
  str = str:gsub("${fileBasename}", vim.fn.expand("%:t"))

  -- ${home}
  str = str:gsub("${home}", os.getenv("HOME"))

  return str
end

return M
