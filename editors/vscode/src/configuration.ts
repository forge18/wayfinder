import * as vscode from 'vscode';
import * as fs from 'fs';
import * as path from 'path';
import { execSync } from 'child_process';

export type LuaRuntime = 'lua51' | 'lua52' | 'lua53' | 'lua54' | 'luanext';

export interface WayfinderConfig {
  wayfinderPath: string;
  runtimePaths: Record<LuaRuntime, string>;
  defaultPort: number;
  autoDetectRuntime: boolean;
  sourceMapBehavior: 'ask' | 'lenient' | 'strict';
}

export class Configuration {
  private static instance: Configuration;

  private config: WayfinderConfig;

  private constructor() {
    this.config = this.loadConfig();
  }

  static getInstance(): Configuration {
    if (!Configuration.instance) {
      Configuration.instance = new Configuration();
    }
    return Configuration.instance;
  }

  private loadConfig(): WayfinderConfig {
    const vscodeConfig = vscode.workspace.getConfiguration('wayfinder');

    return {
      wayfinderPath: this.resolveWayfinderPath(
        vscodeConfig.get<string>('wayfinder.path') || 'wayfinder'
      ),
      runtimePaths: {
        lua51: vscodeConfig.get<string>('runtime.lua51.path') || 'lua5.1',
        lua52: vscodeConfig.get<string>('runtime.lua52.path') || 'lua5.2',
        lua53: vscodeConfig.get<string>('runtime.lua53.path') || 'lua5.3',
        lua54: vscodeConfig.get<string>('runtime.lua54.path') || 'lua5.4',
        luanext: vscodeConfig.get<string>('runtime.luanext.path') || 'luanext',
      },
      defaultPort: vscodeConfig.get<number>('debug.port') || 5858,
      autoDetectRuntime: vscodeConfig.get<boolean>('debug.autoDetectRuntime') ?? true,
      sourceMapBehavior: vscodeConfig.get<'ask' | 'lenient' | 'strict'>('debug.sourceMapBehavior') || 'ask',
    };
  }

  private resolveWayfinderPath(basePath: string): string {
    // Try exact path first
    if (fs.existsSync(basePath)) {
      return basePath;
    }

    // Try common locations
    const candidates = [
      basePath,
      path.join(process.env.HOME || '', '.cargo', 'bin', 'wayfinder'),
      path.join(process.env.CARGO_HOME || '', 'bin', 'wayfinder'),
      'wayfinder',
    ];

    for (const candidate of candidates) {
      try {
        execSync(`which "${candidate}"`, { stdio: 'pipe' });
        return candidate;
      } catch {
        // Continue to next candidate
      }
    }

    // Default to wayfinder in PATH
    return 'wayfinder';
  }

  getConfig(): WayfinderConfig {
    return this.config;
  }

  getRuntimePath(runtime: LuaRuntime): string {
    return this.config.runtimePaths[runtime];
  }

  getWayfinderPath(): string {
    return this.config.wayfinderPath;
  }

  getDefaultPort(): number {
    return this.config.defaultPort;
  }

  /**
   * Detects the appropriate Lua runtime based on file type and workspace configuration
   */
  async detectRuntime(fileUri?: vscode.Uri): Promise<LuaRuntime> {
    if (!fileUri) {
      return this.config.autoDetectRuntime ? 'lua54' : 'lua54';
    }

    const fileName = path.basename(fileUri.fsPath);
    const fileExt = path.extname(fileName);

    // LuaNext files should use LuaNext runtime
    if (fileExt === '.luax') {
      return 'luanext';
    }

    // Check wayfinder.yaml for runtime configuration
    const workspaceFolder = vscode.workspace.getWorkspaceFolder(fileUri);
    if (workspaceFolder) {
      const wayfinderConfigPath = path.join(
        workspaceFolder.uri.fsPath,
        'wayfinder.yaml'
      );

      if (fs.existsSync(wayfinderConfigPath)) {
        try {
          const content = fs.readFileSync(wayfinderConfigPath, 'utf-8');
          // Simple YAML parsing for runtime field
          const runtimeMatch = content.match(/runtime:\s*(\w+)/);
          if (runtimeMatch) {
            const runtime = runtimeMatch[1].toLowerCase();
            if (this.isValidRuntime(runtime)) {
              return runtime as LuaRuntime;
            }
          }
        } catch (error) {
          console.error('Error reading wayfinder.yaml:', error);
        }
      }
    }

    // Default to lua54
    return 'lua54';
  }

  private isValidRuntime(runtime: string): boolean {
    return ['lua51', 'lua52', 'lua53', 'lua54', 'luanext'].includes(
      runtime
    );
  }

  /**
   * Verify that configured runtimes are available
   */
  async verifyRuntimes(): Promise<Record<LuaRuntime, boolean>> {
    const results: Record<LuaRuntime, boolean> = {
      lua51: this.checkRuntimeExists('lua51'),
      lua52: this.checkRuntimeExists('lua52'),
      lua53: this.checkRuntimeExists('lua53'),
      lua54: this.checkRuntimeExists('lua54'),
      luanext: this.checkRuntimeExists('luanext'),
    };

    return results;
  }

  private checkRuntimeExists(runtime: LuaRuntime): boolean {
    try {
      const runtimePath = this.getRuntimePath(runtime);
      execSync(`which "${runtimePath}" 2>/dev/null || test -x "${runtimePath}"`, {
        stdio: 'pipe',
      });
      return true;
    } catch {
      return false;
    }
  }

  reload(): void {
    this.config = this.loadConfig();
  }
}
