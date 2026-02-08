pub use crate::bin::run_cli;

mod bin {
    use clap::{Parser, Subcommand};
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;
    use tokio::process::Command;
    use wayfinder_core::runtime::puc_lua::PUCLuaRuntime;
    use wayfinder_core::session::DapServer;

    #[derive(Parser)]
    #[command(name = "wayfinder")]
    #[command(author, version, about, long_about = None)]
    struct Args {
        #[command(subcommand)]
        command: Option<Commands>,
    }

    #[derive(Subcommand)]
    enum Commands {
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
            script: Option<String>,
        },
        #[command(about = "Attach to a running process")]
        Attach {
            #[arg(long, short = 'p')]
            port: Option<u16>,
            #[arg(long)]
            pid: Option<u32>,
        },
    }

    #[derive(Serialize, Deserialize)]
    pub struct Config {
        pub runtime: Option<String>,
        pub stop_on_entry: bool,
        pub cwd: Option<String>,
        pub env: Option<std::collections::HashMap<String, String>>,
    }

    #[derive(Serialize, Deserialize)]
    struct ConfigFile {
        runtime: Option<String>,
        stopOnEntry: Option<bool>,
        cwd: Option<String>,
        env: Option<std::collections::HashMap<String, String>>,
    }

    impl Config {
        pub fn load(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
            if !path.exists() {
                return Ok(Self::default());
            }

            let content = std::fs::read_to_string(path)?;
            let config_file: ConfigFile = serde_yaml::from_str(&content)?;

            Ok(Self {
                runtime: config_file.runtime,
                stop_on_entry: config_file.stopOnEntry.unwrap_or(false),
                cwd: config_file.cwd,
                env: config_file.env,
            })
        }
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                runtime: None,
                stop_on_entry: false,
                cwd: None,
                env: None,
            }
        }
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

    pub fn run_cli() {
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
                if let Some(p) = port {
                    println!("Port: {}", p);
                    // In a real implementation, we would start a TCP server here
                } else {
                    // STDIO mode
                    println!("Running in stdio mode");

                    // Create DAP server
                    let mut server: DapServer<PUCLuaRuntime> = DapServer::new();

                    // Set up the runtime
                    let runtime = PUCLuaRuntime::new();
                    server.set_runtime(runtime);

                    // Run the event loop
                    if let Err(e) =
                        tokio::runtime::Handle::current().block_on(server.run_event_loop())
                    {
                        eprintln!("Error running DAP server: {}", e);
                    }
                }
            }
            Some(Commands::Launch {
                runtime,
                cwd,
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
                if let Some(s) = script {
                    println!("Script: {}", s);

                    // Actually launch the script
                    let mut cmd = Command::new("lua");
                    if let Some(cwd) = &effective_cwd {
                        cmd.current_dir(cwd);
                    }
                    cmd.arg(&s);

                    match cmd.spawn() {
                        Ok(child) => {
                            if let Some(pid) = child.id() {
                                println!("Launched process with PID: {}", pid);
                            } else {
                                println!("Launched process (PID unavailable)");
                            }
                            // In a real implementation, we would attach the debugger here
                        }
                        Err(e) => {
                            eprintln!("Failed to launch script: {}", e);
                        }
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
            }
            None => {
                println!("No command specified. Use --help for usage.");
            }
        }
    }

    fn main() {
        run_cli();
    }
}
