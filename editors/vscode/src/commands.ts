import * as vscode from 'vscode';
import { Configuration, LuaRuntime } from './configuration';
import { RuntimeManager } from './runtime-manager';

export class CommandHandler {
  constructor(
    private config: Configuration,
    private runtimeManager: RuntimeManager
  ) {}

  /**
   * Register all extension commands
   */
  registerCommands(context: vscode.ExtensionContext): void {
    context.subscriptions.push(
      vscode.commands.registerCommand(
        'wayfinder.debugFile',
        this.debugFile.bind(this)
      ),
      vscode.commands.registerCommand(
        'wayfinder.selectRuntime',
        this.selectRuntime.bind(this)
      ),
      vscode.commands.registerCommand(
        'wayfinder.attachProcess',
        this.attachProcess.bind(this)
      )
    );
  }

  /**
   * Debug the currently active file
   */
  private async debugFile(fileUri?: vscode.Uri): Promise<void> {
    let uri = fileUri;

    // If no file URI provided, use active editor
    if (!uri) {
      const editor = vscode.window.activeTextEditor;
      if (!editor) {
        vscode.window.showErrorMessage('No file is currently open');
        return;
      }

      const langId = editor.document.languageId;
      if (langId !== 'lua' && langId !== 'luanext') {
        vscode.window.showErrorMessage(
          'File must be a Lua (.lua) or LuaNext (.luax) file'
        );
        return;
      }

      uri = editor.document.uri;
    }

    // Detect runtime for the file
    const runtime = await this.config.detectRuntime(uri);

    // Start debugging
    vscode.debug.startDebugging(
      vscode.workspace.getWorkspaceFolder(uri),
      {
        type: 'wayfinder',
        name: `Debug ${uri.fsPath.split('/').pop()}`,
        request: 'launch',
        program: uri.fsPath,
        runtime: runtime,
        stopOnEntry: false,
      }
    );
  }

  /**
   * Select or change the Lua runtime
   */
  private async selectRuntime(): Promise<void> {
    const runtimes: LuaRuntime[] = ['lua51', 'lua52', 'lua53', 'lua54', 'luanext'];

    const quickPick = vscode.window.createQuickPick();
    quickPick.items = runtimes.map((runtime) => ({
      label: runtime,
      description: this.config.getRuntimePath(runtime),
    }));

    quickPick.placeholder = 'Select a Lua runtime';

    quickPick.onDidChangeSelection(async (items) => {
      if (items.length > 0) {
        const selected = items[0].label as LuaRuntime;
        quickPick.hide();

        // Verify the selected runtime is available
        try {
          const available = await this.verifyRuntime(selected);
          if (available) {
            vscode.window.showInformationMessage(
              `Selected runtime: ${selected}`
            );
          } else {
            vscode.window.showWarningMessage(
              `Runtime ${selected} not found at ${this.config.getRuntimePath(
                selected
              )}`
            );
          }
        } catch (error) {
          vscode.window.showErrorMessage(
            `Error verifying runtime: ${error}`
          );
        }
      }
    });

    quickPick.show();
  }

  /**
   * Attach to a running Lua process
   */
  private async attachProcess(): Promise<void> {
    const port = await vscode.window.showInputBox({
      prompt: 'Enter the DAP port of the running Lua process',
      value: '5858',
      validateInput: (value) => {
        const num = parseInt(value);
        if (isNaN(num) || num < 1 || num > 65535) {
          return 'Please enter a valid port number (1-65535)';
        }
        return '';
      },
    });

    if (!port) {
      return;
    }

    const host = await vscode.window.showInputBox({
      prompt: 'Enter the host address',
      value: 'localhost',
    });

    if (host === undefined) {
      return;
    }

    // Start attach debugging
    vscode.debug.startDebugging(
      vscode.workspace.workspaceFolders?.[0],
      {
        type: 'wayfinder',
        name: `Attach to ${host}:${port}`,
        request: 'attach',
        port: parseInt(port),
        host: host,
      }
    );
  }

  /**
   * Verify that a runtime is available
   */
  private async verifyRuntime(runtime: LuaRuntime): Promise<boolean> {
    const results = await this.config.verifyRuntimes();
    return results[runtime];
  }
}
