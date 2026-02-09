import { ChildProcess, spawn } from 'child_process';
import { Configuration, LuaRuntime } from './configuration';

export interface DebugSession {
  id: string;
  port: number;
  process: ChildProcess;
  runtime: LuaRuntime;
  startTime: Date;
}

export class RuntimeManager {
  private static instance: RuntimeManager;
  private activeSessions: Map<string, DebugSession> = new Map();
  private nextPort: number;

  private constructor(private config: Configuration) {
    this.nextPort = config.getDefaultPort();
  }

  static getInstance(config: Configuration): RuntimeManager {
    if (!RuntimeManager.instance) {
      RuntimeManager.instance = new RuntimeManager(config);
    }
    return RuntimeManager.instance;
  }

  /**
   * Get the next available DAP port
   */
  getNextPort(): number {
    const port = this.nextPort;
    this.nextPort++;
    return port;
  }

  /**
   * Start a Wayfinder debug session
   */
  startSession(
    sessionId: string,
    wayfinderPath: string,
    port: number,
    runtime: LuaRuntime,
    programPath: string,
    cwd: string,
    args: string[] = []
  ): Promise<DebugSession> {
    return new Promise((resolve, reject) => {
      try {
        // Build Wayfinder command arguments
        const wayfindArgs = [
          'dap-server',
          '--port', port.toString(),
          '--runtime', runtime,
          '--script', programPath,
          '--cwd', cwd,
        ];

        if (args.length > 0) {
          wayfindArgs.push('--args', ...args);
        }

        // Spawn Wayfinder process
        const process = spawn(wayfinderPath, wayfindArgs, {
          cwd,
          stdio: ['pipe', 'pipe', 'pipe'],
        });

        const session: DebugSession = {
          id: sessionId,
          port,
          process,
          runtime,
          startTime: new Date(),
        };

        process.on('error', (error) => {
          this.activeSessions.delete(sessionId);
          reject(new Error(`Failed to start Wayfinder: ${error.message}`));
        });

        process.on('exit', () => {
          this.activeSessions.delete(sessionId);
        });

        this.activeSessions.set(sessionId, session);
        resolve(session);
      } catch (error) {
        reject(error);
      }
    });
  }

  /**
   * Stop a debug session
   */
  stopSession(sessionId: string): void {
    const session = this.activeSessions.get(sessionId);
    if (session) {
      try {
        session.process.kill();
      } catch (error) {
        console.error(`Error stopping session ${sessionId}:`, error);
      }
      this.activeSessions.delete(sessionId);
    }
  }

  /**
   * Get active session info
   */
  getSession(sessionId: string): DebugSession | undefined {
    return this.activeSessions.get(sessionId);
  }

  /**
   * Get all active sessions
   */
  getAllSessions(): DebugSession[] {
    return Array.from(this.activeSessions.values());
  }

  /**
   * Stop all active sessions
   */
  stopAllSessions(): void {
    for (const [sessionId] of this.activeSessions) {
      this.stopSession(sessionId);
    }
  }
}
