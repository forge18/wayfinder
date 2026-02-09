package com.wayfinder.lua.debugger.run

import com.intellij.execution.ExecutionException
import com.intellij.execution.Executor
import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.execution.configurations.RunConfiguration
import com.intellij.execution.configurations.RunConfigurationBase
import com.intellij.execution.runners.ExecutionEnvironment
import com.intellij.openapi.options.SettingsEditor
import com.intellij.openapi.project.Project
import com.wayfinder.lua.debugger.WayfinderDebuggerComponent
import com.wayfinder.lua.debugger.WayfinderProjectComponent
import javax.swing.JComponent

/**
 * Run configuration for debugging Lua scripts.
 */
class WayfinderRunConfiguration(
  project: Project,
  factory: WayfinderConfigurationFactory,
  name: String
) : RunConfigurationBase<WayfinderRunConfiguration>(project, factory, name) {

  var scriptPath: String = ""
  var runtime: String = "lua54"
  var workingDirectory: String = project.basePath ?: ""
  var arguments: String = ""
  var sourceMapBehavior: String = "ask"

  override fun getConfigurationEditor(): SettingsEditor<WayfinderRunConfiguration> {
    return WayfinderRunConfigurationEditor(project)
  }

  override fun checkConfiguration() {
    if (scriptPath.isEmpty()) {
      throw ExecutionException("Script path not specified")
    }

    val component = WayfinderDebuggerComponent.getInstance()
    if (!component.verifyRuntime(runtime)) {
      throw ExecutionException("Runtime '$runtime' is not available")
    }
  }

  override fun getState(
    executor: Executor,
    environment: ExecutionEnvironment
  ): WayfinderProgramRunner? {
    return null
  }
}
