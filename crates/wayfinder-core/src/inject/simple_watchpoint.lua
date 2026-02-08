--[[
    Simple Watchpoint Runtime Injection Script
    
    This is a lightweight version of watchpoint injection that can be
    easily loaded into any Lua environment to provide basic variable
    monitoring capabilities.
--]]

-- Simple watchpoint system
local watchpoints = {}
local watchpoint_counter = 0

-- Function to add a watchpoint
function add_watchpoint(variable_name, callback)
    watchpoint_counter = watchpoint_counter + 1
    local id = watchpoint_counter
    
    watchpoints[id] = {
        name = variable_name,
        callback = callback,
        last_value = nil
    }
    
    return id
end

-- Function to remove a watchpoint
function remove_watchpoint(id)
    watchpoints[id] = nil
end

-- Function to check all watchpoints (should be called periodically)
function check_watchpoints()
    for id, wp in pairs(watchpoints) do
        -- Get current value using rawget to avoid metamethods
        local current_value = rawget(_G, wp.name)
        
        -- Check if value has changed
        if wp.last_value ~= current_value then
            -- Value changed, call callback
            if wp.callback then
                wp.callback(id, wp.name, wp.last_value, current_value)
            end
            
            -- Update last value
            wp.last_value = current_value
        end
    end
end

-- Function to check a specific variable
function check_variable(variable_name)
    for id, wp in pairs(watchpoints) do
        if wp.name == variable_name then
            local current_value = rawget(_G, wp.name)
            if wp.last_value ~= current_value then
                if wp.callback then
                    wp.callback(id, wp.name, wp.last_value, current_value)
                end
                wp.last_value = current_value
                return true  -- Changed
            end
            return false  -- Not changed
        end
    end
    return nil  -- Not watched
end

-- Export functions
_wayfinder_watch = {
    add = add_watchpoint,
    remove = remove_watchpoint,
    check_all = check_watchpoints,
    check_variable = check_variable
}

-- Example usage:
-- local id = _wayfinder_watch.add("myvar", function(id, name, old_val, new_val)
--     print("Watchpoint triggered:", name, old_val, "->", new_val)
-- end)
--
-- -- Periodically call this to check for changes
-- _wayfinder_watch.check_all()

return _wayfinder_watch