package com.wayfinder.lua.debugger

import com.intellij.openapi.components.Service
import com.intellij.openapi.diagnostic.logger
import com.intellij.openapi.project.Project
import com.wayfinder.lua.debugger.config.WayfinderConfiguration
import com.wayfinder.lua.debugger.config.RuntimeManager
import java.io.File

/**
 * Main component for Wayfinder debugger plugin.
 * Handles initialization and lifecycle management.
 */
@Service
class WayfinderDebuggerComponent {
  companion object {
    private val LOG = logger<WayfinderDebuggerComponent>()

    fun getInstance(): WayfinderDebuggerComponent = service()
  }

  private var configuration: WayfinderConfiguration = WayfinderConfiguration()
  private var runtimeManager: RuntimeManager = RuntimeManager()
  private var isInitialized = false

  fun initialize() {
    if (isInitialized) return

    try {
      LOG.info("Initializing Wayfinder debugger plugin")

      // Load configuration
      configuration.load()

      // Verify Wayfinder binary
      if (!verifyWayfinderBinary()) {
        LOG.warn("Wayfinder binary not found in PATH")
      }

      // Initialize runtime manager
      runtimeManager.refresh()

      isInitialized = true
      LOG.info("Wayfinder debugger initialized successfully")
    } catch (e: Exception) {
      LOG.error("Failed to initialize Wayfinder debugger", e)
    }
  }

  fun getConfiguration(): WayfinderConfiguration = configuration

  fun getRuntimeManager(): RuntimeManager = runtimeManager

  fun verifyWayfinderBinary(): Boolean {
    val wayfinderPath = configuration.wayfinderPath
    return try {
      val file = File(wayfinderPath)
      file.exists() && file.canExecute()
    } catch (e: Exception) {
      false
    }
  }

  fun detectRuntime(filePath: String): String {
    return when {
      filePath.endsWith(".luax") -> "luanext"
      filePath.endsWith(".lua") -> configuration.defaultRuntime
      else -> configuration.defaultRuntime
    }
  }

  fun verifyRuntime(runtime: String): Boolean {
    return runtimeManager.isAvailable(runtime)
  }

  fun getAvailableRuntimes(): Map<String, String> {
    return runtimeManager.getAvailableRuntimes()
  }
}
