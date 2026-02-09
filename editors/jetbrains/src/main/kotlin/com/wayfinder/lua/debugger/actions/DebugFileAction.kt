package com.wayfinder.lua.debugger.actions

import com.intellij.execution.ExecutionManager
import com.intellij.execution.executors.DefaultDebugExecutor
import com.intellij.openapi.actionSystem.AnAction
import com.intellij.openapi.actionSystem.AnActionEvent
import com.intellij.openapi.actionSystem.CommonDataKeys
import com.intellij.openapi.fileEditor.FileEditorManager
import com.intellij.openapi.project.Project
import com.intellij.psi.PsiManager
import com.wayfinder.lua.debugger.WayfinderDebuggerComponent
import com.wayfinder.lua.debugger.WayfinderProjectComponent
import com.wayfinder.lua.debugger.run.WayfinderRunConfiguration
import com.wayfinder.lua.debugger.run.WayfinderConfigurationFactory
import com.wayfinder.lua.debugger.run.WayfinderConfigurationType

/**
 * Action to debug the current Lua file.
 */
class DebugFileAction : AnAction("Debug Lua File") {
  override fun actionPerformed(e: AnActionEvent) {
    val project = e.project ?: return
    val editor = e.getData(CommonDataKeys.EDITOR) ?: return
    val file = e.getData(CommonDataKeys.VIRTUAL_FILE) ?: return

    if (!isLuaFile(file.name)) {
      return
    }

    val debugger = WayfinderDebuggerComponent.getInstance()
    debugger.initialize()

    // Detect runtime from file
    val runtime = debugger.detectRuntime(file.path)

    if (!debugger.verifyRuntime(runtime)) {
      return
    }

    // Create and run configuration
    val configurationType = WayfinderConfigurationType()
    val factory = configurationType.configurationFactories.first() as WayfinderConfigurationFactory
    val runConfig = WayfinderRunConfiguration(project, factory, file.nameWithoutExtension)

    runConfig.scriptPath = file.path
    runConfig.runtime = runtime

    val executor = DefaultDebugExecutor()
    ExecutionManager.getInstance(project).startRunProfile(runConfig, executor, project) {
      // Debug session started
    }
  }

  override fun update(e: AnActionEvent) {
    val file = e.getData(CommonDataKeys.VIRTUAL_FILE)
    e.presentation.isEnabled = file != null && isLuaFile(file.name)
  }

  private fun isLuaFile(fileName: String): Boolean {
    return fileName.endsWith(".lua") || fileName.endsWith(".luax")
  }
}
