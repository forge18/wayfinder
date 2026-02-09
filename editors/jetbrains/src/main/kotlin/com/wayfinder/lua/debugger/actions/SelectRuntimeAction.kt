package com.wayfinder.lua.debugger.actions

import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.ui.popup.JBPopupFactory
import com.intellij.openapi.ui.popup.PopupStep
import com.intellij.openapi.ui.popup.util.BaseListPopupStep
import com.intellij.ui.awt.RelativePoint
import com.wayfinder.lua.debugger.WayfinderDebuggerComponent
import com.wayfinder.lua.debugger.WayfinderProjectComponent
import java.awt.event.MouseEvent

/**
 * Action to select Lua runtime for debugging.
 */
class SelectRuntimeAction : AnAction("Select Lua Runtime") {
  override fun actionPerformed(e: AnActionEvent) {
    val project = e.project ?: return
    val debugger = WayfinderDebuggerComponent.getInstance()
    val runtimes = debugger.getAvailableRuntimes()

    if (runtimes.isEmpty()) {
      e.presentation.isEnabled = false
      return
    }

    val step = object : BaseListPopupStep<String>("Select Runtime", runtimes.keys.toList()) {
      override fun onChosen(selectedValue: String?, finalChoice: Boolean): PopupStep<*>? {
        if (selectedValue != null) {
          val projectComponent = WayfinderProjectComponent.getInstance(project)
          // Store selected runtime in project settings
        }
        return PopupStep.FINAL_CHOICE
      }

      override fun getTextFor(value: String): String {
        val path = runtimes[value] ?: value
        val status = if (debugger.verifyRuntime(value)) "✓" else "✗"
        return "$status $value ($path)"
      }
    }

    JBPopupFactory.getInstance()
      .createListPopup(step)
      .show(RelativePoint.getCenterOf(e.inputEvent as? MouseEvent))
  }

  override fun update(e: AnActionEvent) {
    e.presentation.isEnabled = e.project != null
  }
}
