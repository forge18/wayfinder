package com.wayfinder.lua.debugger.run

import com.intellij.execution.configurations.ConfigurationFactory
import com.intellij.execution.configurations.ConfigurationType
import com.intellij.icons.AllIcons
import com.intellij.openapi.util.IconLoader
import javax.swing.Icon

/**
 * Configuration type for Lua debug configurations.
 */
class WayfinderConfigurationType : ConfigurationType {
  override fun getDisplayName(): String = "Lua (Wayfinder)"

  override fun getConfigurationTypeDescription(): String =
    "Debug Lua scripts using Wayfinder DAP"

  override fun getIcon(): Icon = IconLoader.getIcon("/icons/lua.svg", this::class.java)

  override fun getConfigurationFactories(): Array<ConfigurationFactory> =
    arrayOf(WayfinderConfigurationFactory(this))

  override fun getId(): String = "WayfinderLua"
}
