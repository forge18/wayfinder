package com.wayfinder.lua.debugger.config

import com.intellij.openapi.diagnostic.logger
import java.io.File

/**
 * Application-level configuration for Wayfinder debugger.
 * Loads from multiple sources with precedence order.
 */
class WayfinderConfiguration {
  companion object {
    private val LOG = logger<WayfinderConfiguration>()
  }

  var wayfinderPath: String = "wayfinder"
  var defaultPort: Int = 5858
  var defaultRuntime: String = "lua54"
  var autoDetectRuntime: Boolean = true
  var sourceMapBehavior: String = "ask"

  var runtimePaths: Map<String, String> = mapOf(
    "lua51" to "lua5.1",
    "lua52" to "lua5.2",
    "lua53" to "lua5.3",
    "lua54" to "lua5.4",
    "luanext" to "luanext"
  )

  fun load() {
    try {
      // Load from environment variables (highest priority)
      wayfinderPath = System.getenv("WAYFINDER_PATH") ?: wayfinderPath
      defaultPort = System.getenv("WAYFINDER_PORT")?.toIntOrNull() ?: defaultPort

      // Could load from IDE settings here
      // Could load from wayfinder.yaml in project root

      LOG.info("Configuration loaded: wayfinderPath=$wayfinderPath, defaultPort=$defaultPort")
    } catch (e: Exception) {
      LOG.error("Error loading configuration", e)
    }
  }

  fun getRuntimePath(runtime: String): String {
    return runtimePaths[runtime] ?: runtime
  }

  fun verifyRuntime(runtime: String): Boolean {
    val path = getRuntimePath(runtime)
    return try {
      val file = File(path)
      file.exists() && file.canExecute()
    } catch (e: Exception) {
      false
    }
  }

  fun findWayfinderBinary(): String? {
    // Try to find wayfinder in PATH
    val pathEnv = System.getenv("PATH") ?: return null
    return pathEnv.split(File.pathSeparator).firstNotNullOfOrNull { dir ->
      val candidate = File(dir, "wayfinder")
      if (candidate.exists() && candidate.canExecute()) candidate.absolutePath else null
    }
  }

  fun substituteVariables(value: String): String {
    var result = value
    result = result.replace("\${workspaceFolder}", System.getProperty("user.dir"))
    result = result.replace("\${file}", "")
    result = result.replace("\${fileBasename}", "")
    return result
  }
}
