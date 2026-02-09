import * as vscode from 'vscode';
import { Configuration } from './configuration';
import { WayfinderDebugConfigurationProvider } from './debug-provider';
import { WayfinderDebugAdapterDescriptorFactory } from './adapter';
import { RuntimeManager } from './runtime-manager';
import { CommandHandler } from './commands';

let adapterFactory: WayfinderDebugAdapterDescriptorFactory;
let runtimeManager: RuntimeManager;

export function activate(context: vscode.ExtensionContext) {
  console.log('Wayfinder Debugger extension activated');

  // Initialize configuration
  const config = Configuration.getInstance();

  // Initialize runtime manager
  runtimeManager = RuntimeManager.getInstance(config);

  // Register debug configuration provider
  const debugProvider = new WayfinderDebugConfigurationProvider(config);
  context.subscriptions.push(
    vscode.debug.registerDebugConfigurationProvider('wayfinder', debugProvider)
  );

  // Register debug adapter descriptor factory
  adapterFactory = new WayfinderDebugAdapterDescriptorFactory(config);
  context.subscriptions.push(
    vscode.debug.registerDebugAdapterDescriptorFactory(
      'wayfinder',
      adapterFactory
    )
  );

  // Register commands
  const commandHandler = new CommandHandler(config, runtimeManager);
  commandHandler.registerCommands(context);

  // Handle debug session termination
  context.subscriptions.push(
    vscode.debug.onDidTerminateDebugSession((session) => {
      if (session.type === 'wayfinder') {
        runtimeManager.stopSession(session.id);
      }
    })
  );

  // Show welcome message
  vscode.window.showInformationMessage(
    'Wayfinder Lua Debugger is ready!'
  );
}

export function deactivate() {
  console.log('Wayfinder Debugger extension deactivated');

  // Stop all active debug sessions
  if (runtimeManager) {
    runtimeManager.stopAllSessions();
  }

  // Dispose adapter factory
  if (adapterFactory) {
    adapterFactory.dispose();
  }
}
