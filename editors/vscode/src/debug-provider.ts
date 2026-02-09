import * as vscode from 'vscode';
import { Configuration, LuaRuntime } from './configuration';

export interface WayfinderDebugConfiguration
  extends vscode.DebugConfiguration {
  type: 'wayfinder';
  request: 'launch' | 'attach';
  program?: string;
  cwd?: string;
  args?: string[];
  runtime?: LuaRuntime;
  port?: number;
  stopOnEntry?: boolean;
  console?: 'integratedTerminal' | 'externalTerminal' | 'internalConsole';
  host?: string;
}

export class WayfinderDebugConfigurationProvider
  implements vscode.DebugConfigurationProvider {
  constructor(private config: Configuration) {}

  async provideDebugConfigurations(
    folder: vscode.WorkspaceFolder | undefined,
    _token?: vscode.CancellationToken
  ): Promise<vscode.DebugConfiguration[]> {
    if (!folder) {
      return [];
    }

    // Detect default runtime based on workspace
    const runtime = await this.config.detectRuntime(folder.uri);

    return [
      {
        type: 'wayfinder',
        name: 'Launch Script',
        request: 'launch',
        program: '${workspaceFolder}/main.lua',
        cwd: '${workspaceFolder}',
        runtime: runtime,
        stopOnEntry: false,
      },
    ];
  }

  async resolveDebugConfiguration(
    folder: vscode.WorkspaceFolder | undefined,
    config: vscode.DebugConfiguration,
    _token?: vscode.CancellationToken
  ): Promise<vscode.DebugConfiguration | undefined> {
    // Handle missing required fields
    if (config.type !== 'wayfinder') {
      return config;
    }

    const wayfinder = config as WayfinderDebugConfiguration;

    // Handle launch request
    if (wayfinder.request === 'launch') {
      if (!wayfinder.program) {
        const editor = vscode.window.activeTextEditor;
        if (
          editor &&
          (editor.document.languageId === 'lua' ||
            editor.document.languageId === 'luanext')
        ) {
          wayfinder.program = editor.document.uri.fsPath;
        } else {
          const selected = await vscode.window.showInputBox({
            prompt: 'Enter the path to the Lua script to debug',
            value: '${workspaceFolder}/main.lua',
          });

          if (!selected) {
            return undefined;
          }

          wayfinder.program = selected;
        }
      }

      // Substitute variables
      wayfinder.program = this.substituteVariables(
        wayfinder.program,
        folder
      );

      if (!wayfinder.cwd) {
        wayfinder.cwd = folder?.uri.fsPath || process.cwd();
      } else {
        wayfinder.cwd = this.substituteVariables(wayfinder.cwd, folder);
      }

      // Detect runtime if not specified
      if (!wayfinder.runtime) {
        const fileUri = vscode.Uri.file(wayfinder.program);
        wayfinder.runtime =
          await this.config.detectRuntime(fileUri);
      }

      wayfinder.port = wayfinder.port || this.config.getDefaultPort();
      wayfinder.stopOnEntry = wayfinder.stopOnEntry ?? false;
      wayfinder.console = wayfinder.console || 'integratedTerminal';
    }

    // Handle attach request
    if (wayfinder.request === 'attach') {
      wayfinder.port = wayfinder.port || this.config.getDefaultPort();
      wayfinder.host = wayfinder.host || 'localhost';
    }

    return wayfinder;
  }

  private substituteVariables(
    str: string,
    folder?: vscode.WorkspaceFolder
  ): string {
    let result = str;

    if (folder) {
      result = result.replace(
        /\$\{workspaceFolder\}/g,
        folder.uri.fsPath
      );
      result = result.replace(
        /\$\{workspaceFolderBasename\}/g,
        folder.name
      );
    }

    result = result.replace(
      /\$\{userHome\}/g,
      process.env.HOME || ''
    );

    // Handle ${file} from active editor
    const editor = vscode.window.activeTextEditor;
    if (editor) {
      result = result.replace(
        /\$\{file\}/g,
        editor.document.uri.fsPath
      );
      result = result.replace(/\$\{fileDirname\}/g,
        editor.document.uri.fsPath.split('/').slice(0, -1).join('/')
      );
      result = result.replace(
        /\$\{fileBasename\}/g,
        editor.document.uri.fsPath.split('/').pop() || ''
      );
    }

    return result;
  }
}
