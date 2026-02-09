-- Simple test script for wayfinder launch command
print("Hello from Wayfinder!")
print("Testing Lua script execution...")

-- Simple calculation
local function factorial(n)
    if n == 0 then
        return 1
    else
        return n * factorial(n - 1)
    end
end

print("Factorial of 5 is:", factorial(5))
print("Script completed successfully!")
