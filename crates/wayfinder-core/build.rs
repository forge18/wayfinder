use std::env;

fn main() {
    // Try to find Lua using pkg-config first
    if pkg_config::Config::new()
        .atleast_version("5.4")
        .probe("lua5.4")
        .is_ok()
    {
        return;
    }

    // If pkg-config fails, try manual configuration for common platforms
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "macos" => {
            // Check if Homebrew Lua is installed
            let homebrew_lib = "/opt/homebrew/lib";
            let homebrew_include = "/opt/homebrew/include/lua";

            if std::path::Path::new(homebrew_lib).exists()
                && std::path::Path::new(homebrew_include).exists()
            {
                println!("cargo:rustc-link-search=native={}", homebrew_lib);
                println!("cargo:rustc-link-lib=static=lua");
                println!("cargo:rustc-link-lib=framework=CoreFoundation");
                return;
            }
        }
        "linux" => {
            // Common Linux paths
            let lib_paths = ["/usr/lib", "/usr/local/lib"];
            let include_paths = [
                "/usr/include/lua5.4",
                "/usr/local/include/lua5.4",
                "/usr/include",
                "/usr/local/include",
            ];

            for lib_path in &lib_paths {
                if std::path::Path::new(lib_path).exists() {
                    println!("cargo:rustc-link-search=native={}", lib_path);
                    break;
                }
            }

            // Find the first existing include path
            for include_path in &include_paths {
                if std::path::Path::new(include_path).exists() {
                    // We don't need to tell rustc about include paths for cc
                    break;
                }
            }

            println!("cargo:rustc-link-lib=dylib=lua5.4");
            println!("cargo:rustc-link-lib=dylib=m");
            return;
        }
        _ => {}
    }

    // If we get here, we couldn't find Lua automatically
    // The user will need to set the paths manually
    println!("cargo:warning=Could not automatically find Lua 5.4 library");
    println!("cargo:warning=You may need to set PKG_CONFIG_PATH or LUA_LIB_DIR/LUA_INCLUDE_DIR environment variables");
}
