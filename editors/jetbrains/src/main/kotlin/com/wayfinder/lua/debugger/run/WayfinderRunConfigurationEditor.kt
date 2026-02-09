package com.wayfinder.lua.debugger.run

import com.intellij.openapi.fileChooser.FileChooserDescriptor
import com.intellij.openapi.options.SettingsEditor
import com.intellij.openapi.project.Project
import com.intellij.openapi.ui.LabeledComponent
import com.intellij.openapi.ui.TextFieldWithBrowseButton
import com.intellij.ui.RawCommandLineEditor
import javax.swing.JComboBox
import javax.swing.JComponent
import javax.swing.JPanel
import java.awt.BorderLayout

/**
 * Settings editor for Lua run configurations.
 */
class WayfinderRunConfigurationEditor(val project: Project) :
  SettingsEditor<WayfinderRunConfiguration>() {

  private val scriptPathField = TextFieldWithBrowseButton()
  private val runtimeCombo = JComboBox<String>(arrayOf("lua51", "lua52", "lua53", "lua54", "luanext"))
  private val workingDirField = TextFieldWithBrowseButton()
  private val argumentsField = RawCommandLineEditor()

  init {
    val descriptor = FileChooserDescriptor(true, false, false, false, false, false)
      .withFileFilter { it.name.endsWith(".lua") || it.name.endsWith(".luax") }
    scriptPathField.addBrowseFolderListener(
      "Select Lua Script",
      "Choose a Lua script to debug",
      project,
      descriptor
    )

    workingDirField.addBrowseFolderListener(
      "Select Working Directory",
      "Choose the working directory for script execution",
      project,
      FileChooserDescriptor(false, true, false, false, false, false)
    )
  }

  override fun resetEditorFrom(s: WayfinderRunConfiguration) {
    scriptPathField.text = s.scriptPath
    runtimeCombo.selectedItem = s.runtime
    workingDirField.text = s.workingDirectory
    argumentsField.text = s.arguments
  }

  override fun applyEditorTo(s: WayfinderRunConfiguration) {
    s.scriptPath = scriptPathField.text
    s.runtime = runtimeCombo.selectedItem as String
    s.workingDirectory = workingDirField.text
    s.arguments = argumentsField.text
  }

  override fun createEditor(): JComponent {
    val panel = JPanel(BorderLayout())
    panel.add(scriptPathField, BorderLayout.NORTH)
    panel.add(runtimeCombo, BorderLayout.CENTER)
    return panel
  }
}
