-- Telescope integration for Wayfinder

local M = {}

-- Reference to modules
local wayfinder
local config_module

function M.setup()
  wayfinder = require("wayfinder")
  config_module = require("wayfinder.config")
end

-- Select runtime using Telescope
function M.select_runtime()
  local telescope = require("telescope.pickers")
  local finders = require("telescope.finders")
  local actions = require("telescope.actions")
  local action_state = require("telescope.actions.state")
  local themes = require("telescope.themes")

  local runtimes = { "lua51", "lua52", "lua53", "lua54", "luanext" }

  -- Create list of runtimes with status
  local runtime_list = {}
  for _, runtime in ipairs(runtimes) do
    local path = config_module.get_runtime_path(runtime)
    local available = config_module.verify_runtime(runtime)
    local status = available and "✓" or "✗"
    table.insert(runtime_list, {
      runtime = runtime,
      display = string.format("%s %s (%s)", status, runtime, path),
      path = path,
      available = available,
    })
  end

  local picker = telescope.new(
    finders.new_table({
      results = runtime_list,
      entry_maker = function(entry)
        return {
          value = entry.runtime,
          display = entry.display,
          ordinal = entry.runtime,
        }
      end,
    }),
    themes.get_dropdown({
      prompt_title = "Select Lua Runtime",
      previewer = false,
      layout_config = {
        width = 0.5,
        height = 0.5,
      },
    }),
    {
      attach_mappings = function(prompt_bufnr, map)
        actions.select_default:replace(function()
          actions.close(prompt_bufnr)
          local selection = action_state.get_selected_entry()
          if selection then
            wayfinder.set_runtime(selection.value)
          end
        end)
        return true
      end,
    }
  )

  picker:find()
end

-- Open file picker for debugging
function M.debug_file_picker()
  local telescope = require("telescope.pickers")
  local finders = require("telescope.finders")
  local actions = require("telescope.actions")
  local action_state = require("telescope.actions.state")
  local themes = require("telescope.themes")
  local dap_module = require("wayfinder.dap")

  local picker = telescope.new(
    finders.new_oneshot_job({
      "find",
      vim.fn.getcwd(),
      "-type",
      "f",
      "-name",
      "*.lua",
      "-o",
      "-name",
      "*.luax",
    }),
    themes.get_dropdown({
      prompt_title = "Select Lua File to Debug",
      previewer = false,
      layout_config = {
        width = 0.7,
      },
    }),
    {
      attach_mappings = function(prompt_bufnr, map)
        actions.select_default:replace(function()
          actions.close(prompt_bufnr)
          local selection = action_state.get_selected_entry()
          if selection then
            dap_module.debug_file(nil, {})
          end
        end)
        return true
      end,
    }
  )

  picker:find()
end

-- Open runtime configuration picker
function M.runtime_config()
  local telescope = require("telescope.pickers")
  local finders = require("telescope.finders")
  local actions = require("telescope.actions")
  local action_state = require("telescope.actions.state")
  local themes = require("telescope.themes")

  local configs = {
    {
      name = "Lua 5.1",
      key = "runtime_paths.lua51",
    },
    {
      name = "Lua 5.2",
      key = "runtime_paths.lua52",
    },
    {
      name = "Lua 5.3",
      key = "runtime_paths.lua53",
    },
    {
      name = "Lua 5.4",
      key = "runtime_paths.lua54",
    },
    {
      name = "LuaNext",
      key = "runtime_paths.luanext",
    },
    {
      name = "Wayfinder Binary",
      key = "wayfinder_path",
    },
    {
      name = "DAP Port",
      key = "default_port",
    },
  }

  local picker = telescope.new(
    finders.new_table({
      results = configs,
      entry_maker = function(entry)
        local current = config_module.get(entry.key, "")
        return {
          value = entry.key,
          display = string.format("%s: %s", entry.name, current),
          ordinal = entry.name,
        }
      end,
    }),
    themes.get_dropdown({
      prompt_title = "Configure Runtime Settings",
      previewer = false,
    }),
    {
      attach_mappings = function(prompt_bufnr, map)
        actions.select_default:replace(function()
          actions.close(prompt_bufnr)
          local selection = action_state.get_selected_entry()
          if selection then
            M.edit_config_value(selection.value)
          end
        end)
        return true
      end,
    }
  )

  picker:find()
end

-- Edit a configuration value
function M.edit_config_value(key)
  local current = config_module.get(key, "")

  vim.ui.input({
    prompt = "Enter new value for " .. key .. ": ",
    default = tostring(current),
  }, function(value)
    if value then
      if key == "default_port" then
        value = tonumber(value) or current
      end
      config_module.set(key, value)
      vim.notify("Updated " .. key .. " = " .. tostring(value), vim.log.levels.INFO)
    end
  end)
end

return M
