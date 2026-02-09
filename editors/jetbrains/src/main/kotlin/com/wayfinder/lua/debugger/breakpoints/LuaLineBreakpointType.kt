package com.wayfinder.lua.debugger.breakpoints

import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.xdebugger.breakpoints.XLineBreakpointType
import com.intellij.xdebugger.breakpoints.XBreakpointProperties

/**
 * Lua line breakpoint type.
 */
class LuaLineBreakpointType : XLineBreakpointType<XBreakpointProperties>("lua-line", "Lua Breakpoint") {
  override fun canPutAt(file: VirtualFile, line: Int, project: Project): Boolean {
    return file.name.endsWith(".lua") || file.name.endsWith(".luax")
  }

  override fun getDisplayText(breakpoint: com.intellij.xdebugger.breakpoints.XLineBreakpoint<XBreakpointProperties>): String {
    return "Line ${breakpoint.lineNumber + 1}"
  }
}
