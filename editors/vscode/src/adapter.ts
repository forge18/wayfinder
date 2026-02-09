import * as vscode from 'vscode';
import { RuntimeManager } from './runtime-manager';
import { Configuration } from './configuration';
import { WayfinderDebugConfiguration } from './debug-provider';

export class WayfinderDebugAdapterDescriptorFactory
  implements vscode.DebugAdapterDescriptorFactory {
  private runtimeManager: RuntimeManager;

  constructor(private config: Configuration) {
    this.runtimeManager = RuntimeManager.getInstance(config);
  }

  async createDebugAdapterDescriptor(
    session: vscode.DebugSession,
    _executable?: vscode.DebugAdapterExecutable
  ): Promise<vscode.DebugAdapterDescriptor | null> {
    const config = session.configuration as WayfinderDebugConfiguration;

    if (config.request === 'attach') {
      // For attach requests, connect directly to the running process
      return new vscode.DebugAdapterServer(config.port || 5858, config.host || 'localhost');
    }

    // For launch requests, start the Wayfinder debug server
    try {
      const port = config.port || this.runtimeManager.getNextPort();
      const runtime = config.runtime || (await this.config.detectRuntime());
      const program = config.program || '';
      const cwd = config.cwd || process.cwd();
      const args = config.args || [];

      const wayfinderPath = this.config.getWayfinderPath();

      // Start the debug session
      await this.runtimeManager.startSession(
        session.id,
        wayfinderPath,
        port,
        runtime,
        program,
        cwd,
        args
      );

      // Return server descriptor to connect VSCode to the debug adapter
      return new vscode.DebugAdapterServer(port, 'localhost');
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      throw new Error(`Failed to start Wayfinder debug session: ${errorMessage}`);
    }
  }

  dispose(): void {
    // Clean up all active sessions
    this.runtimeManager.stopAllSessions();
  }
}
