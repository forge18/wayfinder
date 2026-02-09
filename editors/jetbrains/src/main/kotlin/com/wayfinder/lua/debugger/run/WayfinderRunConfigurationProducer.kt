package com.wayfinder.lua.debugger.run

import com.intellij.execution.actions.ConfigurationContext
import com.intellij.execution.actions.RunConfigurationProducer
import com.intellij.execution.configurations.ConfigurationFromContext
import com.intellij.openapi.util.Ref
import com.intellij.psi.PsiElement
import com.intellij.psi.PsiFile

/**
 * Producer for automatically creating run configurations from context.
 */
class WayfinderRunConfigurationProducer : RunConfigurationProducer<WayfinderRunConfiguration>(
  WayfinderConfigurationType()
) {

  override fun setupConfigurationFromContext(
    configuration: WayfinderRunConfiguration,
    context: ConfigurationContext,
    sourceElement: Ref<PsiElement>
  ): Boolean {
    val file = context.psiLocation?.containingFile ?: return false

    if (!isLuaFile(file)) return false

    configuration.scriptPath = file.virtualFile?.path ?: return false
    return true
  }

  override fun isConfigurationFromContext(
    configuration: WayfinderRunConfiguration,
    context: ConfigurationContext
  ): Boolean {
    val file = context.psiLocation?.containingFile ?: return false
    return configuration.scriptPath == file.virtualFile?.path
  }

  private fun isLuaFile(file: PsiFile): Boolean {
    val name = file.name
    return name.endsWith(".lua") || name.endsWith(".luax")
  }
}
