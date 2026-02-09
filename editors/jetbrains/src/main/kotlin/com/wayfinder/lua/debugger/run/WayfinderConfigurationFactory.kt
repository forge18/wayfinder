package com.wayfinder.lua.debugger.run

import com.intellij.execution.configurations.ConfigurationFactory
import com.intellij.execution.configurations.ConfigurationType
import com.intellij.execution.configurations.RunConfiguration
import com.intellij.openapi.project.Project

/**
 * Factory for creating Wayfinder run configurations.
 */
class WayfinderConfigurationFactory(type: ConfigurationType) : ConfigurationFactory(type) {
  override fun createTemplateConfiguration(project: Project): RunConfiguration {
    return WayfinderRunConfiguration(project, this, "Lua")
  }

  override fun getName(): String = "Lua"

  override fun getId(): String = "WayfinderLuaFactory"
}
