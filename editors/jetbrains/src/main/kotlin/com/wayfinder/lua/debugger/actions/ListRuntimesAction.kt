package com.wayfinder.lua.debugger.actions

import com.intellij.notification.NotificationGroupManager
import com.intellij.notification.NotificationType
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.project.Project
import com.wayfinder.lua.debugger.WayfinderDebuggerComponent

/**
 * Action to list available Lua runtimes.
 */
class ListRuntimesAction : AnAction("List Lua Runtimes") {
  override fun actionPerformed(e: AnActionEvent) {
    val project = e.project ?: return
    val debugger = WayfinderDebuggerComponent.getInstance()

    val runtimes = debugger.getAvailableRuntimes()

    val message = buildString {
      appendLine("Available Lua Runtimes:")
      appendLine("========================================")
      if (runtimes.isEmpty()) {
        appendLine("  (none installed)")
      } else {
        for ((name, path) in runtimes) {
          val status = if (debugger.verifyRuntime(name)) "✓" else "✗"
          appendLine("  $status $name: $path")
        }
      }
      appendLine("=========================================")
    }

    NotificationGroupManager.getInstance()
      .getNotificationGroup("Wayfinder")
      .createNotification(message, NotificationType.INFORMATION)
      .notify(project)
  }

  override fun update(e: AnActionEvent) {
    e.presentation.isEnabled = e.project != null
  }
}
