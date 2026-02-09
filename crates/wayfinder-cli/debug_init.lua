-- Wayfinder Debug Initialization Script
-- This script is injected into the Lua process to enable debugging capabilities

-- Save original debug functions
local original_sethook = debug.sethook
local original_getinfo = debug.getinfo
local original_getlocal = debug.getlocal
local original_getupvalue = debug.getupvalue

-- Debug state
local wayfinder = {
    breakpoints = {},
    paused = false,
    step_mode = nil,  -- nil, "in", "over", "out"
    step_depth = 0,
    output_callback = nil,
}

-- Set output callback for DAP communication
function wayfinder.set_output_callback(callback)
    wayfinder.output_callback = callback
end

-- Send a message to the DAP server
local function send_message(msg_type, data)
    if wayfinder.output_callback then
        wayfinder.output_callback(msg_type, data)
    else
        -- Fallback: print to stderr
        io.stderr:write(string.format("[WAYFINDER:%s] %s\n", msg_type, tostring(data)))
    end
end

-- Add a breakpoint
function wayfinder.add_breakpoint(source, line)
    local key = source .. ":" .. line
    wayfinder.breakpoints[key] = true
    send_message("breakpoint_added", {source = source, line = line})
end

-- Remove a breakpoint
function wayfinder.remove_breakpoint(source, line)
    local key = source .. ":" .. line
    wayfinder.breakpoints[key] = nil
    send_message("breakpoint_removed", {source = source, line = line})
end

-- Check if we should break at current location
local function should_break(info)
    if not info.source or not info.currentline then
        return false
    end

    local key = info.source .. ":" .. info.currentline
    return wayfinder.breakpoints[key] == true
end

-- Get current stack trace
function wayfinder.get_stack_trace()
    local stack = {}
    local level = 2  -- Skip this function and the hook

    while true do
        local info = debug.getinfo(level, "nSlf")
        if not info then break end

        table.insert(stack, {
            name = info.name or "<anonymous>",
            source = info.source,
            line = info.currentline,
            what = info.what,
        })

        level = level + 1
    end

    return stack
end

-- Get local variables at a specific stack level
function wayfinder.get_locals(level)
    local locals = {}
    local i = 1

    while true do
        local name, value = debug.getlocal(level + 1, i)
        if not name then break end

        if name:sub(1, 1) ~= "(" then  -- Skip internal variables
            locals[name] = tostring(value)
        end

        i = i + 1
    end

    return locals
end

-- Debug hook function
local function debug_hook(event, line)
    local info = debug.getinfo(2, "nSlf")

    -- Check breakpoints
    if should_break(info) then
        wayfinder.paused = true
        send_message("paused", {
            reason = "breakpoint",
            source = info.source,
            line = line,
        })
    end

    -- Handle stepping
    if wayfinder.step_mode then
        if wayfinder.step_mode == "in" then
            wayfinder.paused = true
            send_message("paused", {
                reason = "step",
                source = info.source,
                line = line,
            })
        elseif wayfinder.step_mode == "over" then
            -- Step over: pause at next line at same or lower depth
            local current_depth = 0
            local level = 2
            while debug.getinfo(level) do
                current_depth = current_depth + 1
                level = level + 1
            end

            if current_depth <= wayfinder.step_depth then
                wayfinder.paused = true
                send_message("paused", {
                    reason = "step",
                    source = info.source,
                    line = line,
                })
            end
        elseif wayfinder.step_mode == "out" then
            -- Step out: pause when we return to caller
            local current_depth = 0
            local level = 2
            while debug.getinfo(level) do
                current_depth = current_depth + 1
                level = level + 1
            end

            if current_depth < wayfinder.step_depth then
                wayfinder.paused = true
                send_message("paused", {
                    reason = "step",
                    source = info.source,
                    line = line,
                })
            end
        end
    end

    -- Wait while paused
    while wayfinder.paused do
        -- In a real implementation, this would wait for DAP commands
        -- For now, we just break out
        break
    end
end

-- Enable debugging
function wayfinder.start()
    debug.sethook(debug_hook, "l")
    send_message("debug_started", {})
end

-- Disable debugging
function wayfinder.stop()
    debug.sethook()
    send_message("debug_stopped", {})
end

-- Continue execution
function wayfinder.continue()
    wayfinder.paused = false
    wayfinder.step_mode = nil
end

-- Step into
function wayfinder.step_in()
    wayfinder.step_mode = "in"
    wayfinder.paused = false
end

-- Step over
function wayfinder.step_over()
    wayfinder.step_mode = "over"
    wayfinder.step_depth = 0
    local level = 2
    while debug.getinfo(level) do
        wayfinder.step_depth = wayfinder.step_depth + 1
        level = level + 1
    end
    wayfinder.paused = false
end

-- Step out
function wayfinder.step_out()
    wayfinder.step_mode = "out"
    wayfinder.step_depth = 0
    local level = 2
    while debug.getinfo(level) do
        wayfinder.step_depth = wayfinder.step_depth + 1
        level = level + 1
    end
    wayfinder.paused = false
end

-- Expose wayfinder globally
_G.wayfinder = wayfinder

-- Print initialization message
io.stderr:write("[WAYFINDER] Debug helper initialized\n")

return wayfinder
