#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>
#include <stdio.h>

int main() {
    lua_State *L = luaL_newstate();
    luaL_openlibs(L);
    
    // Test a simple Lua script
    if (luaL_dostring(L, "print('Hello from Lua!')") != LUA_OK) {
        fprintf(stderr, "Error: %s\n", lua_tostring(L, -1));
        lua_pop(L, 1);
    }
    
    lua_close(L);
    return 0;
}