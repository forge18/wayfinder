// Module declarations
pub mod commands {
    pub mod launch;
    pub mod attach;
    pub mod dap;
    pub mod hot_reload;
}
pub mod config_mod;

// Re-exports for convenience
pub use config_mod::Config;

// Helper function to create PUCLuaRuntime in both static and dynamic modes
pub fn create_puc_lua_runtime() -> wayfinder_core::runtime::puc_lua::PUCLuaRuntime {
    #[cfg(feature = "static-lua")]
    {
        wayfinder_core::runtime::puc_lua::PUCLuaRuntime::new()
    }

    #[cfg(feature = "dynamic-lua")]
    {
        use wayfinder_core::runtime::lua_loader::LuaLibrary;
        use wayfinder_core::runtime::LuaVersion;

        // Load Lua library for the specified version
        let version = LuaVersion::V54; // TODO: Get from config
        let lib = LuaLibrary::load(version)
            .expect("Failed to load Lua library");

        wayfinder_core::runtime::puc_lua::PUCLuaRuntime::new_with_library(lib)
    }
}

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "wayfinder")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Run as DAP server")]
    Dap {
        #[arg(long, short = 'p')]
        port: Option<u16>,
    },
    #[command(about = "Launch and debug a script")]
    Launch {
        #[arg(long, short = 'r')]
        runtime: Option<String>,
        #[arg(long, short = 'c')]
        cwd: Option<String>,
        #[arg(long, short = 'd', help = "Enable DAP debugging")]
        debug: bool,
        script: Option<String>,
    },
    #[command(about = "Attach to a running process")]
    Attach {
        #[arg(long, short = 'p')]
        port: Option<u16>,
        #[arg(long)]
        pid: Option<u32>,
    },
    #[command(about = "Hot reload a module")]
    HotReload {
        #[arg(long, short = 'm', help = "Module name to reload")]
        module: String,
        #[arg(long, short = 'p', help = "Port to connect to DAP server")]
        port: Option<u16>,
        #[arg(long, default_value = "127.0.0.1", help = "Host to connect to")]
        host: String,
    },
}

fn find_config() -> Option<PathBuf> {
    if let Ok(cwd) = std::env::current_dir() {
        let path = cwd.join("wayfinder.yaml");
        if path.exists() {
            return Some(path);
        }
    }
    if let Some(home) = home::home_dir() {
        let path = home.join(".wayfinder.yaml");
        if path.exists() {
            return Some(path);
        }
    }
    None
}

pub async fn run_cli() {
    let args = Args::parse();

    let config = if let Some(config_path) = find_config() {
        match Config::load(&config_path) {
            Ok(cfg) => {
                println!("Loaded config: {}", config_path.display());
                Some(cfg)
            }
            Err(e) => {
                println!("Error loading config: {}", e);
                None
            }
        }
    } else {
        None
    };

    match args.command {
        Some(Commands::Dap { port }) => {
            println!("DAP server mode");

            let dap_config = commands::dap::DapConfig {
                port,
                multi_client: false, // Could be made configurable
            };

            if let Err(e) = commands::dap::run_dap_server(dap_config).await {
                eprintln!("Error running DAP server: {}", e);
            }
        }
        Some(Commands::Launch {
            runtime,
            cwd,
            debug,
            script,
        }) => {
            println!("Launch mode");

            let effective_runtime = runtime.or(config.as_ref().and_then(|c| c.runtime.clone()));
            let effective_cwd = cwd.or(config.as_ref().and_then(|c| c.cwd.clone()));

            if let Some(r) = &effective_runtime {
                println!("Runtime: {}", r);
            }
            if let Some(c) = &effective_cwd {
                println!("CWD: {}", c);
            }
            if debug {
                println!("Debug mode: enabled");
            }
            if let Some(s) = script {
                println!("Script: {}", s);

                let launch_config = commands::launch::LaunchConfig {
                    runtime: effective_runtime,
                    cwd: effective_cwd,
                    env: config.as_ref().and_then(|c| c.env.clone()),
                    script: s,
                    debug,
                };

                if let Err(e) = commands::launch::launch_script(launch_config).await {
                    eprintln!("Failed to launch script: {}", e);
                }
            }
        }
        Some(Commands::Attach { port, pid }) => {
            println!("Attach mode");
            if let Some(p) = port {
                println!("Port: {}", p);
            }
            if let Some(p) = pid {
                println!("PID: {}", p);
            }

            let attach_config = commands::attach::AttachConfig {
                port,
                pid,
            };

            if let Err(e) = commands::attach::attach_to_process(attach_config).await {
                eprintln!("Error attaching to process: {}", e);
            }
        }
        Some(Commands::HotReload { module, port, host }) => {
            println!("Hot reload mode");
            println!("Module: {}", module);

            let hot_reload_config = commands::hot_reload::HotReloadConfig {
                module,
                host,
                port,
            };

            if let Err(e) = commands::hot_reload::send_hot_reload(hot_reload_config).await {
                eprintln!("Error sending hot reload request: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            println!("No command specified. Use --help for usage.");
        }
    }
}