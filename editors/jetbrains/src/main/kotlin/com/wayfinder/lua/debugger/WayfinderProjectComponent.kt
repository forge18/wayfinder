package com.wayfinder.lua.debugger

import com.intellij.openapi.components.Service
import com.intellij.openapi.diagnostic.logger
import com.intellij.openapi.project.Project
import com.wayfinder.lua.debugger.config.ProjectConfiguration

/**
 * Project-level component for Wayfinder debugger.
 * Manages per-project settings and state.
 */
@Service
class WayfinderProjectComponent(private val project: Project) {
  companion object {
    private val LOG = logger<WayfinderProjectComponent>()

    fun getInstance(project: Project): WayfinderProjectComponent = project.service()
  }

  private val projectConfig = ProjectConfiguration(project)
  private var activeDebugSessions = mutableMapOf<String, DebugSession>()
  private var nextPort = 5858

  data class DebugSession(
    val sessionId: String,
    val filePath: String,
    val runtime: String,
    val port: Int,
    val timestamp: Long = System.currentTimeMillis()
  )

  fun initialize() {
    LOG.info("Initializing Wayfinder project component for: ${project.name}")

    // Load project-specific configuration (wayfinder.yaml, .wayfinder.toml, etc.)
    projectConfig.load()
  }

  fun getProjectConfiguration(): ProjectConfiguration = projectConfig

  fun getNextPort(): Int {
    return nextPort++
  }

  fun registerSession(filePath: String, runtime: String, port: Int): String {
    val sessionId = generateSessionId()
    activeDebugSessions[sessionId] = DebugSession(sessionId, filePath, runtime, port)
    LOG.info("Registered debug session: $sessionId (port: $port)")
    return sessionId
  }

  fun unregisterSession(sessionId: String) {
    activeDebugSessions.remove(sessionId)
    LOG.info("Unregistered debug session: $sessionId")
  }

  fun getSession(sessionId: String): DebugSession? {
    return activeDebugSessions[sessionId]
  }

  fun getActiveSessions(): List<DebugSession> {
    return activeDebugSessions.values.toList()
  }

  fun getRuntime(filePath: String): String {
    return WayfinderDebuggerComponent.getInstance().detectRuntime(filePath)
  }

  private fun generateSessionId(): String {
    return "lua-debug-${System.currentTimeMillis()}-${(0..999).random()}"
  }
}
