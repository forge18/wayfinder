//! Launch command implementation
//!
//! This module handles launching Lua scripts with debugging capabilities.

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use wayfinder_core::runtime::puc_lua::PUCLuaRuntime;
use wayfinder_core::session::DapServer;

/// Launch configuration
#[derive(Debug)]
pub struct LaunchConfig {
    /// Runtime to use (e.g., "lua5.1", "lua5.2", "lua5.3", "lua5.4")
    pub runtime: Option<String>,
    /// Current working directory
    pub cwd: Option<String>,
    /// Environment variables
    pub env: Option<HashMap<String, String>>,
    /// Script to launch
    pub script: String,
    /// Enable DAP debugging
    pub debug: bool,
}

/// Launch a Lua script with debugging capabilities
pub async fn launch_script(config: LaunchConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Determine the runtime executable
    let runtime_executable = config.runtime.clone().unwrap_or_else(|| "lua".to_string());

    println!("Launching {} with {}", config.script, runtime_executable);
    if config.debug {
        println!("Debug mode enabled - injecting debug helpers");
    }

    // Verify the script exists
    if !Path::new(&config.script).exists() {
        return Err(format!("Script not found: {}", config.script).into());
    }

    // Build the command
    let mut cmd = Command::new(&runtime_executable);

    // Set working directory if provided
    if let Some(cwd) = &config.cwd {
        println!("Working directory: {}", cwd);
        cmd.current_dir(cwd);
    }

    // Set environment variables if provided
    if let Some(env_vars) = &config.env {
        for (key, value) in env_vars {
            println!("Setting env: {}={}", key, value);
            cmd.env(key, value);
        }
    }

    // If debug mode is enabled, prepend the debug initialization script
    if config.debug {
        // Create a wrapper script that loads debug helpers then the user script
        let debug_init_path = get_debug_init_path()?;

        // Use -l flag to preload the debug module (Lua 5.2+)
        // For Lua 5.1, we'll use a different approach
        cmd.arg("-e");
        cmd.arg(format!("dofile('{}')", debug_init_path.display()));
        cmd.arg("-e");
        cmd.arg("wayfinder.start()");
    }

    // Add the script as an argument
    cmd.arg(&config.script);

    // Configure stdio to allow communication with the debugger
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::inherit()); // Show stderr directly to user

    // Spawn the process
    println!("Spawning Lua process...");
    let mut child = cmd.spawn()?;

    // Get the process ID
    if let Some(pid) = child.id() {
        println!("✓ Launched process with PID: {}", pid);
    } else {
        println!("✓ Launched process (PID unavailable)");
    }

    // If debug mode is enabled, set up DAP debugging
    if config.debug {
        println!("Starting DAP debugging session...");
        return launch_with_debugging(child, config.runtime).await;
    }

    // Normal execution without debugging
    // Forward stdout from the Lua process
    if let Some(stdout) = child.stdout.take() {
        let mut reader = BufReader::new(stdout);
        let mut line = String::new();

        println!("\n--- Script Output ---");
        while reader.read_line(&mut line).await? > 0 {
            print!("{}", line);
            std::io::stdout().flush()?;
            line.clear();
        }
    }

    // Wait for the process to complete
    let status = child.wait().await?;
    println!("\n--- Script Finished ---");
    println!("Exit status: {}", status);

    Ok(())
}

/// Launch with DAP debugging enabled
async fn launch_with_debugging(child: tokio::process::Child, runtime_version: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("DAP debugging enabled - starting debug session");
    eprintln!("Note: Full DAP debugging requires IDE connection");
    eprintln!("For now, the process will run with debug helpers injected");

    // Create DAP server
    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();

    // Set up the runtime with specified version
    let runtime = crate::create_puc_lua_runtime(runtime_version.as_deref());
    server.set_runtime(runtime);

    // Store the process handle
    server.set_process(child);

    // In a full implementation, this would:
    // 1. Start a DAP server (stdio or TCP) to accept debugger connections
    // 2. Wait for IDE to connect
    // 3. Handle DAP initialize/launch/attach requests
    // 4. Communicate with the injected Lua debug helpers
    // 5. Forward breakpoint hits, variable inspection, etc.

    // For now, we'll just wait for the process to complete
    eprintln!("Process running with debug capabilities");
    eprintln!("Connect a DAP client to begin debugging");

    // Run the basic event loop (placeholder)
    server.run_event_loop().await?;

    Ok(())
}

/// Get the path to the debug initialization script
fn get_debug_init_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Try to find debug_init.lua relative to the executable
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            // Check in the same directory as the executable
            let debug_init = exe_dir.join("debug_init.lua");
            if debug_init.exists() {
                return Ok(debug_init);
            }

            // Check in parent directory (for development builds)
            if let Some(parent_dir) = exe_dir.parent() {
                let debug_init = parent_dir.join("debug_init.lua");
                if debug_init.exists() {
                    return Ok(debug_init);
                }
            }
        }
    }

    // Try current working directory
    let cwd_debug_init = Path::new("debug_init.lua");
    if cwd_debug_init.exists() {
        return Ok(cwd_debug_init.to_path_buf());
    }

    // Try the CLI crate directory (for development)
    let cli_debug_init = Path::new("crates/wayfinder-cli/debug_init.lua");
    if cli_debug_init.exists() {
        return Ok(cli_debug_init.to_path_buf());
    }

    Err("Could not find debug_init.lua - make sure it's in the same directory as the wayfinder binary".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launch_config_creation() {
        let config = LaunchConfig {
            runtime: Some("lua5.4".to_string()),
            cwd: Some("/tmp".to_string()),
            env: None,
            script: "test.lua".to_string(),
            debug: false,
        };

        assert_eq!(config.runtime, Some("lua5.4".to_string()));
        assert_eq!(config.cwd, Some("/tmp".to_string()));
        assert_eq!(config.script, "test.lua".to_string());
        assert_eq!(config.debug, false);
    }
}