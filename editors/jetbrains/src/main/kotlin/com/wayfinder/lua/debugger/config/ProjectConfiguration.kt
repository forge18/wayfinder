package com.wayfinder.lua.debugger.config

import com.intellij.openapi.diagnostic.logger
import com.intellij.openapi.project.Project
import java.io.File

/**
 * Project-level configuration loading.
 * Reads from wayfinder.yaml or other config files in project root.
 */
class ProjectConfiguration(val project: Project) {
  companion object {
    private val LOG = logger<ProjectConfiguration>()
  }

  var runtime: String = "lua54"
  var port: Int = 5858
  var sourceMapBehavior: String = "ask"

  fun load() {
    val projectDir = File(project.basePath ?: return)

    // Try to load wayfinder.yaml
    val wayfinderYaml = File(projectDir, "wayfinder.yaml")
    if (wayfinderYaml.exists()) {
      LOG.info("Loading wayfinder.yaml from ${projectDir.absolutePath}")
      loadYaml(wayfinderYaml)
      return
    }

    // Try to load wayfinder.toml
    val wayfinderToml = File(projectDir, "wayfinder.toml")
    if (wayfinderToml.exists()) {
      LOG.info("Loading wayfinder.toml from ${projectDir.absolutePath}")
      loadToml(wayfinderToml)
      return
    }

    LOG.info("No wayfinder config file found in project root")
  }

  private fun loadYaml(file: File) {
    try {
      val content = file.readText()
      // Simple YAML parsing (for production, use a proper YAML library)
      content.lines().forEach { line ->
        when {
          line.startsWith("runtime:") -> runtime = line.substringAfter(":").trim()
          line.startsWith("port:") -> port = line.substringAfter(":").trim().toIntOrNull() ?: 5858
          line.startsWith("sourceMapBehavior:") -> sourceMapBehavior = line.substringAfter(":").trim()
        }
      }
    } catch (e: Exception) {
      LOG.error("Error loading wayfinder.yaml", e)
    }
  }

  private fun loadToml(file: File) {
    try {
      val content = file.readText()
      // Simple TOML parsing (for production, use a proper TOML library)
      content.lines().forEach { line ->
        when {
          line.startsWith("runtime") -> runtime = line.substringAfter("=").trim().trim('"')
          line.startsWith("port") -> port = line.substringAfter("=").trim().toIntOrNull() ?: 5858
          line.startsWith("sourceMapBehavior") -> sourceMapBehavior = line.substringAfter("=").trim().trim('"')
        }
      }
    } catch (e: Exception) {
      LOG.error("Error loading wayfinder.toml", e)
    }
  }
}
