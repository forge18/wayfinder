package com.wayfinder.lua.debugger.execution

import com.intellij.xdebugger.XDebuggerSupport
import com.intellij.xdebugger.XSourcePosition
import com.intellij.xdebugger.breakpoints.XBreakpoint
import com.intellij.xdebugger.breakpoints.XBreakpointType
import com.intellij.xdebugger.breakpoints.XLineBreakpoint
import com.wayfinder.lua.debugger.breakpoints.LuaLineBreakpointType

/**
 * Debugger support for Lua language.
 */
class LuaDebuggerSupport : XDebuggerSupport() {
  override fun getBreakpointTypes(): Array<out XBreakpointType<*, *>> {
    return arrayOf(LuaLineBreakpointType())
  }
}
