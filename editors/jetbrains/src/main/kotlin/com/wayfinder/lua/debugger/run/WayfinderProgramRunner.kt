package com.wayfinder.lua.debugger.run

import com.intellij.execution.ExecutionException
import com.intellij.execution.ExecutionManager
import com.intellij.execution.Executor
import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.execution.process.ProcessHandler
import com.intellij.execution.runners.AsyncProgramRunner
import com.intellij.execution.runners.ExecutionEnvironment
import com.intellij.execution.ui.RunContentDescriptor
import com.intellij.openapi.fileEditor.FileDocumentManager
import com.intellij.util.concurrency.AppExecutorUtil
import com.wayfinder.lua.debugger.WayfinderDebuggerComponent
import com.wayfinder.lua.debugger.WayfinderProjectComponent
import org.jetbrains.concurrency.AsyncPromise
import org.jetbrains.concurrency.Promise

/**
 * Program runner for executing Lua debug sessions.
 */
class WayfinderProgramRunner : AsyncProgramRunner<WayfinderRunConfiguration>() {
  override fun getRunnerId(): String = "WayfinderLuaRunner"

  override fun canRun(executorId: String, profile: WayfinderRunConfiguration): Boolean {
    return executorId == "Debug"
  }

  override fun executeAsync(
    environment: ExecutionEnvironment,
    callback: (RunContentDescriptor?) -> Unit
  ): Promise<RunContentDescriptor?> {
    val promise = AsyncPromise<RunContentDescriptor?>()
    val project = environment.project
    val profile = environment.runProfile as? WayfinderRunConfiguration ?: run {
      promise.setError(ExecutionException("Invalid run configuration"))
      return promise
    }

    FileDocumentManager.getInstance().saveAllDocuments()

    AppExecutorUtil.getAppExecutorService().execute {
      try {
        val debugger = WayfinderDebuggerComponent.getInstance()
        debugger.initialize()

        // Verify configuration
        profile.checkConfiguration()

        val projectComponent = WayfinderProjectComponent.getInstance(project)
        val port = projectComponent.getNextPort()

        // Create command line
        val cmdLine = GeneralCommandLine()
          .withExePath(debugger.getConfiguration().wayfinderPath)
          .withParameters("dap-server")
          .withParameters("--port", port.toString())
          .withParameters("--runtime", profile.runtime)
          .withParameters("--script", profile.scriptPath)

        if (profile.arguments.isNotEmpty()) {
          cmdLine.withParameters(*profile.arguments.split(" ").toTypedArray())
        }

        if (profile.workingDirectory.isNotEmpty()) {
          cmdLine.withWorkingDirectory(profile.workingDirectory)
        }

        // Register session
        val sessionId = projectComponent.registerSession(profile.scriptPath, profile.runtime, port)

        promise.setResult(null)
      } catch (e: ExecutionException) {
        promise.setError(e)
      } catch (e: Exception) {
        promise.setError(ExecutionException("Failed to start debug session", e))
      }
    }

    return promise
  }
}
