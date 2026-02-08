--[[
    Watchpoint Runtime Injection Script
    
    This script provides advanced watchpoint functionality by injecting
    metamethods into tables and hooking into variable access patterns.
    
    Features:
    - Table field access monitoring via __index/__newindex
    - Function call monitoring
    - Variable assignment tracking
    - Efficient change detection with minimal performance impact
--]]

-- Global table to store watchpoint information
local _WAYFINDER_WATCHPOINTS = _WAYFINDER_WATCHPOINTS or {}
local _WAYFINDER_ORIGINAL_TABLES = _WAYFINDER_ORIGINAL_TABLES or {}

-- Utility function to generate unique IDs
local function generate_id()
    return tostring({})  -- Simple unique ID based on table address
end

-- Create a proxy table that intercepts field access
local function create_watchpoint_proxy(original_table, watchpoint_id)
    local proxy = {}
    local mt = {
        __index = function(t, k)
            -- Check if there's a watchpoint for this field
            local watchpoint = _WAYFINDER_WATCHPOINTS[watchpoint_id]
            if watchpoint and watchpoint.field == k and 
               (watchpoint.access_type == "read" or watchpoint.access_type == "readwrite") then
                -- Report read access
                if _WAYFINDER_REPORT_WATCHPOINT then
                    _WAYFINDER_REPORT_WATCHPOINT(watchpoint_id, "read", k, original_table[k])
                end
            end
            
            -- Return the original value
            return original_table[k]
        end,
        
        __newindex = function(t, k, v)
            -- Check if there's a watchpoint for this field
            local watchpoint = _WAYFINDER_WATCHPOINTS[watchpoint_id]
            if watchpoint and watchpoint.field == k and 
               (watchpoint.access_type == "write" or watchpoint.access_type == "readwrite") then
                -- Report write access
                if _WAYFINDER_REPORT_WATCHPOINT then
                    _WAYFINDER_REPORT_WATCHPOINT(watchpoint_id, "write", k, v, original_table[k])
                end
            end
            
            -- Set the value in the original table
            original_table[k] = v
        end,
        
        __pairs = function(t)
            return pairs(original_table)
        end,
        
        __ipairs = function(t)
            return ipairs(original_table)
        end,
        
        -- Forward other metamethods to the original table when possible
        __metatable = function(t)
            return getmetatable(original_table)
        end
    }
    
    setmetatable(proxy, mt)
    return proxy
end

-- Function to set up a table field watchpoint
function wayfinder_watch_table_field(table_ref, field_name, access_type)
    local watchpoint_id = generate_id()
    
    -- Store watchpoint information
    _WAYFINDER_WATCHPOINTS[watchpoint_id] = {
        type = "table_field",
        table_ref = table_ref,
        field = field_name,
        access_type = access_type or "write",  -- default to write-only
        created = os.time()
    }
    
    -- If we're watching a table field, replace the table with a proxy
    if type(table_ref) == "table" then
        local proxy = create_watchpoint_proxy(table_ref, watchpoint_id)
        _WAYFINDER_ORIGINAL_TABLES[proxy] = table_ref
        return proxy, watchpoint_id
    end
    
    return nil, watchpoint_id
end

-- Function to set up a global variable watchpoint
function wayfinder_watch_global(var_name, access_type)
    local watchpoint_id = generate_id()
    
    -- Store watchpoint information
    _WAYFINDER_WATCHPOINTS[watchpoint_id] = {
        type = "global",
        variable = var_name,
        access_type = access_type or "write",
        created = os.time()
    }
    
    -- Create a proxy for the global environment
    local original_env = _G
    local env_proxy = {}
    local env_mt = {
        __index = function(t, k)
            if k == var_name and 
               (access_type == "read" or access_type == "readwrite") then
                if _WAYFINDER_REPORT_WATCHPOINT then
                    _WAYFINDER_REPORT_WATCHPOINT(watchpoint_id, "read", k, original_env[k])
                end
            end
            return original_env[k]
        end,
        
        __newindex = function(t, k, v)
            if k == var_name and 
               (access_type == "write" or access_type == "readwrite") then
                local old_value = original_env[k]
                original_env[k] = v
                if _WAYFINDER_REPORT_WATCHPOINT then
                    _WAYFINDER_REPORT_WATCHPOINT(watchpoint_id, "write", k, v, old_value)
                end
                return
            end
            original_env[k] = v
        end
    }
    
    setmetatable(env_proxy, env_mt)
    
    -- Set the proxy as the new environment (this is a simplified approach)
    -- In practice, this would need to be more sophisticated
    return env_proxy, watchpoint_id
end

-- Function to remove a watchpoint
function wayfinder_remove_watchpoint(watchpoint_id)
    _WAYFINDER_WATCHPOINTS[watchpoint_id] = nil
end

-- Function to list all active watchpoints
function wayfinder_list_watchpoints()
    local result = {}
    for id, watchpoint in pairs(_WAYFINDER_WATCHPOINTS) do
        table.insert(result, {
            id = id,
            type = watchpoint.type,
            target = watchpoint.variable or watchpoint.field,
            access_type = watchpoint.access_type,
            created = watchpoint.created
        })
    end
    return result
end

-- Function to clear all watchpoints
function wayfinder_clear_watchpoints()
    _WAYFINDER_WATCHPOINTS = {}
    _WAYFINDER_ORIGINAL_TABLES = {}
end

-- Example usage:
-- local proxy, id = wayfinder_watch_table_field(my_table, "field_name", "readwrite")
-- _G.my_table = proxy  -- Replace original table with proxy

return {
    watch_table_field = wayfinder_watch_table_field,
    watch_global = wayfinder_watch_global,
    remove_watchpoint = wayfinder_remove_watchpoint,
    list_watchpoints = wayfinder_list_watchpoints,
    clear_watchpoints = wayfinder_clear_watchpoints
}