package com.wayfinder.lua.debugger.config

import com.intellij.openapi.diagnostic.logger
import java.io.File

/**
 * Manages available Lua runtimes and their verification.
 */
class RuntimeManager {
  companion object {
    private val LOG = logger<RuntimeManager>()

    private val RUNTIMES = mapOf(
      "lua51" to "lua5.1",
      "lua52" to "lua5.2",
      "lua53" to "lua5.3",
      "lua54" to "lua5.4",
      "luanext" to "luanext"
    )
  }

  private var availableRuntimes: Map<String, String> = mutableMapOf()

  fun refresh() {
    val available = mutableMapOf<String, String>()

    for ((key, binary) in RUNTIMES) {
      if (isAvailable(key)) {
        available[key] = binary
      }
    }

    availableRuntimes = available
    LOG.info("Runtime scan complete: ${available.keys}")
  }

  fun isAvailable(runtime: String): Boolean {
    val binary = RUNTIMES[runtime] ?: return false
    return try {
      val file = File(binary)
      file.exists() && file.canExecute()
    } catch (e: Exception) {
      false
    }
  }

  fun getAvailableRuntimes(): Map<String, String> = availableRuntimes

  fun getRuntime(name: String): String? = RUNTIMES[name]

  fun verify(runtime: String): Boolean {
    if (!RUNTIMES.containsKey(runtime)) {
      LOG.warn("Unknown runtime: $runtime")
      return false
    }

    if (!isAvailable(runtime)) {
      LOG.warn("Runtime not available: $runtime")
      return false
    }

    return true
  }
}
